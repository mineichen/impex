use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Data, DeriveInput, Fields, Ident, Index, Token, parse_macro_input, punctuated::Punctuated,
};

struct GenerateContext<'a> {
    impex_name: &'a Ident,
    original_name: &'a Ident,
    vis: &'a syn::Visibility,
    derives: proc_macro2::TokenStream,
    has_partial_eq: bool,
    has_eq: bool,
}

#[proc_macro_derive(Impex, attributes(impex))]
pub fn derive_impex(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let (derives, has_partial_eq, has_eq) = parse_impex_attributes(&input.attrs);
    let ctx = GenerateContext {
        impex_name: &Ident::new(&format!("{}Impex", name), name.span()),
        original_name: name,
        vis: &input.vis,
        derives,
        has_partial_eq,
        has_eq,
    };

    let expanded = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => generate_named_struct(ctx, fields),
            Fields::Unnamed(fields) => generate_tuple_struct(ctx, fields),
            Fields::Unit => panic!("Unit structs are not supported"),
        },
        Data::Enum(data_enum) => generate_enum(ctx, data_enum),
        Data::Union(_) => panic!("Unions are not supported"),
    };

    TokenStream::from(expanded)
}

fn parse_impex_attributes(attrs: &[syn::Attribute]) -> (proc_macro2::TokenStream, bool, bool) {
    let has_partial_eq = &mut false;
    let has_eq = &mut false;

    let extra_derives = attrs
        .iter()
        .filter(|attr| attr.path().is_ident("impex"))
        .filter_map(|attr| {
            let meta = attr.parse_args::<syn::Meta>().ok()?;
            let tokens = match meta {
                syn::Meta::List(list) => list.path.is_ident("derive").then_some(list.tokens)?,
                _ => return None,
            };

            syn::parse::Parser::parse2(
                |input: syn::parse::ParseStream| {
                    Punctuated::<syn::Path, Token![,]>::parse_terminated(input)
                },
                tokens,
            )
            .ok()
        })
        .flat_map(|tokens| {
            tokens
                .into_iter()
                .filter_map(|path| path.get_ident().cloned())
        })
        .filter(|ident| {
            if ident == "PartialEq" {
                *has_partial_eq = true;
                false
            } else if ident == "Eq" {
                *has_eq = true;
                false
            } else if ident == "Clone" {
                // Clone is always derived, so filter it out
                false
            } else {
                true
            }
        });

    let derives = quote! { #(#extra_derives),* };
    (derives, *has_partial_eq, *has_eq)
}

fn generate_named_struct(
    ctx: GenerateContext,
    fields: &syn::FieldsNamed,
) -> proc_macro2::TokenStream {
    let GenerateContext {
        impex_name,
        original_name,
        vis,
        derives,
        has_partial_eq,
        has_eq,
    } = ctx;

    let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
    let field_types: Vec<_> = fields.named.iter().map(|f| &f.ty).collect();

    // Generate the Impex struct definition (without serde attributes)
    let impex_fields = fields.named.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        let field_vis = &f.vis;
        quote! {
            #field_vis #name: <#ty as ::impex::IntoImpex<TW>>::Impex
        }
    });

    // Generate IntoImpex implementation
    let into_impex_fields = field_names.iter().map(|name| {
        quote! {
            #name: ::impex::IntoImpex::<TW>::into_impex(self.#name, is_explicit)
        }
    });

    // Generate is_explicit check (all fields OR'd together)
    let mut field_iter = field_names.iter();
    let is_explicit_body = if let Some(first) = field_iter.next() {
        let first_check = quote! { ::impex::Impex::<TW>::is_explicit(&self.#first) };
        let rest_checks = field_iter.map(|name| {
            quote! { || ::impex::Impex::<TW>::is_explicit(&self.#name) }
        });
        quote! {
            #first_check #(#rest_checks)*
        }
    } else {
        quote! { false }
    };

    // Generate into_value implementation
    let into_value_fields = field_names.iter().map(|name| {
        quote! {
            #name: ::impex::Impex::<TW>::into_value(self.#name)
        }
    });

    // Generate set_impex implementation (all fields)
    let set_impex_fields = field_names.iter().map(|name| {
        quote! {
            ::impex::Impex::<TW>::set_impex(&mut self.#name, v.#name, is_explicit);
        }
    });

    // Generate default implementation
    let default_fields = field_names.iter().map(|name| {
        quote! {
            #name: ::impex::IntoImpex::<TW>::into_implicit(x.#name)
        }
    });

    // Generate Visitor implementation (only if visitor feature is enabled)
    let visitor_impl = if cfg!(feature = "visitor") {
        let visitor_where_clauses = field_types.iter().map(|ty| {
            quote! {
                <#ty as ::impex::IntoImpex<TW>>::Impex: ::impex::Visitor<T>
            }
        });

        let visitor_visit_fields = field_names.iter().map(|name| {
            quote! {
                ::impex::Visitor::<T>::visit(&mut self.#name, ctx);
            }
        });

        quote! {
            impl<T, TW: ::impex::WrapperSettings> ::impex::Visitor<T> for #impex_name<TW>
            where
                #(#visitor_where_clauses),*
            {
                fn visit(&mut self, ctx: &mut T) {
                    #(#visitor_visit_fields)*
                }
            }
        }
    } else {
        quote! {}
    };

    // Generate PartialEq and Eq implementations with proper bounds
    let mut eq_impl = quote! {};
    let mut partial_eq_impl = quote! {};
    if has_partial_eq || has_eq {
        let partial_eq_where_clauses = field_types.iter().map(|ty| {
            quote! {
                <#ty as ::impex::IntoImpex<TW>>::Impex: PartialEq
            }
        });

        let eq_where_clauses = field_types.iter().map(|ty| {
            quote! {
                <#ty as ::impex::IntoImpex<TW>>::Impex: Eq
            }
        });

        let field_comparisons = field_names.iter().map(|name| {
            quote! { self.#name == other.#name }
        });

        if has_partial_eq {
            partial_eq_impl = quote! {
                impl<TW: ::impex::WrapperSettings> PartialEq for #impex_name<TW>
                where
                    #(#partial_eq_where_clauses),*
                {
                    fn eq(&self, other: &Self) -> bool {
                        #(#field_comparisons)&&*
                    }
                }
            }
        }

        if has_eq {
            eq_impl = quote! {
                impl<TW: ::impex::WrapperSettings> Eq for #impex_name<TW>
                where
                    #(#eq_where_clauses),*
                {}
            }
        }
    }

    // Generate serialization struct with serde attributes
    let serde_struct_name = Ident::new(&format!("{}Serde", impex_name), impex_name.span());
    let serde_fields = fields.named.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {
            #[serde(skip_serializing_if = "::impex::Impex::<TW>::is_implicit")]
            #name: <#ty as ::impex::IntoImpex<TW>>::Impex
        }
    });

    let serde_from_fields: Vec<_> = field_names
        .iter()
        .map(|name| {
            quote! { #name: value.#name }
        })
        .collect();

    let serde_where_clauses: Vec<_> = field_types.iter().map(|ty| {
        quote! {
            <#ty as ::impex::IntoImpex<TW>>::Impex: ::serde::Serialize + ::serde::de::DeserializeOwned
        }
    }).collect();

    quote! {
        #[derive(Clone, #derives)]
        #vis struct #impex_name<TW: ::impex::WrapperSettings = ::impex::DefaultWrapperSettings> {
            #(#impex_fields),*
        }

        #[derive(::serde::Serialize, ::serde::Deserialize)]
        #[serde(default, bound = "")]
        struct #serde_struct_name<TW: ::impex::WrapperSettings> {
            #(#serde_fields),*
        }

        impl<TW: ::impex::WrapperSettings> Default for #serde_struct_name<TW>
        where
            #(#serde_where_clauses),*
        {
            fn default() -> Self {
                let default_value = #original_name::default();
                let impex: #impex_name<TW> = ::impex::IntoImpex::into_impex(default_value, false);
                impex.into()
            }
        }

        impl<TW: ::impex::WrapperSettings> From<#serde_struct_name<TW>> for #impex_name<TW> {
            fn from(value: #serde_struct_name<TW>) -> Self {
                Self {
                    #(#serde_from_fields),*
                }
            }
        }

        impl<TW: ::impex::WrapperSettings> From<#impex_name<TW>> for #serde_struct_name<TW> {
            fn from(value: #impex_name<TW>) -> Self {
                Self {
                    #(#field_names: value.#field_names),*
                }
            }
        }

        impl<TW: ::impex::WrapperSettings> ::serde::Serialize for #impex_name<TW>
        where
            #(#serde_where_clauses,)*
            Self: Clone,
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                let serde_struct: #serde_struct_name<TW> = Clone::clone(self).into();
                serde_struct.serialize(serializer)
            }
        }

        impl<'de, TW: ::impex::WrapperSettings> ::serde::Deserialize<'de> for #impex_name<TW>
        where
            #(#serde_where_clauses),*
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                let serde_struct = #serde_struct_name::<TW>::deserialize(deserializer)?;
                Ok(serde_struct.into())
            }
        }

        impl<TW: ::impex::WrapperSettings> ::impex::IntoImpex<TW> for #original_name {
            type Impex = #impex_name<TW>;

            fn into_impex(self, is_explicit: bool) -> Self::Impex {
                #impex_name {
                    #(#into_impex_fields),*
                }
            }
        }

        impl<TW: ::impex::WrapperSettings> ::impex::Impex<TW> for #impex_name<TW> {
            type Value = #original_name;

            fn is_explicit(&self) -> bool {
                #is_explicit_body
            }

            fn into_value(self) -> Self::Value {
                #original_name {
                    #(#into_value_fields),*
                }
            }

            fn set_impex(&mut self, v: Self::Value, is_explicit: bool) {
                #(#set_impex_fields)*
            }
        }

        impl<TW: ::impex::WrapperSettings> Default for #impex_name<TW> {
            fn default() -> Self {
                let x = #original_name::default();
                Self {
                    #(#default_fields),*
                }
            }
        }

        #visitor_impl
        #eq_impl
        #partial_eq_impl
    }
}

