#[test]
fn serialize_with_defaults() {
    let text = r#"{"num_cores":3}"#;
    let obj = serde_json::from_str::<LoadedConfig>(text).unwrap();
    assert_eq!(3, *obj.num_cores);
    assert_eq!(42, *obj.num_threads);
    assert_eq!(text, serde_json::to_string(&obj).unwrap().as_str());
}

#[derive(serde::Deserialize, serde::Serialize)]
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

#[derive(PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(default)]
struct LoadedConfig {
    #[serde(skip_serializing_if = "<u32 as ::meta_data::IntoExplicit>::is_implicit")]
    num_cores: <u32 as ::meta_data::IntoExplicit>::Value,
    #[serde(skip_serializing_if = "<u32 as ::meta_data::IntoExplicit>::is_implicit")]
    num_threads: <u32 as ::meta_data::IntoExplicit>::Value,
    #[serde(skip_serializing_if = "<SubConfig as ::meta_data::IntoExplicit>::is_implicit")]
    sub: <SubConfig as ::meta_data::IntoExplicit>::Value,
}

impl Default for LoadedConfig {
    fn default() -> Self {
        let c = Config::default();
        Self {
            num_cores: ::meta_data::IntoExplicit::into_implicit(c.num_cores),
            num_threads: ::meta_data::IntoExplicit::into_implicit(c.num_threads),
            sub: ::meta_data::IntoExplicit::into_implicit(c.sub),
        }
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
struct SubConfig {
    foo: String,
    bar: String,
}

#[derive(Default, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(default)]
struct LoadedSubConfig {
    #[serde(skip_serializing_if = "::meta_data::MaybeExplicit::is_implicit")]
    foo: ::meta_data::MaybeExplicit<String>,
    #[serde(skip_serializing_if = "::meta_data::MaybeExplicit::is_implicit")]
    bar: ::meta_data::MaybeExplicit<String>,
}

impl ::meta_data::IntoExplicit for Config {
    type Value = LoadedConfig;

    fn into_implicit(self) -> Self::Value {
        Self::Value {
            num_cores: ::meta_data::IntoExplicit::into_implicit(self.num_cores),
            num_threads: ::meta_data::IntoExplicit::into_implicit(self.num_threads),
            sub: ::meta_data::IntoExplicit::into_implicit(self.sub),
        }
    }

    fn is_explicit(value: &Self::Value) -> bool {
        <u32 as ::meta_data::IntoExplicit>::is_explicit(&value.num_cores)
            || <u32 as ::meta_data::IntoExplicit>::is_explicit(&value.num_threads)
            || <SubConfig as ::meta_data::IntoExplicit>::is_explicit(&value.sub)
    }
}
impl ::meta_data::IntoExplicit for SubConfig {
    type Value = LoadedSubConfig;

    fn into_implicit(self) -> Self::Value {
        Self::Value {
            foo: ::meta_data::IntoExplicit::into_implicit(self.foo),
            bar: ::meta_data::IntoExplicit::into_implicit(self.bar),
        }
    }

    fn is_explicit(value: &Self::Value) -> bool {
        <String as ::meta_data::IntoExplicit>::is_explicit(&value.foo)
            || <String as ::meta_data::IntoExplicit>::is_explicit(&value.bar)
    }
}
