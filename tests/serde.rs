#[test]
fn serialize_with_defaults() {
    use meta_data::Explicit;

    let text = r#"{"num_cores":3}"#;
    let mut obj = serde_json::from_str::<ExplicitConfig>(text).unwrap();
    assert_eq!(3, *obj.num_cores);
    assert_eq!(42, *obj.num_threads);
    assert!(obj.num_threads.is_explicit());
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
impl ::meta_data::IntoExplicit for Config {
    type Value = ExplicitConfig;

    fn into_implicit(self) -> Self::Value {
        Self::Value {
            num_cores: ::meta_data::IntoExplicit::into_implicit(self.num_cores),
            num_threads: ::meta_data::IntoExplicit::into_implicit(self.num_threads),
            sub: ::meta_data::IntoExplicit::into_implicit(self.sub),
        }
    }
}

impl ::meta_data::Explicit for ExplicitConfig {
    fn is_explicit(&self) -> bool {
        ::meta_data::Explicit::is_explicit(&self.num_cores)
            || ::meta_data::Explicit::is_explicit(&self.num_threads)
            || ::meta_data::Explicit::is_explicit(&self.sub)
    }
}

#[derive(PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(default)]
struct ExplicitConfig {
    #[serde(skip_serializing_if = "::meta_data::Explicit::is_implicit")]
    num_cores: <u32 as ::meta_data::IntoExplicit>::Value,
    #[serde(skip_serializing_if = "meta_data::Explicit::is_implicit")]
    num_threads: <u32 as ::meta_data::IntoExplicit>::Value,
    #[serde(skip_serializing_if = "meta_data::Explicit::is_implicit")]
    sub: <SubConfig as ::meta_data::IntoExplicit>::Value,
}

impl Default for ExplicitConfig {
    fn default() -> Self {
        let c = Config::default();
        Self {
            num_cores: ::meta_data::IntoExplicit::into_implicit(c.num_cores),
            num_threads: ::meta_data::IntoExplicit::into_implicit(c.num_threads),
            sub: ::meta_data::IntoExplicit::into_implicit(c.sub),
        }
    }
}

impl ::meta_data::IntoExplicit for SubConfig {
    type Value = ExplicitSubConfig;

    fn into_implicit(self) -> Self::Value {
        Self::Value {
            foo: ::meta_data::IntoExplicit::into_implicit(self.foo),
            bar: ::meta_data::IntoExplicit::into_implicit(self.bar),
        }
    }
}
impl ::meta_data::Explicit for ExplicitSubConfig {
    fn is_explicit(&self) -> bool {
        ::meta_data::Explicit::is_explicit(&self.foo)
            || ::meta_data::Explicit::is_explicit(&self.bar)
    }
}

#[derive(PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(default)]
struct ExplicitSubConfig {
    #[serde(skip_serializing_if = "::meta_data::Explicit::is_implicit")]
    foo: ::meta_data::MaybeExplicit<String>,
    #[serde(skip_serializing_if = "::meta_data::Explicit::is_implicit")]
    bar: ::meta_data::MaybeExplicit<String>,
}

impl Default for ExplicitSubConfig {
    fn default() -> Self {
        let c = SubConfig::default();
        Self {
            foo: ::meta_data::IntoExplicit::into_implicit(c.foo),
            bar: ::meta_data::IntoExplicit::into_implicit(c.bar),
        }
    }
}
