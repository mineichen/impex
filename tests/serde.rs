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
    //obj.sub.bar.set("Custom".into());
    *obj.sub.bar.make_defined() = "Custom".into();
    assert_eq!("Custom", *obj.sub.bar);
    assert_eq!("Foo", *obj.sub.foo);
    assert_eq!(
        r#"{"num_cores":3,"sub":{"bar":"Custom"}}"#,
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
struct SubConfig {
    foo: String,
    bar: String,
}

impl Default for SubConfig {
    fn default() -> Self {
        Self {
            foo: "Foo".into(),
            bar: "Bar".into(),
        }
    }
}

///
/// THE following will be auto generated
///
impl ::impex::IntoImpex for Config {
    type Explicit = ExplicitConfig;

    fn into_implicit(self) -> Self::Explicit {
        Self::Explicit {
            num_cores: ::impex::IntoImpex::into_implicit(self.num_cores),
            num_threads: ::impex::IntoImpex::into_implicit(self.num_threads),
            sub: ::impex::IntoImpex::into_implicit(self.sub),
        }
    }
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

#[derive(PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(default)]
struct ExplicitConfig {
    #[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
    num_cores: <u32 as ::impex::IntoImpex>::Explicit,
    #[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
    num_threads: <u32 as ::impex::IntoImpex>::Explicit,
    #[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
    sub: <SubConfig as ::impex::IntoImpex>::Explicit,
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

    fn into_implicit(self) -> Self::Explicit {
        Self::Explicit {
            foo: ::impex::IntoImpex::into_implicit(self.foo),
            bar: ::impex::IntoImpex::into_implicit(self.bar),
        }
    }
}
impl ::impex::Impex for ExplicitSubConfig {
    type Value = SubConfig;

    fn is_explicit(&self) -> bool {
        ::impex::Impex::is_explicit(&self.foo) || ::impex::Impex::is_explicit(&self.bar)
    }

    fn into_value(self) -> Self::Value {
        Self::Value {
            foo: ::impex::Impex::into_value(self.bar),
            bar: ::impex::Impex::into_value(self.foo),
        }
    }

    fn set(&mut self, v: Self::Value) {
        ::impex::Impex::set(&mut self.foo, v.foo);
        ::impex::Impex::set(&mut self.bar, v.bar);
    }
}

#[derive(PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(default)]
struct ExplicitSubConfig {
    #[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
    foo: ::impex::MaybeExplicit<String>,
    #[serde(skip_serializing_if = "::impex::Impex::is_implicit")]
    bar: ::impex::MaybeExplicit<String>,
}

impl Default for ExplicitSubConfig {
    fn default() -> Self {
        let c = SubConfig::default();
        Self {
            foo: ::impex::IntoImpex::into_implicit(c.foo),
            bar: ::impex::IntoImpex::into_implicit(c.bar),
        }
    }
}