fn generate_tuple_struct(
    ctx: GenerateContext,
    fields: &syn::FieldsUnnamed,
) -> proc_macro2::TokenStream {
    let GenerateContext {
        impex_name,
        original_name,
        vis,
        derives,
        has_partial_eq,
        has_eq,
    } = ctx;

    let field_types: Vec<_> = fields.unnamed.iter().map(|f| &f.ty).collect();
    let field_vis: Vec<_> = fields.unnamed.iter().map(|f| &f.vis).collect();
    let field_indices: Vec<Index> = (0..fields.unnamed.len()).map(Index::from).collect();

    // Generate the Impex struct definition
    let impex_fields: Vec<_> = field_types
        .iter()
        .zip(field_vis.iter())
        .map(|(ty, vis)| {
            quote! {
                #vis <#ty as ::impex::IntoImpex<TW>>::Impex
            }
        })
        .collect();

    // Generate IntoImpex implementation
    let into_impex_fields = field_indices.iter().map(|idx| {
        quote! {
            ::impex::IntoImpex::<TW>::into_impex(self.#idx, is_explicit)
        }
    });

    // Generate is_explicit check (all fields OR'd together)
    let mut idx_iter = field_indices.iter();
    let is_explicit_body = if let Some(first) = idx_iter.next() {
        let first_check = quote! { ::impex::Impex::<TW>::is_explicit(&self.#first) };
        let rest_checks = idx_iter.map(|idx| {
            quote! { || ::impex::Impex::<TW>::is_explicit(&self.#idx) }
        });
        quote! {
            #first_check #(#rest_checks)*
        }
    } else {
        quote! { false }
    };

    // Generate into_value implementation
    let into_value_fields = field_indices.iter().map(|idx| {
        quote! {
            ::impex::Impex::<TW>::into_value(self.#idx)
        }
    });

    // Generate set_impex implementation
    let set_impex_fields = field_indices.iter().map(|idx| {
        quote! {
            ::impex::Impex::<TW>::set_impex(&mut self.#idx, v.#idx, is_explicit);
        }
    });

    // Generate default implementation
    let default_fields = field_indices.iter().map(|idx| {
        quote! {
            ::impex::IntoImpex::<TW>::into_implicit(x.#idx)
        }
    });

    // Generate Visitor implementation (only if visitor feature is enabled)
    let visitor_impl = if cfg!(feature = "visitor") {
        let visitor_where_clauses = field_types.iter().map(|ty| {
            quote! {
                <#ty as ::impex::IntoImpex<TW>>::Impex: ::impex::Visitor<T>
            }
        });

        let visitor_visit_fields = field_indices.iter().map(|idx| {
            quote! {
                ::impex::Visitor::<T>::visit(&mut self.#idx, ctx);
            }
        });

        quote! {
            impl<T, TW: ::impex::WrapperSettings> ::impex::Visitor<T> for #impex_name<TW>
            where
                #(#visitor_where_clauses),*
            {
                fn visit(&mut self, ctx: &mut T) {
                    #(#visitor_visit_fields)*
                }
            }
        }
    } else {
        quote! {}
    };

    let mut eq_impl = quote! {};
    let mut partial_eq_impl = quote! {};
    if has_partial_eq || has_eq {
        let partial_eq_where_clauses = field_types.iter().map(|ty| {
            quote! {
                <#ty as ::impex::IntoImpex<TW>>::Impex: PartialEq
            }
        });

        let eq_where_clauses = field_types.iter().map(|ty| {
            quote! {
                <#ty as ::impex::IntoImpex<TW>>::Impex: Eq
            }
        });

        let field_comparisons = field_indices.iter().map(|idx| {
            quote! { self.#idx == other.#idx }
        });

        if has_partial_eq {
            partial_eq_impl = quote! {
                impl<TW: ::impex::WrapperSettings> PartialEq for #impex_name<TW>
                where
                    #(#partial_eq_where_clauses),*
                {
                    fn eq(&self, other: &Self) -> bool {
                        #(#field_comparisons)&&*
                    }
                }
            }
        }

        if has_eq {
            eq_impl = quote! {
                impl<TW: ::impex::WrapperSettings> Eq for #impex_name<TW>
                where
                    #(#eq_where_clauses),*
                {}
            }
        }
    }

    // Generate serialization struct
    let serde_struct_name = Ident::new(&format!("{}Serde", impex_name), impex_name.span());
    let serde_from_fields: Vec<_> = field_indices
        .iter()
        .map(|idx| {
            quote! { value.#idx }
        })
        .collect();
    let serde_where_clauses: Vec<_> = field_types.iter().map(|ty| {
        quote! {
            <#ty as ::impex::IntoImpex<TW>>::Impex: ::serde::Serialize + ::serde::de::DeserializeOwned
        }
    }).collect();

    quote! {
        #[derive(Clone, #derives)]
        #vis struct #impex_name<TW: ::impex::WrapperSettings = ::impex::DefaultWrapperSettings>(
            #(#impex_fields),*
        );

        #[derive(::serde::Serialize, ::serde::Deserialize)]
        #[serde(bound = "")]
        struct #serde_struct_name<TW: ::impex::WrapperSettings>(
            #(#impex_fields),*
        );

        impl<TW: ::impex::WrapperSettings> Default for #serde_struct_name<TW>
        where
            #(#serde_where_clauses),*
        {
            fn default() -> Self {
                let default_value = #original_name::default();
                let impex: #impex_name<TW> = ::impex::IntoImpex::into_impex(default_value, false);
                Self(#(impex.#field_indices),*)
            }
        }

        impl<TW: ::impex::WrapperSettings> From<#serde_struct_name<TW>> for #impex_name<TW> {
            fn from(value: #serde_struct_name<TW>) -> Self {
                Self(#(#serde_from_fields),*)
            }
        }

        impl<TW: ::impex::WrapperSettings> From<#impex_name<TW>> for #serde_struct_name<TW> {
            fn from(value: #impex_name<TW>) -> Self {
                Self(#(value.#field_indices),*)
            }
        }

        impl<TW: ::impex::WrapperSettings> ::serde::Serialize for #impex_name<TW>
        where
            #(#serde_where_clauses,)*
            Self: Clone,
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                let serde_struct: #serde_struct_name<TW> = Clone::clone(self).into();
                serde_struct.serialize(serializer)
            }
        }

        impl<'de, TW: ::impex::WrapperSettings> ::serde::Deserialize<'de> for #impex_name<TW>
        where
            #(#serde_where_clauses),*
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                let serde_struct = #serde_struct_name::<TW>::deserialize(deserializer)?;
                Ok(serde_struct.into())
            }
        }

        impl<TW: ::impex::WrapperSettings> ::impex::IntoImpex<TW> for #original_name {
            type Impex = #impex_name<TW>;

            fn into_impex(self, is_explicit: bool) -> Self::Impex {
                #impex_name(
                    #(#into_impex_fields),*
                )
            }
        }

        impl<TW: ::impex::WrapperSettings> ::impex::Impex<TW> for #impex_name<TW> {
            type Value = #original_name;

            fn is_explicit(&self) -> bool {
                #is_explicit_body
            }

            fn into_value(self) -> Self::Value {
                #original_name(
                    #(#into_value_fields),*
                )
            }

            fn set_impex(&mut self, v: Self::Value, is_explicit: bool) {
                #(#set_impex_fields)*
            }
        }

        impl<TW: ::impex::WrapperSettings> Default for #impex_name<TW> {
            fn default() -> Self {
                let x = #original_name::default();
                Self(
                    #(#default_fields),*
                )
            }
        }

        #visitor_impl
        #eq_impl
        #partial_eq_impl
    }
}

/// Generate a visibility struct for a unit variant.
/// The visibility struct tracks explicit/implicit state and handles serialization.
fn generate_visibility_struct(
    vis: &syn::Visibility,
    visibility_name: &Ident,
    variant_str: &str,
) -> proc_macro2::TokenStream {
    quote! {
        /// Visibility marker for unit variant - tracks explicit/implicit state
        #[derive(Debug, Clone)]
        #vis struct #visibility_name<TW: ::impex::WrapperSettings = ::impex::DefaultWrapperSettings> {
            is_explicit: bool,
            #[doc(hidden)]
            _phantom: ::std::marker::PhantomData<TW>,
        }

        impl<TW: ::impex::WrapperSettings> PartialEq for #visibility_name<TW> {
            fn eq(&self, other: &Self) -> bool {
                self.is_explicit == other.is_explicit
            }
        }

        impl<TW: ::impex::WrapperSettings> Eq for #visibility_name<TW> {}

        impl<TW: ::impex::WrapperSettings> Default for #visibility_name<TW> {
            fn default() -> Self {
                Self { is_explicit: false, _phantom: ::std::marker::PhantomData } // Default is implicit
            }
        }

        impl<TW: ::impex::WrapperSettings> ::serde::Serialize for #visibility_name<TW> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.serialize_str(#variant_str)
            }
        }

        impl<'de, TW: ::impex::WrapperSettings> ::serde::Deserialize<'de> for #visibility_name<TW> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                let _ = String::deserialize(deserializer)?;
                Ok(Self { is_explicit: true, _phantom: ::std::marker::PhantomData })
            }
        }
    }
}

