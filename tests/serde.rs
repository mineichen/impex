use serde::{Deserialize, Serialize};

#[test]
fn serialize_unit_struct() {
    #[derive(serde::Serialize)]
    struct Foo;

    #[derive(serde::Serialize)]
    struct Outer {
        foo: Foo,
    }
    panic!("{}", serde_json::to_string(&Outer { foo: Foo }).unwrap());
}
#[test]
fn serialize_with_defaults() {
    use impex::Impex;
    //use std::ops::{Deref, DerefMut};

    let text = r#"{"num_cores":3}"#;
    let mut obj = serde_json::from_str::<ImpexKeyStructConfig>(text).unwrap();
    assert_eq!(3, *obj.num_cores);
    assert_eq!(42, *obj.num_threads);
    assert!(obj.num_cores.is_explicit());
    assert!(obj.num_threads.is_implicit());
    assert_eq!(text, serde_json::to_string(&obj).unwrap().as_str());

    match &mut obj.enum_config {
        ImpexEnumConfig::Bar(_, x) => x.set(43),
        _ => panic!(),
    }
    let after_set_bar_field_text = serde_json::to_string(&obj).unwrap();

    assert_eq!(
        r#"{"num_cores":3,"enum_config":{"Bar":[null,43]}}"#,
        after_set_bar_field_text.as_str()
    );

    obj.enum_config.set(EnumConfig::Bar("Custom".into(), 42));
    match &obj.enum_config {
        ImpexEnumConfig::Bar(x, y) => {
            assert!(x.is_explicit());
            assert_eq!(x.as_str(), "Custom");
            assert!(y.is_explicit());
            assert_eq!(**y, 42);
        }
        _ => panic!(),
    }
    // *obj.sub.bar.make_defined() = "Custom".into();
    // assert_eq!("Custom", *obj.sub.bar);
    // assert_eq!("Foo", *obj.sub.foo);
    assert_eq!(
        r#"{"num_cores":3,"enum_config":{"Bar":["Custom",42]}}"#,
        serde_json::to_string(&obj).unwrap().as_str()
    );
}

#[derive(serde::Deserialize, serde::Serialize)]
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
#[derive(serde::Deserialize, serde::Serialize)]
struct TupleStructConfig(i32, i64);

