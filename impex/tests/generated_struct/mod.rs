// This module uses the #[derive(Impex)] macro to auto-generate
// the same code that is manually written in manual_struct/mod.rs

#[derive(impex::Impex)]
pub struct KeyStructConfig {
    pub num_cores: u32,
    pub num_threads: u32,
    pub enum_config: EnumConfig,
    pub tuple_struct_config: TupleStructConfig,
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

#[derive(impex::Impex)]
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

#[derive(Debug, PartialEq, impex::Impex)]
pub struct TupleStructConfig(pub i32, pub i64);

impl Default for TupleStructConfig {
    fn default() -> Self {
        Self(42, 43)
    }
}