fn generate_enum(ctx: GenerateContext, data_enum: &syn::DataEnum) -> proc_macro2::TokenStream {
    let GenerateContext {
        impex_name,
        original_name,
        vis,
        derives,
        has_partial_eq,
        has_eq,
    } = ctx;

    // Collect unit variants to generate visibility structs for them
    let unit_variants: Vec<_> = data_enum
        .variants
        .iter()
        .filter(|v| matches!(v.fields, Fields::Unit))
        .collect();

    // Generate visibility structs for unit variants
    let visibility_structs: Vec<_> = unit_variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            let visibility_name = Ident::new(
                &format!("{}{}Visibility", impex_name, variant_name),
                variant_name.span(),
            );
            let variant_str = variant_name.to_string();
            generate_visibility_struct(vis, &visibility_name, &variant_str)
        })
        .collect();

    // Generate enum variants - unit variants become tuple variants with visibility struct
    let impex_variants = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        match &variant.fields {
            Fields::Named(fields) => {
                let fields = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let ty = &f.ty;
                    quote! {
                        #name: <#ty as ::impex::IntoImpex<TW>>::Impex
                    }
                });
                quote! {
                    #variant_name {
                        #(#fields),*
                    }
                }
            }
            Fields::Unnamed(fields) => {
                let fields = fields.unnamed.iter().map(|f| {
                    let ty = &f.ty;
                    quote! {
                        <#ty as ::impex::IntoImpex<TW>>::Impex
                    }
                });
                quote! {
                    #variant_name(#(#fields),*)
                }
            }
            // Unit variants become tuple variants with visibility struct
            Fields::Unit => {
                let visibility_name = Ident::new(
                    &format!("{}{}Visibility", impex_name, variant_name),
                    variant_name.span(),
                );
                quote! {
                    #variant_name(#visibility_name<TW>)
                }
            }
        }
    });

    // Generate IntoImpex match arms
    let into_impex_arms = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        match &variant.fields {
            Fields::Named(fields) => {
                let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
                let field_conversions = field_names.iter().map(|name| {
                    quote! {
                        #name: ::impex::IntoImpex::<TW>::into_impex(#name, is_explicit)
                    }
                });
                quote! {
                    #original_name::#variant_name { #(#field_names),* } => #impex_name::#variant_name {
                        #(#field_conversions),*
                    }
                }
            }
            Fields::Unnamed(fields) => {
                let field_names: Vec<Ident> = (0..fields.unnamed.len())
                    .map(|i| Ident::new(&format!("x{}", i + 1), variant_name.span()))
                    .collect();
                let field_conversions = field_names.iter().map(|name| {
                    quote! {
                        ::impex::IntoImpex::<TW>::into_impex(#name, is_explicit)
                    }
                });
                quote! {
                    #original_name::#variant_name(#(#field_names),*) => #impex_name::#variant_name(
                        #(#field_conversions),*
                    )
                }
            }
            // Unit variants use visibility struct
            Fields::Unit => {
                let visibility_name = Ident::new(
                    &format!("{}{}Visibility", impex_name, variant_name),
                    variant_name.span(),
                );
                quote! {
                    #original_name::#variant_name => #impex_name::#variant_name(#visibility_name { is_explicit, _phantom: ::std::marker::PhantomData })
                }
            }
        }
    });

    // Generate is_explicit match arms
    let is_explicit_arms = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        match &variant.fields {
            Fields::Named(fields) => {
                let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
                let checks = field_names.iter().map(|name| {
                    quote! {
                        ::impex::Impex::<TW>::is_explicit(#name)
                    }
                });
                quote! {
                    #impex_name::#variant_name { #(#field_names),* } => {
                        #(#checks)||*
                    }
                }
            }
            Fields::Unnamed(fields) => {
                let field_names: Vec<Ident> = (0..fields.unnamed.len())
                    .map(|i| Ident::new(&format!("x{}", i + 1), variant_name.span()))
                    .collect();
                let checks = field_names.iter().map(|name| {
                    quote! {
                        ::impex::Impex::<TW>::is_explicit(#name)
                    }
                });
                quote! {
                    #impex_name::#variant_name(#(#field_names),*) => {
                        #(#checks)||*
                    }
                }
            }
            // Unit variants check visibility struct
            Fields::Unit => quote! {
                #impex_name::#variant_name(v) => v.is_explicit
            },
        }
    });

    // Generate into_value match arms
    let into_value_arms = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        match &variant.fields {
            Fields::Named(fields) => {
                let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
                let field_conversions = field_names.iter().map(|name| {
                    quote! {
                        #name: ::impex::Impex::<TW>::into_value(#name)
                    }
                });
                quote! {
                    #impex_name::#variant_name { #(#field_names),* } => #original_name::#variant_name {
                        #(#field_conversions),*
                    }
                }
            }
            Fields::Unnamed(fields) => {
                let field_names: Vec<Ident> = (0..fields.unnamed.len())
                    .map(|i| Ident::new(&format!("x{}", i + 1), variant_name.span()))
                    .collect();
                let field_conversions = field_names.iter().map(|name| {
                    quote! {
                        ::impex::Impex::<TW>::into_value(#name)
                    }
                });
                quote! {
                    #impex_name::#variant_name(#(#field_names),*) => #original_name::#variant_name(
                        #(#field_conversions),*
                    )
                }
            }
            // Unit variants return original unit variant
            Fields::Unit => quote! {
                #impex_name::#variant_name(_) => #original_name::#variant_name
            },
        }
    });

    // Generate set_impex match arms
    let set_impex_arms = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        match &variant.fields {
            Fields::Named(fields) => {
                let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
                let field_conversions = field_names.iter().map(|name| {
                    quote! {
                        #name: ::impex::IntoImpex::<TW>::into_impex(#name, is_explicit)
                    }
                });
                quote! {
                    Self::Value::#variant_name { #(#field_names),* } => #impex_name::#variant_name {
                        #(#field_conversions),*
                    }
                }
            }
            Fields::Unnamed(fields) => {
                let field_names: Vec<Ident> = (0..fields.unnamed.len())
                    .map(|i| Ident::new(&format!("x{}", i + 1), variant_name.span()))
                    .collect();
                let field_conversions = field_names.iter().map(|name| {
                    quote! {
                        ::impex::IntoImpex::<TW>::into_impex(#name, is_explicit)
                    }
                });
                quote! {
                    Self::Value::#variant_name(#(#field_names),*) => #impex_name::#variant_name(
                        #(#field_conversions),*
                    )
                }
            }
            // Unit variants use visibility struct
            Fields::Unit => {
                let visibility_name = Ident::new(
                    &format!("{}{}Visibility", impex_name, variant_name),
                    variant_name.span(),
                );
                quote! {
                    Self::Value::#variant_name => #impex_name::#variant_name(#visibility_name { is_explicit, _phantom: ::std::marker::PhantomData })
                }
            }
        }
    });

    // Generate default match arms
    let default_arms = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        match &variant.fields {
            Fields::Named(fields) => {
                let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
                let field_conversions = field_names.iter().map(|name| {
                    quote! {
                        #name: ::impex::IntoImpex::<TW>::into_implicit(#name)
                    }
                });
                quote! {
                    #original_name::#variant_name { #(#field_names),* } => Self::#variant_name {
                        #(#field_conversions),*
                    }
                }
            }
            Fields::Unnamed(fields) => {
                let field_names: Vec<Ident> = (0..fields.unnamed.len())
                    .map(|i| Ident::new(&format!("x{}", i + 1), variant_name.span()))
                    .collect();
                let field_conversions = field_names.iter().map(|name| {
                    quote! {
                        ::impex::IntoImpex::<TW>::into_implicit(#name)
                    }
                });
                quote! {
                    #original_name::#variant_name(#(#field_names),*) => Self::#variant_name(
                        #(#field_conversions),*
                    )
                }
            }
            // Unit variants use default visibility (is_explicit: false)
            Fields::Unit => quote! {
                #original_name::#variant_name => Self::#variant_name(Default::default())
            },
        }
    });

    let visitor_impl = if cfg!(feature = "visitor") {
        let mut visitor_field_types = Vec::new();
        for variant in &data_enum.variants {
            match &variant.fields {
                Fields::Named(fields) => {
                    visitor_field_types.extend(fields.named.iter().map(|f| &f.ty));
                }
                Fields::Unnamed(fields) => {
                    visitor_field_types.extend(fields.unnamed.iter().map(|f| &f.ty));
                }
                Fields::Unit => {}
            }
        }

        let visitor_where_clauses = visitor_field_types.iter().map(|ty| {
            quote! {
                <#ty as ::impex::IntoImpex<TW>>::Impex: ::impex::Visitor<T>
            }
        });

        // Generate Visitor match arms
        let visitor_match_arms = data_enum.variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            match &variant.fields {
                Fields::Named(fields) => {
                    let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
                    let visit_calls = field_names.iter().map(|name| {
                        quote! {
                            ::impex::Visitor::<T>::visit(#name, ctx);
                        }
                    });
                    quote! {
                        Self::#variant_name { #(#field_names),* } => {
                            #(#visit_calls)*
                        }
                    }
                }
                Fields::Unnamed(fields) => {
                    let field_names: Vec<Ident> = (0..fields.unnamed.len())
                        .map(|i| Ident::new(&format!("x{}", i + 1), variant_name.span()))
                        .collect();
                    let visit_calls = field_names.iter().map(|name| {
                        quote! {
                            ::impex::Visitor::<T>::visit(#name, ctx);
                        }
                    });
                    quote! {
                        Self::#variant_name(#(#field_names),*) => {
                            #(#visit_calls)*
                        }
                    }
                }
                // Unit variants (now tuple variants with visibility) have no fields to visit
                Fields::Unit => quote! {
                    Self::#variant_name(_) => {}
                },
            }
        });

        let visitor_where_clauses: Vec<_> = visitor_where_clauses.collect();
        let where_clause = if visitor_where_clauses.is_empty() {
            quote! {}
        } else {
            quote! { where #(#visitor_where_clauses),* }
        };
        quote! {
            impl<T, TW: ::impex::WrapperSettings> ::impex::Visitor<T> for #impex_name<TW>
            #where_clause
            {
                fn visit(&mut self, ctx: &mut T) {
                    match self {
                        #(#visitor_match_arms),*
                    }
                }
            }
        }
    } else {
        quote! {}
    };

    let mut eq_impl = quote! {};
    let mut partial_eq_impl = quote! {};

    if has_partial_eq || has_eq {
        let mut all_field_types = Vec::new();
        for variant in &data_enum.variants {
            match &variant.fields {
                Fields::Named(fields) => {
                    all_field_types.extend(fields.named.iter().map(|f| &f.ty));
                }
                Fields::Unnamed(fields) => {
                    all_field_types.extend(fields.unnamed.iter().map(|f| &f.ty));
                }
                Fields::Unit => {}
            }
        }

        let partial_eq_where_clauses = all_field_types.iter().map(|ty| {
            quote! {
                <#ty as ::impex::IntoImpex<TW>>::Impex: PartialEq
            }
        });

        let eq_where_clauses = all_field_types.iter().map(|ty| {
            quote! {
                <#ty as ::impex::IntoImpex<TW>>::Impex: Eq
            }
        });

        // Generate PartialEq match arms for comparing variants
        let partial_eq_match_arms = data_enum.variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            match &variant.fields {
                Fields::Named(fields) => {
                    let field_names: Vec<_> = fields
                        .named
                        .iter()
                        .map(|f| f.ident.as_ref().unwrap())
                        .collect();
                    let self_fields: Vec<_> = field_names
                        .iter()
                        .map(|name| Ident::new(&format!("self_{}", name), name.span()))
                        .collect();
                    let other_fields: Vec<_> = field_names
                        .iter()
                        .map(|name| Ident::new(&format!("other_{}", name), name.span()))
                        .collect();
                    let comparisons = self_fields.iter().zip(other_fields.iter()).map(|(s, o)| {
                        quote! { #s == #o }
                    });
                    quote! {
                        (Self::#variant_name { #(#field_names: #self_fields),* },
                         Self::#variant_name { #(#field_names: #other_fields),* }) => {
                            #(#comparisons)&&*
                        }
                    }
                }
                Fields::Unnamed(fields) => {
                    let self_fields: Vec<Ident> = (0..fields.unnamed.len())
                        .map(|i| Ident::new(&format!("self_{}", i + 1), variant_name.span()))
                        .collect();
                    let other_fields: Vec<Ident> = (0..fields.unnamed.len())
                        .map(|i| Ident::new(&format!("other_{}", i + 1), variant_name.span()))
                        .collect();
                    let comparisons = self_fields.iter().zip(other_fields.iter()).map(|(s, o)| {
                        quote! { #s == #o }
                    });
                    quote! {
                        (Self::#variant_name(#(#self_fields),*),
                         Self::#variant_name(#(#other_fields),*)) => {
                            #(#comparisons)&&*
                        }
                    }
                }
                // Unit variants (now tuple variants with visibility) compare visibility
                Fields::Unit => quote! {
                    (Self::#variant_name(a), Self::#variant_name(b)) => a == b
                },
            }
        });

        let partial_eq_where_clauses: Vec<_> = partial_eq_where_clauses.collect();
        let eq_where_clauses: Vec<_> = eq_where_clauses.collect();

        if has_partial_eq {
            let where_clause = if partial_eq_where_clauses.is_empty() {
                quote! {}
            } else {
                quote! { where #(#partial_eq_where_clauses),* }
            };
            partial_eq_impl = quote! {
                impl<TW: ::impex::WrapperSettings> PartialEq for #impex_name<TW>
                #where_clause
                {
                    fn eq(&self, other: &Self) -> bool {
                        match (self, other) {
                            #(#partial_eq_match_arms,)*
                            _ => false,
                        }
                    }
                }
            }
        };

        if has_eq {
            let where_clause = if eq_where_clauses.is_empty() {
                quote! {}
            } else {
                quote! { where #(#eq_where_clauses),* }
            };
            eq_impl = quote! {
                impl<TW: ::impex::WrapperSettings> Eq for #impex_name<TW>
                #where_clause
                {}
            }
        }
    }

    // Collect field types for serde where clauses
    let mut serde_field_types = Vec::new();
    for variant in &data_enum.variants {
        match &variant.fields {
            Fields::Named(fields) => {
                serde_field_types.extend(fields.named.iter().map(|f| &f.ty));
            }
            Fields::Unnamed(fields) => {
                serde_field_types.extend(fields.unnamed.iter().map(|f| &f.ty));
            }
            Fields::Unit => {}
        }
    }

    let serialize_where_clauses: Vec<_> = serde_field_types
        .iter()
        .map(|ty| {
            quote! {
                <#ty as ::impex::IntoImpex<TW>>::Impex: ::serde::Serialize
            }
        })
        .collect();

    let deserialize_where_clauses: Vec<_> = serde_field_types
        .iter()
        .map(|ty| {
            quote! {
                <#ty as ::impex::IntoImpex<TW>>::Impex: ::serde::de::DeserializeOwned
            }
        })
        .collect();

    // Collect variant info for serialize/deserialize
    let variant_names_str: Vec<_> = data_enum
        .variants
        .iter()
        .map(|v| v.ident.to_string())
        .collect();

    // Generate serialize match arms
    let serialize_arms: Vec<_> = data_enum.variants.iter().enumerate().map(|(idx, variant)| {
        let variant_name = &variant.ident;
        let variant_str = variant_name.to_string();
        let idx_u32 = idx as u32;
        match &variant.fields {
            Fields::Named(fields) => {
                let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
                let field_count = field_names.len();
                quote! {
                    Self::#variant_name { #(#field_names),* } => {
                        use ::serde::ser::SerializeStructVariant;
                        let mut sv = serializer.serialize_struct_variant(
                            stringify!(#impex_name),
                            #idx_u32,
                            #variant_str,
                            #field_count,
                        )?;
                        #(sv.serialize_field(stringify!(#field_names), #field_names)?;)*
                        sv.end()
                    }
                }
            }
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() == 1 {
                    quote! {
                        Self::#variant_name(x) => {
                            serializer.serialize_newtype_variant(stringify!(#impex_name), #idx_u32, #variant_str, x)
                        }
                    }
                } else {
                    let field_names: Vec<Ident> = (0..fields.unnamed.len())
                        .map(|i| Ident::new(&format!("x{}", i), variant_name.span()))
                        .collect();
                    let field_count = field_names.len();
                    quote! {
                        Self::#variant_name(#(#field_names),*) => {
                            use ::serde::ser::SerializeTupleVariant;
                            let mut tv = serializer.serialize_tuple_variant(
                                stringify!(#impex_name),
                                #idx_u32,
                                #variant_str,
                                #field_count,
                            )?;
                            #(tv.serialize_field(#field_names)?;)*
                            tv.end()
                        }
                    }
                }
            }
            // Unit variants serialize as just the string
            Fields::Unit => {
                quote! {
                    Self::#variant_name(_) => serializer.serialize_str(#variant_str)
                }
            }
        }
    }).collect();

    // Generate deserialize visit_str arms for unit variants
    let deserialize_str_arms: Vec<_> = data_enum.variants.iter().filter_map(|variant| {
        if matches!(variant.fields, Fields::Unit) {
            let variant_name = &variant.ident;
            let variant_str = variant_name.to_string();
            let visibility_name = Ident::new(
                &format!("{}{}Visibility", impex_name, variant_name),
                variant_name.span(),
            );
            Some(quote! {
                #variant_str => Ok(#impex_name::#variant_name(#visibility_name { is_explicit: true, _phantom: ::std::marker::PhantomData }))
            })
        } else {
            None
        }
    }).collect();

    // Generate deserialize visit_map arms for all variants
    let deserialize_map_arms: Vec<_> = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let variant_str = variant_name.to_string();
        match &variant.fields {
            Fields::Named(fields) => {
                let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
                let field_types: Vec<_> = fields.named.iter().map(|f| &f.ty).collect();
                quote! {
                    #variant_str => {
                        #[derive(::serde::Deserialize)]
                        #[serde(bound = "")]
                        struct __Fields<TW: ::impex::WrapperSettings> {
                            #(#field_names: <#field_types as ::impex::IntoImpex<TW>>::Impex),*
                        }
                        let fields: __Fields<TW> = map.next_value()?;
                        Ok(#impex_name::#variant_name { #(#field_names: fields.#field_names),* })
                    }
                }
            }
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() == 1 {
                    let ty = &fields.unnamed[0].ty;
                    quote! {
                        #variant_str => {
                            let value: <#ty as ::impex::IntoImpex<TW>>::Impex = map.next_value()?;
                            Ok(#impex_name::#variant_name(value))
                        }
                    }
                } else {
                    let field_types: Vec<_> = fields.unnamed.iter().map(|f| &f.ty).collect();
                    let field_indices: Vec<syn::Index> = (0..fields.unnamed.len())
                        .map(syn::Index::from)
                        .collect();
                    quote! {
                        #variant_str => {
                            let value: (#(<#field_types as ::impex::IntoImpex<TW>>::Impex),*,) = map.next_value()?;
                            Ok(#impex_name::#variant_name(#(value.#field_indices),*))
                        }
                    }
                }
            }
            // Unit variants in map format (for backwards compat)
            Fields::Unit => {
                let visibility_name = Ident::new(
                    &format!("{}{}Visibility", impex_name, variant_name),
                    variant_name.span(),
                );
                quote! {
                    #variant_str => {
                        let _: ::serde::de::IgnoredAny = map.next_value()?;
                        Ok(#impex_name::#variant_name(#visibility_name { is_explicit: true, _phantom: ::std::marker::PhantomData }))
                    }
                }
            }
        }
    }).collect();

    let serialize_where_clause = if serialize_where_clauses.is_empty() {
        quote! {}
    } else {
        quote! { where #(#serialize_where_clauses),* }
    };
    let deserialize_where_clause = if deserialize_where_clauses.is_empty() {
        quote! {}
    } else {
        quote! { where #(#deserialize_where_clauses),* }
    };

    quote! {
        // Visibility structs for unit variants
        #(#visibility_structs)*

        #[derive(Clone, #derives)]
        #vis enum #impex_name<TW: ::impex::WrapperSettings = ::impex::DefaultWrapperSettings> {
            #(#impex_variants),*
        }

        // Custom Serialize - unit variants as strings, others as normal
        impl<TW: ::impex::WrapperSettings> ::serde::Serialize for #impex_name<TW>
        #serialize_where_clause
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                match self {
                    #(#serialize_arms),*
                }
            }
        }

        // Custom Deserialize - unit variants from strings, others from map
        impl<'de, TW: ::impex::WrapperSettings> ::serde::Deserialize<'de> for #impex_name<TW>
        #deserialize_where_clause
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                use ::serde::de::{self, MapAccess, Visitor};
                use std::fmt;
                use std::marker::PhantomData;

                struct __Visitor<TW>(PhantomData<TW>);

                impl<'de, TW: ::impex::WrapperSettings> Visitor<'de> for __Visitor<TW>
                #deserialize_where_clause
                {
                    type Value = #impex_name<TW>;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(concat!("a string or map for ", stringify!(#impex_name)))
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            #(#deserialize_str_arms,)*
                            _ => Err(de::Error::unknown_variant(value, &[#(#variant_names_str),*])),
                        }
                    }

                    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
                    where
                        M: MapAccess<'de>,
                    {
                        let key: String = map.next_key()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                        let result = match key.as_str() {
                            #(#deserialize_map_arms,)*
                            _ => Err(de::Error::unknown_variant(&key, &[#(#variant_names_str),*])),
                        };
                        if map.next_key::<String>()?.is_some() {
                            return Err(de::Error::custom("expected single variant key"));
                        }
                        result
                    }
                }

                deserializer.deserialize_any(__Visitor(PhantomData))
            }
        }

        impl<TW: ::impex::WrapperSettings> ::impex::IntoImpex<TW> for #original_name {
            type Impex = #impex_name<TW>;

            fn into_impex(self, is_explicit: bool) -> Self::Impex {
                match self {
                    #(#into_impex_arms),*
                }
            }
        }

        impl<TW: ::impex::WrapperSettings> ::impex::Impex<TW> for #impex_name<TW> {
            type Value = #original_name;

            fn is_explicit(&self) -> bool {
                match self {
                    #(#is_explicit_arms),*
                }
            }

            fn into_value(self) -> Self::Value {
                match self {
                    #(#into_value_arms),*
                }
            }

            fn set_impex(&mut self, v: Self::Value, is_explicit: bool) {
                *self = match v {
                    #(#set_impex_arms),*
                };
            }
        }

        impl<TW: ::impex::WrapperSettings> Default for #impex_name<TW> {
            fn default() -> Self {
                let c = #original_name::default();
                match c {
                    #(#default_arms),*
                }
            }
        }

        #visitor_impl
        #eq_impl
        #partial_eq_impl
    }
}
