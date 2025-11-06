// #[derive(Impex)]
pub struct KeyStructConfig {
    pub num_cores: u32,
    pub num_threads: Vec<u32>,
    pub enum_config: EnumConfig,
    pub tuple_struct_config: TupleStructConfig,
}

impl Default for KeyStructConfig {
    fn default() -> Self {
        Self {
            num_cores: Default::default(),
            num_threads: vec![42],
            enum_config: Default::default(),
            tuple_struct_config: TupleStructConfig::default(),
        }
    }
}

// #[derive(Impex)]
pub enum EnumConfig {
    Foo {
        foo_value: String,
        tuple_struct_config: TupleStructConfig,
    },
    Bar(String, i32, TupleStructConfig),
}

impl Default for EnumConfig {
    fn default() -> Self {
        EnumConfig::Bar("Bar".into(), 42, Default::default())
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub struct TupleStructConfig(pub i32, pub i64);

impl Default for TupleStructConfig {
    fn default() -> Self {
        Self(42, 43)
    }
}

///
/// THE following will be auto generated
///

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct KeyStructConfigImpex<TW: ::impex::WrapperSettings = ::impex::DefaultWrapperSettings> {
    #[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
    pub num_cores: <u32 as ::impex::IntoImpex<TW>>::Impex,
    #[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
    pub num_threads: <Vec<u32> as ::impex::IntoImpex<TW>>::Impex,
    #[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
    pub enum_config: <EnumConfig as ::impex::IntoImpex<TW>>::Impex,
    #[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
    pub tuple_struct_config: <TupleStructConfig as ::impex::IntoImpex<TW>>::Impex,
}

impl<TW: ::impex::WrapperSettings> ::impex::IntoImpex<TW> for KeyStructConfig {
    type Impex = KeyStructConfigImpex<TW>;

    fn into_impex(self, is_expclicit: bool) -> Self::Impex {
        KeyStructConfigImpex {
            num_cores: ::impex::IntoImpex::<TW>::into_impex(self.num_cores, is_expclicit),
            num_threads: ::impex::IntoImpex::<TW>::into_impex(self.num_threads, is_expclicit),
            enum_config: ::impex::IntoImpex::<TW>::into_impex(self.enum_config, is_expclicit),
            tuple_struct_config: ::impex::IntoImpex::<TW>::into_impex(
                self.tuple_struct_config,
                is_expclicit,
            ),
        }
    }
}

impl<TW: ::impex::WrapperSettings> ::impex::Impex<TW> for KeyStructConfigImpex<TW> {
    type Value = KeyStructConfig;
    fn is_explicit(&self) -> bool {
        ::impex::Impex::<TW>::is_explicit(&self.num_cores)
            || ::impex::Impex::<TW>::is_explicit(&self.num_threads)
            || ::impex::Impex::<TW>::is_explicit(&self.enum_config)
    }

    fn into_value(self) -> Self::Value {
        KeyStructConfig {
            num_cores: ::impex::Impex::<TW>::into_value(self.num_cores),
            num_threads: ::impex::Impex::<TW>::into_value(self.num_threads),
            enum_config: ::impex::Impex::<TW>::into_value(self.enum_config),
            tuple_struct_config: ::impex::Impex::into_value(self.tuple_struct_config),
        }
    }

    fn set_impex(&mut self, v: Self::Value, is_explicit: bool) {
        ::impex::Impex::<TW>::set_impex(&mut self.num_cores, v.num_cores, is_explicit);
        ::impex::Impex::<TW>::set_impex(&mut self.num_threads, v.num_threads, is_explicit);
        ::impex::Impex::<TW>::set_impex(&mut self.enum_config, v.enum_config, is_explicit);
    }
}

impl<TW: ::impex::WrapperSettings> Default for KeyStructConfigImpex<TW> {
    fn default() -> Self {
        let x = KeyStructConfig::default();
        Self {
            num_cores: ::impex::IntoImpex::<TW>::into_implicit(x.num_cores),
            num_threads: ::impex::IntoImpex::<TW>::into_implicit(x.num_threads),
            enum_config: ::impex::IntoImpex::<TW>::into_implicit(x.enum_config),
            tuple_struct_config: ::impex::IntoImpex::<TW>::into_implicit(x.tuple_struct_config),
        }
    }
}

impl<TW: ::impex::WrapperSettings> PartialEq for KeyStructConfigImpex<TW>
where
    <u32 as ::impex::IntoImpex<TW>>::Impex: PartialEq,
    <Vec<u32> as ::impex::IntoImpex<TW>>::Impex: PartialEq,
    <EnumConfig as ::impex::IntoImpex<TW>>::Impex: PartialEq,
    <TupleStructConfig as ::impex::IntoImpex<TW>>::Impex: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.num_cores == other.num_cores
            && self.num_threads == other.num_threads
            && self.enum_config == other.enum_config
            && self.tuple_struct_config == other.tuple_struct_config
    }
}

