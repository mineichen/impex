#[test]
fn serialize_with_defaults() {
    use impex::Impex;
    //use std::ops::{Deref, DerefMut};

    let text = r#"{"num_cores":3}"#;
    let mut obj =
        serde_json::from_str::<ImpexKeyStructConfig<::impex::DefaultWrapperSettings>>(text)
            .unwrap();
    assert_eq!(3, *obj.num_cores);
    assert_eq!(42, *obj.num_threads);
    assert!(obj.num_cores.is_explicit());
    assert!(obj.num_threads.is_implicit());
    assert_eq!(text, serde_json::to_string(&obj).unwrap().as_str());

    match &mut obj.enum_config {
        ImpexEnumConfig::Bar(_, x, _) => x.set_explicit(43),
        _ => panic!(),
    }
    let after_set_bar_field_text = serde_json::to_string(&obj).unwrap();

    assert_eq!(
        r#"{"num_cores":3,"enum_config":{"Bar":[null,43,[null,null]]}}"#,
        after_set_bar_field_text.as_str()
    );

    obj.enum_config
        .set_explicit(EnumConfig::Bar("Custom".into(), 42, Default::default()));
    match &obj.enum_config {
        ImpexEnumConfig::Bar(x1, x2, x3) => {
            assert!(x1.is_explicit());
            assert_eq!(x1.as_str(), "Custom");

            assert!(x2.is_explicit());
            assert_eq!(**x2, 42);

            assert!(x3.is_explicit());
            assert_eq!(
                x3,
                &::impex::IntoImpex::into_explicit(TupleStructConfig::default())
            );
        }
        _ => panic!(),
    }
    // *obj.sub.bar.make_defined() = "Custom".into();
    // assert_eq!("Custom", *obj.sub.bar);
    // assert_eq!("Foo", *obj.sub.foo);
    assert_eq!(
        r#"{"num_cores":3,"enum_config":{"Bar":["Custom",42,[42,43]]}}"#,
        serde_json::to_string(&obj).unwrap().as_str()
    );
}

#[test]
fn test_serialize_field_enum_skips_implicit_fields() {
    use impex::Impex;
    let text = r#"{"enum_config":{"Foo":{}}}"#;
    let x: ImpexKeyStructConfig<::impex::DefaultWrapperSettings> =
        serde_json::from_str(text).unwrap();

    let ImpexEnumConfig::Foo {
        foo_value,
        tuple_struct_config,
    } = x.enum_config
    else {
        panic!("Should be a FooConfig")
    };
    assert!(foo_value.is_implicit());
    assert_eq!(foo_value.into_value(), String::default());
    assert!(tuple_struct_config.is_implicit());
    assert_eq!(
        tuple_struct_config.into_value(),
        TupleStructConfig::default()
    );
}
#[test]
fn tuple_struct() {
    use impex::Impex;
    let text = r#"[42, 84]"#;
    let tuple_struct: ImpexTupleStructConfig<::impex::DefaultWrapperSettings> =
        serde_json::from_str(text).unwrap();
    assert!(tuple_struct.0.is_explicit());
}

// #[derive(Impex)]
struct KeyStructConfig {
    num_cores: u32,
    num_threads: u32,
    enum_config: EnumConfig,
    tuple_struct_config: TupleStructConfig,
}

impl Default for KeyStructConfig {
    fn default() -> Self {
        Self {
            num_cores: Default::default(),
            num_threads: 42,
            enum_config: Default::default(),
            tuple_struct_config: TupleStructConfig::default(),
        }
    }
}

// #[derive(Impex)]
enum EnumConfig {
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
struct TupleStructConfig(i32, i64);

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
struct ImpexKeyStructConfig<TW: ::impex::WrapperSettings> {
    #[serde(skip_serializing_if = "::impex::Impex::<TW>::is_implicit")]
    num_cores: <u32 as ::impex::IntoImpex<TW>>::Impex,
    #[serde(skip_serializing_if = "::impex::Impex::<TW>::is_implicit")]
    num_threads: <u32 as ::impex::IntoImpex<TW>>::Impex,
    #[serde(skip_serializing_if = "::impex::Impex::<TW>::is_implicit")]
    enum_config: <EnumConfig as ::impex::IntoImpex<TW>>::Impex,
    #[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
    tuple_struct_config: <TupleStructConfig as ::impex::IntoImpex<TW>>::Impex,
}

impl<TW: ::impex::WrapperSettings> ::impex::IntoImpex<TW> for KeyStructConfig {
    type Impex = ImpexKeyStructConfig<TW>;

