#[test]
fn serialize_with_defaults() {
    use impex::Impex;

    let text = r#"{"num_cores":3}"#;
    let mut obj = serde_json::from_str::<ExplicitConfig>(text).unwrap();
    assert_eq!(3, *obj.num_cores);
    assert_eq!(42, *obj.num_threads);
    assert!(obj.num_cores.is_explicit());
    assert!(obj.num_threads.is_implicit());
    assert_eq!(text, serde_json::to_string(&obj).unwrap().as_str());
    obj.sub.set(SubConfig::Bar("Custom".into(), 42));
    match &obj.sub.0 {
        ExplicitSubConfigVariant::Bar(x, y) => {
            assert_eq!(x.as_str(), "Custom");
            assert_eq!(**y, 42);
        }
        _ => panic!(),
    }
    // *obj.sub.bar.make_defined() = "Custom".into();
    // assert_eq!("Custom", *obj.sub.bar);
    // assert_eq!("Foo", *obj.sub.foo);
    assert_eq!(
        r#"{"num_cores":3,"sub":{"Bar":["Custom",42]}}"#,
        serde_json::to_string(&obj).unwrap().as_str()
    );
}

#[derive(serde::Deserialize, serde::Serialize)]
// #[derive(Impex)]
struct Config {
    num_cores: u32,
    num_threads: u32,
    sub: SubConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            num_cores: Default::default(),
            num_threads: 42,
            sub: Default::default(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
// #[derive(Impex)]
enum SubConfig {
    Foo(String),
    Bar(String, i32),
}

impl Default for SubConfig {
    fn default() -> Self {
        SubConfig::Bar("Bar".into(), 42)
    }
}

///
/// THE following will be auto generated
///
impl ::impex::IntoImpex for Config {
    type Explicit = ExplicitConfig;

    fn into_impex(self, is_expclicit: bool) -> Self::Explicit {
        Self::Explicit {
            num_cores: ::impex::IntoImpex::into_impex(self.num_cores, is_expclicit),
            num_threads: ::impex::IntoImpex::into_impex(self.num_threads, is_expclicit),
            sub: ::impex::IntoImpex::into_impex(self.sub, is_expclicit),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
struct ExplicitConfig {
    #[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
    num_cores: <u32 as ::impex::IntoImpex>::Explicit,
    #[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
    num_threads: <u32 as ::impex::IntoImpex>::Explicit,
    #[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
    sub: <SubConfig as ::impex::IntoImpex>::Explicit,
}

impl ::impex::Impex for ExplicitConfig {
    type Value = Config;
    fn is_explicit(&self) -> bool {
        ::impex::Impex::is_explicit(&self.num_cores)
            || ::impex::Impex::is_explicit(&self.num_threads)
            || ::impex::Impex::is_explicit(&self.sub)
    }

    fn into_value(self) -> Self::Value {
        Self::Value {
            num_cores: ::impex::Impex::into_value(self.num_cores),
            num_threads: ::impex::Impex::into_value(self.num_threads),
            sub: ::impex::Impex::into_value(self.sub),
        }
    }

    fn set(&mut self, v: Self::Value) {
        ::impex::Impex::set(&mut self.num_cores, v.num_cores);
        ::impex::Impex::set(&mut self.num_threads, v.num_threads);
        ::impex::Impex::set(&mut self.sub, v.sub);
    }
}

impl Default for ExplicitConfig {
    fn default() -> Self {
        let c = Config::default();
        Self {
            num_cores: ::impex::IntoImpex::into_implicit(c.num_cores),
            num_threads: ::impex::IntoImpex::into_implicit(c.num_threads),
            sub: ::impex::IntoImpex::into_implicit(c.sub),
        }
    }
}

impl ::impex::IntoImpex for SubConfig {
    type Explicit = ExplicitSubConfig;

    fn into_impex(self, is_explicit: bool) -> Self::Explicit {
        ExplicitSubConfig(match self {
            SubConfig::Foo(x) => {
                ExplicitSubConfigVariant::Foo(::impex::IntoImpex::into_impex(x, is_explicit))
            }
            SubConfig::Bar(x1, x2) => ExplicitSubConfigVariant::Bar(
                ::impex::IntoImpex::into_impex(x1, is_explicit),
                ::impex::IntoImpex::into_impex(x2, is_explicit),
            ),
        })
    }

    fn into_explicit(self) -> Self::Explicit {
        ExplicitSubConfig(match self {
            SubConfig::Foo(x) => {
                ExplicitSubConfigVariant::Foo(::impex::IntoImpex::into_explicit(x))
            }
            SubConfig::Bar(x1, x2) => ExplicitSubConfigVariant::Bar(
                ::impex::IntoImpex::into_explicit(x1),
                ::impex::IntoImpex::into_explicit(x2),
            ),
        })
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
struct ExplicitSubConfig(ExplicitSubConfigVariant);

impl ::impex::Impex for ExplicitSubConfig {
    type Value = SubConfig;

    fn is_explicit(&self) -> bool {
        match &self.0 {
            ExplicitSubConfigVariant::Foo(x) => ::impex::Impex::is_explicit(x),
            ExplicitSubConfigVariant::Bar(x1, x2) => {
                ::impex::Impex::is_explicit(x1) || ::impex::Impex::is_explicit(x2)
            }
        }
    }

    fn into_value(self) -> Self::Value {
        match self.0 {
            ExplicitSubConfigVariant::Foo(x) => SubConfig::Foo(::impex::Impex::into_value(x)),
            ExplicitSubConfigVariant::Bar(x1, x2) => SubConfig::Bar(
                ::impex::Impex::into_value(x1),
                ::impex::Impex::into_value(x2),
            ),
        }
    }

    fn set(&mut self, v: Self::Value) {
        self.0 = match v {
            Self::Value::Foo(x) => {
                ExplicitSubConfigVariant::Foo(::impex::IntoImpex::into_explicit(x))
            }
            Self::Value::Bar(x, y) => ExplicitSubConfigVariant::Bar(
                ::impex::IntoImpex::into_explicit(x),
                ::impex::IntoImpex::into_explicit(y),
            ),
        };
    }
}

#[derive(PartialEq, Eq, serde::Deserialize, serde::Serialize)]
enum ExplicitSubConfigVariant {
    //#[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
    Foo(::impex::MaybeExplicit<String>),
    //#[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
    Bar(::impex::MaybeExplicit<String>, ::impex::MaybeExplicit<i32>),
}

impl Default for ExplicitSubConfigVariant {
    fn default() -> Self {
        let c = SubConfig::default();
        match c {
            SubConfig::Foo(x) => Self::Foo(::impex::IntoImpex::into_implicit(x)),
            SubConfig::Bar(x1, x2) => Self::Bar(
                ::impex::IntoImpex::into_implicit(x1),
                ::impex::IntoImpex::into_implicit(x2),
            ),
        }
    }
}