impl<TW: ::impex::WrapperSettings> Eq for KeyStructConfigImpex<TW>
where
    <u32 as ::impex::IntoImpex<TW>>::Impex: Eq,
    <Vec<u32> as ::impex::IntoImpex<TW>>::Impex: Eq,
    <EnumConfig as ::impex::IntoImpex<TW>>::Impex: Eq,
    <TupleStructConfig as ::impex::IntoImpex<TW>>::Impex: Eq,
{
}

// #[derive(serde::Deserialize, serde::Serialize)]
// struct ImpexSubConfigWrapper(ImpexSubConfig);
// impl std::ops::Deref for ImpexSubConfigWrapper {
//     type Target = ImpexSubConfig;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl std::ops::DerefMut for ImpexSubConfigWrapper {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

// impl ::impex::Impex for ImpexSubConfigWrapper {
//     type Value = SubConfig;

//     fn is_explicit(&self) -> bool {
//         ::impex::Impex::is_explicit(&self.0)
//     }

//     fn into_value(self) -> Self::Value {
//         ::impex::Impex::into_value(self.0)
//     }

//     fn set(&mut self, v: Self::Value) {
//         ::impex::Impex::set(&mut self.0, v)
//     }
// }
//
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum EnumConfigImpex<TW: ::impex::WrapperSettings = ::impex::DefaultWrapperSettings> {
    Foo {
        #[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
        //#[serde(default)]
        foo_value: <String as ::impex::IntoImpex<TW>>::Impex,
        #[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
        //#[serde(default)]
        tuple_struct_config: <TupleStructConfig as ::impex::IntoImpex<TW>>::Impex,
    },
    Bar(
        <String as ::impex::IntoImpex<TW>>::Impex,
        <i32 as ::impex::IntoImpex<TW>>::Impex,
        <TupleStructConfig as ::impex::IntoImpex<TW>>::Impex,
    ),
}

impl<TW: impex::WrapperSettings> ::impex::IntoImpex<TW> for EnumConfig {
    type Impex = EnumConfigImpex<TW>;

    fn into_impex(self, is_explicit: bool) -> Self::Impex {
        match self {
            EnumConfig::Foo {
                foo_value,
                tuple_struct_config,
            } => EnumConfigImpex::Foo {
                foo_value: ::impex::IntoImpex::<TW>::into_impex(foo_value, is_explicit),
                tuple_struct_config: ::impex::IntoImpex::into_impex(
                    tuple_struct_config,
                    is_explicit,
                ),
            },
            EnumConfig::Bar(x1, x2, x3) => EnumConfigImpex::Bar(
                ::impex::IntoImpex::<TW>::into_impex(x1, is_explicit),
                ::impex::IntoImpex::<TW>::into_impex(x2, is_explicit),
                ::impex::IntoImpex::<TW>::into_impex(x3, is_explicit),
            ),
        }
    }
}

impl<TW: ::impex::WrapperSettings> ::impex::Impex<TW> for EnumConfigImpex<TW> {
    type Value = EnumConfig;

    fn is_explicit(&self) -> bool {
        match &self {
            EnumConfigImpex::Foo {
                foo_value,
                tuple_struct_config,
            } => {
                ::impex::Impex::<TW>::is_explicit(foo_value)
                    || impex::Impex::is_explicit(tuple_struct_config)
            }
            EnumConfigImpex::Bar(x1, x2, x3) => {
                ::impex::Impex::<TW>::is_explicit(x1)
                    || ::impex::Impex::<TW>::is_explicit(x2)
                    || ::impex::Impex::<TW>::is_explicit(x3)
            }
        }
    }

