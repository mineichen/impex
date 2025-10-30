use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, Index, parse_macro_input};

#[proc_macro_derive(Impex)]
pub fn derive_impex(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let impex_name = Ident::new(&format!("Impex{}", name), name.span());

    let expanded = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => generate_named_struct(&impex_name, name, fields),
            Fields::Unnamed(fields) => generate_tuple_struct(&impex_name, name, fields),
            Fields::Unit => panic!("Unit structs are not supported"),
        },
        Data::Enum(data_enum) => generate_enum(&impex_name, name, data_enum),
        Data::Union(_) => panic!("Unions are not supported"),
    };

    TokenStream::from(expanded)
}

fn generate_named_struct(
    impex_name: &Ident,
    original_name: &Ident,
    fields: &syn::FieldsNamed,
) -> proc_macro2::TokenStream {
    let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();

    // Generate the Impex struct definition
    let impex_fields = fields.named.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {
            #[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
            pub #name: <#ty as ::impex::IntoImpex<TW>>::Impex
        }
    });

    // Generate IntoImpex implementation
    let into_impex_fields = field_names.iter().map(|name| {
        quote! {
            #name: ::impex::IntoImpex::<TW>::into_impex(self.#name, is_explicit)
        }
    });

    // Generate is_explicit check (all fields OR'd together, excluding last field for structs)
    let is_explicit_checks = field_names
        .iter()
        .take(field_names.len().saturating_sub(1))
        .map(|name| {
            quote! {
                ::impex::Impex::<TW>::is_explicit(&self.#name)
            }
        });

    let is_explicit_body = if is_explicit_checks.len() > 0 {
        quote! {
            #(#is_explicit_checks)||*
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

    // Generate set_impex implementation (excluding last field)
    let set_impex_fields = field_names
        .iter()
        .take(field_names.len().saturating_sub(1))
        .map(|name| {
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

    quote! {
        #[derive(serde::Deserialize, serde::Serialize)]
        #[serde(default)]
        pub struct #impex_name<TW: ::impex::WrapperSettings = ::impex::DefaultWrapperSettings> {
            #(#impex_fields),*
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
    }
}

fn generate_tuple_struct(
    impex_name: &Ident,
    original_name: &Ident,
    fields: &syn::FieldsUnnamed,
) -> proc_macro2::TokenStream {
    let field_types: Vec<_> = fields.unnamed.iter().map(|f| &f.ty).collect();
    let field_indices: Vec<Index> = (0..fields.unnamed.len()).map(Index::from).collect();

    // Generate the Impex struct definition
    let impex_fields = field_types.iter().map(|ty| {
        quote! {
            pub <#ty as ::impex::IntoImpex<TW>>::Impex
        }
    });

    // Generate IntoImpex implementation
    let into_impex_fields = field_indices.iter().map(|idx| {
        quote! {
            ::impex::IntoImpex::<TW>::into_impex(self.#idx, is_explicit)
        }
    });

    // Generate is_explicit check (all fields OR'd together)
    let is_explicit_checks = field_indices.iter().map(|idx| {
        quote! {
            ::impex::Impex::<TW>::is_explicit(&self.#idx)
        }
    });

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

    quote! {
        #[derive(PartialEq, Eq, serde::Deserialize, serde::Serialize, Debug)]
        pub struct #impex_name<TW: ::impex::WrapperSettings = ::impex::DefaultWrapperSettings>(
            #(#impex_fields),*
        );

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
                #(#is_explicit_checks)||*
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
    }
}

fn generate_enum(
    impex_name: &Ident,
    original_name: &Ident,
    data_enum: &syn::DataEnum,
) -> proc_macro2::TokenStream {
    // Generate enum variants
    let impex_variants = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        match &variant.fields {
            Fields::Named(fields) => {
                let fields = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let ty = &f.ty;
                    quote! {
                        #[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
                        #[serde(default)]
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
            Fields::Unit => quote! { #variant_name },
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
            Fields::Unit => quote! {
                #original_name::#variant_name => #impex_name::#variant_name
            },
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
            Fields::Unit => quote! {
                #impex_name::#variant_name => false
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
            Fields::Unit => quote! {
                #impex_name::#variant_name => #original_name::#variant_name
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
            Fields::Unit => quote! {
                Self::Value::#variant_name => #impex_name::#variant_name
            },
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
            Fields::Unit => quote! {
                #original_name::#variant_name => Self::#variant_name
            },
        }
    });

    quote! {
        #[derive(PartialEq, Eq, serde::Deserialize, serde::Serialize)]
        pub enum #impex_name<TW: ::impex::WrapperSettings = ::impex::DefaultWrapperSettings> {
            #(#impex_variants),*
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
    }
}
