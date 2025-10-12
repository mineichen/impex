#[test]
fn serialize_with_defaults() {
    use impex::Explicit;

    let text = r#"{"num_cores":3}"#;
    let mut obj = serde_json::from_str::<ExplicitConfig>(text).unwrap();
    assert_eq!(3, *obj.num_cores);
    assert_eq!(42, *obj.num_threads);
    assert!(obj.num_cores.is_explicit());
    assert!(obj.num_threads.is_implicit());
    assert_eq!(text, serde_json::to_string(&obj).unwrap().as_str());
    //obj.sub.bar.set("Custom".into());
    *obj.sub.bar.make_defined() = "Custom".into();
    assert_eq!(
        r#"{"num_cores":3,"sub":{"bar":"Custom"}}"#,
        serde_json::to_string(&obj).unwrap().as_str()
    );
}

#[derive(serde::Deserialize, serde::Serialize /*, Explicit*/)]
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

#[derive(Default, serde::Deserialize, serde::Serialize /*, Explicit*/)]
struct SubConfig {
    foo: String,
    bar: String,
}

///
/// THE following will be auto generated
///
impl ::impex::IntoExplicit for Config {
    type Explicit = ExplicitConfig;

    fn into_implicit(self) -> Self::Explicit {
        Self::Explicit {
            num_cores: ::impex::IntoExplicit::into_implicit(self.num_cores),
            num_threads: ::impex::IntoExplicit::into_implicit(self.num_threads),
            sub: ::impex::IntoExplicit::into_implicit(self.sub),
        }
    }
}

impl ::impex::Explicit for ExplicitConfig {
    type Value = Config;
    fn is_explicit(&self) -> bool {
        ::impex::Explicit::is_explicit(&self.num_cores)
            || ::impex::Explicit::is_explicit(&self.num_threads)
            || ::impex::Explicit::is_explicit(&self.sub)
    }

    fn into_value(self) -> Self::Value {
        Self::Value {
            num_cores: ::impex::Explicit::into_value(self.num_cores),
            num_threads: ::impex::Explicit::into_value(self.num_threads),
            sub: ::impex::Explicit::into_value(self.sub),
        }
    }
}

#[derive(PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(default)]
struct ExplicitConfig {
    #[serde(skip_serializing_if = "::impex::Explicit::is_implicit")]
    num_cores: <u32 as ::impex::IntoExplicit>::Explicit,
    #[serde(skip_serializing_if = "::impex::Explicit::is_implicit")]
    num_threads: <u32 as ::impex::IntoExplicit>::Explicit,
    #[serde(skip_serializing_if = "::impex::Explicit::is_implicit")]
    sub: <SubConfig as ::impex::IntoExplicit>::Explicit,
}

impl Default for ExplicitConfig {
    fn default() -> Self {
        let c = Config::default();
        Self {
            num_cores: ::impex::IntoExplicit::into_implicit(c.num_cores),
            num_threads: ::impex::IntoExplicit::into_implicit(c.num_threads),
            sub: ::impex::IntoExplicit::into_implicit(c.sub),
        }
    }
}

impl ::impex::IntoExplicit for SubConfig {
    type Explicit = ExplicitSubConfig;

    fn into_implicit(self) -> Self::Explicit {
        Self::Explicit {
            foo: ::impex::IntoExplicit::into_implicit(self.foo),
            bar: ::impex::IntoExplicit::into_implicit(self.bar),
        }
    }
}
impl ::impex::Explicit for ExplicitSubConfig {
    type Value = SubConfig;

    fn is_explicit(&self) -> bool {
        ::impex::Explicit::is_explicit(&self.foo) || ::impex::Explicit::is_explicit(&self.bar)
    }

    fn into_value(self) -> Self::Value {
        Self::Value {
            foo: ::impex::Explicit::into_value(self.bar),
            bar: ::impex::Explicit::into_value(self.foo),
        }
    }
}

#[derive(PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(default)]
struct ExplicitSubConfig {
    #[serde(skip_serializing_if = "::impex::Explicit::is_implicit")]
    foo: ::impex::MaybeExplicit<String>,
    #[serde(skip_serializing_if = "::impex::Explicit::is_implicit")]
    bar: ::impex::MaybeExplicit<String>,
}

impl Default for ExplicitSubConfig {
    fn default() -> Self {
        let c = SubConfig::default();
        Self {
            foo: ::impex::IntoExplicit::into_implicit(c.foo),
            bar: ::impex::IntoExplicit::into_implicit(c.bar),
        }
    }
}