    fn into_value(self) -> Self::Value {
        match self {
            EnumConfigImpex::Foo {
                foo_value,
                tuple_struct_config,
            } => EnumConfig::Foo {
                foo_value: ::impex::Impex::<TW>::into_value(foo_value),
                tuple_struct_config: ::impex::Impex::into_value(tuple_struct_config),
            },
            EnumConfigImpex::Bar(x1, x2, x3) => EnumConfig::Bar(
                ::impex::Impex::<TW>::into_value(x1),
                ::impex::Impex::<TW>::into_value(x2),
                ::impex::Impex::<TW>::into_value(x3),
            ),
        }
    }

    fn set_impex(&mut self, v: Self::Value, is_explicit: bool) {
        *self = match v {
            Self::Value::Foo {
                foo_value,
                tuple_struct_config,
            } => EnumConfigImpex::Foo {
                foo_value: ::impex::IntoImpex::<TW>::into_impex(foo_value, is_explicit),
                tuple_struct_config: ::impex::IntoImpex::<TW>::into_impex(
                    tuple_struct_config,
                    is_explicit,
                ),
            },
            Self::Value::Bar(x1, x2, x3) => EnumConfigImpex::Bar(
                ::impex::IntoImpex::<TW>::into_impex(x1, is_explicit),
                ::impex::IntoImpex::<TW>::into_impex(x2, is_explicit),
                ::impex::IntoImpex::<TW>::into_impex(x3, is_explicit),
            ),
        };
    }
}

impl<TW: ::impex::WrapperSettings> Default for EnumConfigImpex<TW> {
    fn default() -> Self {
        let c = EnumConfig::default();
        match c {
            EnumConfig::Foo {
                foo_value,
                tuple_struct_config,
            } => Self::Foo {
                foo_value: ::impex::IntoImpex::<TW>::into_implicit(foo_value),
                tuple_struct_config: ::impex::IntoImpex::into_implicit(tuple_struct_config),
            },
            EnumConfig::Bar(x1, x2, x3) => Self::Bar(
                ::impex::IntoImpex::<TW>::into_implicit(x1),
                ::impex::IntoImpex::<TW>::into_implicit(x2),
                ::impex::IntoImpex::<TW>::into_implicit(x3),
            ),
        }
    }
}

impl<TW: ::impex::WrapperSettings> PartialEq for EnumConfigImpex<TW>
where
    <String as ::impex::IntoImpex<TW>>::Impex: PartialEq,
    <TupleStructConfig as ::impex::IntoImpex<TW>>::Impex: PartialEq,
    <i32 as ::impex::IntoImpex<TW>>::Impex: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Foo {
                    foo_value: self_foo_value,
                    tuple_struct_config: self_tuple_struct_config,
                },
                Self::Foo {
                    foo_value: other_foo_value,
                    tuple_struct_config: other_tuple_struct_config,
                },
            ) => {
                self_foo_value == other_foo_value
                    && self_tuple_struct_config == other_tuple_struct_config
            }
            (Self::Bar(self_1, self_2, self_3), Self::Bar(other_1, other_2, other_3)) => {
                self_1 == other_1 && self_2 == other_2 && self_3 == other_3
            }
            _ => false,
        }
    }
}