impl Default for TupleStructConfig {
    fn default() -> Self {
        Self(42, 43)
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
// #[derive(Impex)]
enum EnumConfig {
    Foo { foo_value: String },
    Bar(String, i32),
}

impl Default for EnumConfig {
    fn default() -> Self {
        EnumConfig::Bar("Bar".into(), 42)
    }
}

///
/// THE following will be auto generated
///

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
struct ImpexKeyStructConfig {
    #[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
    num_cores: <u32 as ::impex::IntoImpex>::Impex,
    #[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
    num_threads: <u32 as ::impex::IntoImpex>::Impex,
    #[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
    enum_config: <EnumConfig as ::impex::IntoImpex>::Impex,
    #[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
    tuple_struct_config: <TupleStructConfig as ::impex::IntoImpex>::Impex,
}

impl ::impex::IntoImpex for KeyStructConfig {
    type Impex = ImpexKeyStructConfig;

    fn into_impex(self, is_expclicit: bool) -> Self::Impex {
        ImpexKeyStructConfig {
            num_cores: ::impex::IntoImpex::into_impex(self.num_cores, is_expclicit),
            num_threads: ::impex::IntoImpex::into_impex(self.num_threads, is_expclicit),
            enum_config: ::impex::IntoImpex::into_impex(self.enum_config, is_expclicit),
            tuple_struct_config: ::impex::IntoImpex::into_impex(
                self.tuple_struct_config,
                is_expclicit,
            ),
        }
    }
}

impl ::impex::Impex for ImpexKeyStructConfig {
    type Value = KeyStructConfig;
    fn is_explicit(&self) -> bool {
        ::impex::Impex::is_explicit(&self.num_cores)
            || ::impex::Impex::is_explicit(&self.num_threads)
            || ::impex::Impex::is_explicit(&self.enum_config)
    }

    fn into_value(self) -> Self::Value {
        KeyStructConfig {
            num_cores: ::impex::Impex::into_value(self.num_cores),
            num_threads: ::impex::Impex::into_value(self.num_threads),
            enum_config: ::impex::Impex::into_value(self.enum_config),
            tuple_struct_config: ::impex::Impex::into_value(self.tuple_struct_config),
        }
    }

    fn set(&mut self, v: Self::Value) {
        ::impex::Impex::set(&mut self.num_cores, v.num_cores);
        ::impex::Impex::set(&mut self.num_threads, v.num_threads);
        ::impex::Impex::set(&mut self.enum_config, v.enum_config);
    }
}

impl Default for ImpexKeyStructConfig {
    fn default() -> Self {
        let x = KeyStructConfig::default();
        Self {
            num_cores: ::impex::IntoImpex::into_implicit(x.num_cores),
            num_threads: ::impex::IntoImpex::into_implicit(x.num_threads),
            enum_config: ::impex::IntoImpex::into_implicit(x.enum_config),
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
enum ImpexEnumConfig {
    //#[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
    Foo {
        foo_value: <String as ::impex::IntoImpex>::Impex,
    },
    //#[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
    Bar(
        <String as ::impex::IntoImpex>::Impex,
        <i32 as ::impex::IntoImpex>::Impex,
    ),
}

impl ::impex::IntoImpex for EnumConfig {
    type Impex = ImpexEnumConfig;

    fn into_impex(self, is_explicit: bool) -> Self::Impex {
        match self {
            EnumConfig::Foo { foo_value } => ImpexEnumConfig::Foo {
                foo_value: ::impex::IntoImpex::into_impex(foo_value, is_explicit),
            },
            EnumConfig::Bar(x1, x2) => ImpexEnumConfig::Bar(
                ::impex::IntoImpex::into_impex(x1, is_explicit),
                ::impex::IntoImpex::into_impex(x2, is_explicit),
            ),
        }
    }
}

impl ::impex::Impex for ImpexEnumConfig {
    type Value = EnumConfig;

    fn is_explicit(&self) -> bool {
        match &self {
            ImpexEnumConfig::Foo { foo_value } => ::impex::Impex::is_explicit(foo_value),
            ImpexEnumConfig::Bar(x1, x2) => {
                ::impex::Impex::is_explicit(x1) || ::impex::Impex::is_explicit(x2)
            }
        }
    }

    fn into_value(self) -> Self::Value {
        match self {
            ImpexEnumConfig::Foo { foo_value } => EnumConfig::Foo {
                foo_value: ::impex::Impex::into_value(foo_value),
            },
            ImpexEnumConfig::Bar(x1, x2) => EnumConfig::Bar(
                ::impex::Impex::into_value(x1),
                ::impex::Impex::into_value(x2),
            ),
        }
    }

    fn set(&mut self, v: Self::Value) {
        *self = match v {
            Self::Value::Foo { foo_value } => ImpexEnumConfig::Foo {
                foo_value: ::impex::IntoImpex::into_explicit(foo_value),
            },
            Self::Value::Bar(x, y) => ImpexEnumConfig::Bar(
                ::impex::IntoImpex::into_explicit(x),
                ::impex::IntoImpex::into_explicit(y),
            ),
        };
    }
}

impl Default for ImpexEnumConfig {
    fn default() -> Self {
        let c = EnumConfig::default();
        match c {
            EnumConfig::Foo { foo_value } => Self::Foo {
                foo_value: ::impex::IntoImpex::into_implicit(foo_value),
            },
            EnumConfig::Bar(x1, x2) => Self::Bar(
                ::impex::IntoImpex::into_implicit(x1),
                ::impex::IntoImpex::into_implicit(x2),
            ),
        }
    }
}

#[derive(PartialEq, Eq, Deserialize, Serialize)]
struct ImpexTupleStructConfig(
    <i32 as ::impex::IntoImpex>::Impex,
    <i64 as ::impex::IntoImpex>::Impex,
);

impl ::impex::IntoImpex for TupleStructConfig {
    type Impex = ImpexTupleStructConfig;

    fn into_impex(self, is_expclicit: bool) -> Self::Impex {
        ImpexTupleStructConfig(
            ::impex::IntoImpex::into_impex(self.0, is_expclicit),
            ::impex::IntoImpex::into_impex(self.1, is_expclicit),
        )
    }
}

impl ::impex::Impex for ImpexTupleStructConfig {
    type Value = TupleStructConfig;
    fn is_explicit(&self) -> bool {
        ::impex::Impex::is_explicit(&self.0) || ::impex::Impex::is_explicit(&self.1)
    }

    fn into_value(self) -> Self::Value {
        TupleStructConfig(
            ::impex::Impex::into_value(self.0),
            ::impex::Impex::into_value(self.1),
        )
    }

    fn set(&mut self, v: Self::Value) {
        ::impex::Impex::set(&mut self.0, v.0);
        ::impex::Impex::set(&mut self.1, v.1);
    }
}

impl Default for ImpexTupleStructConfig {
    fn default() -> Self {
        let x = TupleStructConfig::default();
        Self(
            ::impex::IntoImpex::into_implicit(x.0),
            ::impex::IntoImpex::into_implicit(x.1),
        )
    }
}