    fn into_impex(self, is_expclicit: bool) -> Self::Impex {
        ImpexKeyStructConfig {
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

impl<TW: ::impex::WrapperSettings> ::impex::Impex<TW> for ImpexKeyStructConfig<TW> {
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

impl<TW: ::impex::WrapperSettings> Default for ImpexKeyStructConfig<TW> {
    fn default() -> Self {
        let x = KeyStructConfig::default();
        Self {
            num_cores: ::impex::IntoImpex::<TW>::into_implicit(x.num_cores),
            num_threads: ::impex::IntoImpex::<TW>::into_implicit(x.num_threads),
            enum_config: ::impex::IntoImpex::<TW>::into_implicit(x.enum_config),
            tuple_struct_config: ::impex::IntoImpex::into_implicit(x.tuple_struct_config),
        }
    }
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
#[derive(PartialEq, Eq, serde::Deserialize, serde::Serialize)]
enum ImpexEnumConfig<TW: ::impex::WrapperSettings> {
    Foo {
        #[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
        #[serde(default)]
        foo_value: <String as ::impex::IntoImpex<TW>>::Impex,
        #[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
        #[serde(default)]
        tuple_struct_config: <TupleStructConfig as ::impex::IntoImpex<TW>>::Impex,
    },
    Bar(
        <String as ::impex::IntoImpex<TW>>::Impex,
        <i32 as ::impex::IntoImpex<TW>>::Impex,
        <TupleStructConfig as ::impex::IntoImpex<TW>>::Impex,
    ),
}

impl<TW: impex::WrapperSettings> ::impex::IntoImpex<TW> for EnumConfig {
    type Impex = ImpexEnumConfig<TW>;

    fn into_impex(self, is_explicit: bool) -> Self::Impex {
        match self {
            EnumConfig::Foo {
                foo_value,
                tuple_struct_config,
            } => ImpexEnumConfig::Foo {
                foo_value: ::impex::IntoImpex::<TW>::into_impex(foo_value, is_explicit),
                tuple_struct_config: ::impex::IntoImpex::into_impex(
                    tuple_struct_config,
                    is_explicit,
                ),
            },
            EnumConfig::Bar(x1, x2, x3) => ImpexEnumConfig::Bar(
                ::impex::IntoImpex::<TW>::into_impex(x1, is_explicit),
                ::impex::IntoImpex::<TW>::into_impex(x2, is_explicit),
                ::impex::IntoImpex::<TW>::into_impex(x3, is_explicit),
            ),
        }
    }
}

impl<TW: ::impex::WrapperSettings> ::impex::Impex<TW> for ImpexEnumConfig<TW> {
    type Value = EnumConfig;

    fn is_explicit(&self) -> bool {
        match &self {
            ImpexEnumConfig::Foo {
                foo_value,
                tuple_struct_config,
            } => {
                ::impex::Impex::<TW>::is_explicit(foo_value)
                    || impex::Impex::is_explicit(tuple_struct_config)
            }
            ImpexEnumConfig::Bar(x1, x2, x3) => {
                ::impex::Impex::<TW>::is_explicit(x1)
                    || ::impex::Impex::<TW>::is_explicit(x2)
                    || ::impex::Impex::<TW>::is_explicit(x3)
            }
        }
    }

    fn into_value(self) -> Self::Value {
        match self {
            ImpexEnumConfig::Foo {
                foo_value,
                tuple_struct_config,
            } => EnumConfig::Foo {
                foo_value: ::impex::Impex::<TW>::into_value(foo_value),
                tuple_struct_config: ::impex::Impex::into_value(tuple_struct_config),
            },
            ImpexEnumConfig::Bar(x1, x2, x3) => EnumConfig::Bar(
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
            } => ImpexEnumConfig::Foo {
                foo_value: ::impex::IntoImpex::<TW>::into_impex(foo_value, is_explicit),
                tuple_struct_config: ::impex::IntoImpex::<TW>::into_impex(
                    tuple_struct_config,
                    is_explicit,
                ),
            },
            Self::Value::Bar(x1, x2, x3) => ImpexEnumConfig::Bar(
                ::impex::IntoImpex::<TW>::into_impex(x1, is_explicit),
                ::impex::IntoImpex::<TW>::into_impex(x2, is_explicit),
                ::impex::IntoImpex::<TW>::into_impex(x3, is_explicit),
            ),
        };
    }
}

impl<TW: ::impex::WrapperSettings> Default for ImpexEnumConfig<TW> {
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

#[derive(PartialEq, Eq, serde::Deserialize, serde::Serialize, Debug)]
struct ImpexTupleStructConfig<TW: ::impex::WrapperSettings>(
    <i32 as ::impex::IntoImpex<TW>>::Impex,
    <i64 as ::impex::IntoImpex<TW>>::Impex,
);

impl<TW: ::impex::WrapperSettings> ::impex::IntoImpex<TW> for TupleStructConfig {
    type Impex = ImpexTupleStructConfig<TW>;

    fn into_impex(self, is_expclicit: bool) -> Self::Impex {
        ImpexTupleStructConfig(
            ::impex::IntoImpex::<TW>::into_impex(self.0, is_expclicit),
            ::impex::IntoImpex::<TW>::into_impex(self.1, is_expclicit),
        )
    }
}

impl<TW: ::impex::WrapperSettings> ::impex::Impex<TW> for ImpexTupleStructConfig<TW> {
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

impl<TW: ::impex::WrapperSettings> Default for ImpexTupleStructConfig<TW> {
    fn default() -> Self {
        let x = TupleStructConfig::default();
        Self(
            ::impex::IntoImpex::<TW>::into_implicit(x.0),
            ::impex::IntoImpex::<TW>::into_implicit(x.1),
        )
    }
}