impl<TW: ::impex::WrapperSettings> Eq for EnumConfigImpex<TW>
where
    <String as ::impex::IntoImpex<TW>>::Impex: Eq,
    <TupleStructConfig as ::impex::IntoImpex<TW>>::Impex: Eq,
    <i32 as ::impex::IntoImpex<TW>>::Impex: Eq,
{
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TupleStructConfigImpex<TW: ::impex::WrapperSettings = ::impex::DefaultWrapperSettings>(
    pub <i32 as ::impex::IntoImpex<TW>>::Impex,
    pub <i64 as ::impex::IntoImpex<TW>>::Impex,
);

impl<TW: ::impex::WrapperSettings> ::impex::IntoImpex<TW> for TupleStructConfig {
    type Impex = TupleStructConfigImpex<TW>;

    fn into_impex(self, is_expclicit: bool) -> Self::Impex {
        TupleStructConfigImpex(
            ::impex::IntoImpex::<TW>::into_impex(self.0, is_expclicit),
            ::impex::IntoImpex::<TW>::into_impex(self.1, is_expclicit),
        )
    }
}

impl<TW: ::impex::WrapperSettings> ::impex::Impex<TW> for TupleStructConfigImpex<TW> {
    type Value = TupleStructConfig;
    fn is_explicit(&self) -> bool {
        ::impex::Impex::<TW>::is_explicit(&self.0) || ::impex::Impex::<TW>::is_explicit(&self.1)
    }

    fn into_value(self) -> Self::Value {
        TupleStructConfig(
            ::impex::Impex::<TW>::into_value(self.0),
            ::impex::Impex::<TW>::into_value(self.1),
        )
    }

    fn set_impex(&mut self, v: Self::Value, is_explicit: bool) {
        ::impex::Impex::<TW>::set_impex(&mut self.0, v.0, is_explicit);
        ::impex::Impex::<TW>::set_impex(&mut self.1, v.1, is_explicit);
    }
}

impl<TW: ::impex::WrapperSettings> Default for TupleStructConfigImpex<TW> {
    fn default() -> Self {
        let x = TupleStructConfig::default();
        Self(
            ::impex::IntoImpex::<TW>::into_implicit(x.0),
            ::impex::IntoImpex::<TW>::into_implicit(x.1),
        )
    }
}

impl<TW: ::impex::WrapperSettings> PartialEq for TupleStructConfigImpex<TW>
where
    <i32 as ::impex::IntoImpex<TW>>::Impex: PartialEq,
    <i64 as ::impex::IntoImpex<TW>>::Impex: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl<TW: ::impex::WrapperSettings> Eq for TupleStructConfigImpex<TW>
where
    <i32 as ::impex::IntoImpex<TW>>::Impex: Eq,
    <i64 as ::impex::IntoImpex<TW>>::Impex: Eq,
{
}

#[cfg(feature = "visitor")]
impl<T, TW: ::impex::WrapperSettings> ::impex::Visitor<T> for KeyStructConfigImpex<TW>
where
    <u32 as ::impex::IntoImpex<TW>>::Impex: ::impex::Visitor<T>,
    <Vec<u32> as ::impex::IntoImpex<TW>>::Impex: ::impex::Visitor<T>,
    <EnumConfig as ::impex::IntoImpex<TW>>::Impex: ::impex::Visitor<T>,
    <TupleStructConfig as ::impex::IntoImpex<TW>>::Impex: ::impex::Visitor<T>,
{
    fn visit(&mut self, ctx: &mut T) {
        ::impex::Visitor::<T>::visit(&mut self.num_cores, ctx);
        ::impex::Visitor::<T>::visit(&mut self.num_threads, ctx);
        ::impex::Visitor::<T>::visit(&mut self.enum_config, ctx);
        ::impex::Visitor::<T>::visit(&mut self.tuple_struct_config, ctx);
    }
}
#[cfg(feature = "visitor")]
impl<T, TW: ::impex::WrapperSettings> ::impex::Visitor<T> for EnumConfigImpex<TW>
where
    <String as ::impex::IntoImpex<TW>>::Impex: ::impex::Visitor<T>,
    <TupleStructConfig as ::impex::IntoImpex<TW>>::Impex: ::impex::Visitor<T>,
    <String as ::impex::IntoImpex<TW>>::Impex: ::impex::Visitor<T>,
    <i32 as ::impex::IntoImpex<TW>>::Impex: ::impex::Visitor<T>,
    <TupleStructConfig as ::impex::IntoImpex<TW>>::Impex: ::impex::Visitor<T>,
{
    fn visit(&mut self, ctx: &mut T) {
        match self {
            Self::Foo {
                foo_value,
                tuple_struct_config,
            } => {
                ::impex::Visitor::<T>::visit(foo_value, ctx);
                ::impex::Visitor::<T>::visit(tuple_struct_config, ctx);
            }
            Self::Bar(x1, x2, x3) => {
                ::impex::Visitor::<T>::visit(x1, ctx);
                ::impex::Visitor::<T>::visit(x2, ctx);
                ::impex::Visitor::<T>::visit(x3, ctx);
            }
        }
    }
}

#[cfg(feature = "visitor")]
impl<T, TW: ::impex::WrapperSettings> ::impex::Visitor<T> for TupleStructConfigImpex<TW>
where
    <i32 as ::impex::IntoImpex<TW>>::Impex: ::impex::Visitor<T>,
    <i64 as ::impex::IntoImpex<TW>>::Impex: ::impex::Visitor<T>,
{
    fn visit(&mut self, ctx: &mut T) {
        ::impex::Visitor::<T>::visit(&mut self.0, ctx);
        ::impex::Visitor::<T>::visit(&mut self.1, ctx);
    }
}
