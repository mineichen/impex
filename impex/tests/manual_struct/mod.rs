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

/// A unit enum - all variants are unit variants (no fields)
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Default)]
pub enum UnionEnumConfig {
    #[default]
    Foo,
    Bar,
}

/// An enum with mixed variants - some unit, some with fields
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum MixedEnumConfig {
    /// Unit variant
    Empty,
    /// Named fields variant
    Named { value: String },
    /// Tuple variant
    Tuple(i32),
}

impl Default for MixedEnumConfig {
    fn default() -> Self {
        MixedEnumConfig::Empty
    }
}

/// A struct containing unit enum fields to test serialization behavior
pub struct StructWithUnitEnum {
    pub unit_enum: UnionEnumConfig,
    pub mixed_enum: MixedEnumConfig,
}

///
/// THE following will be auto generated
///

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct KeyStructConfigImpex<TW: ::impex::WrapperSettings = ::impex::DefaultWrapperSettings> {
    #[serde(skip_serializing_if = "::impex::Impex::<TW>::is_implicit")]
    pub num_cores: <u32 as ::impex::IntoImpex<TW>>::Impex,
    #[serde(skip_serializing_if = "::impex::Impex::<TW>::is_implicit")]
    pub num_threads: <Vec<u32> as ::impex::IntoImpex<TW>>::Impex,
    #[serde(skip_serializing_if = "::impex::Impex::<TW>::is_implicit")]
    pub enum_config: <EnumConfig as ::impex::IntoImpex<TW>>::Impex,
    #[serde(skip_serializing_if = "::impex::Impex::<TW>::is_implicit")]
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
        #[serde(skip_serializing_if = "::impex::Impex::<TW>::is_implicit")]
        //#[serde(default)]
        foo_value: <String as ::impex::IntoImpex<TW>>::Impex,
        #[serde(skip_serializing_if = "::impex::Impex::<TW>::is_implicit")]
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

// ============================================================================
// UnionEnumConfig Impex Implementation (pure unit enum)
// ============================================================================

/// Visibility marker for Foo variant - tracks explicit/implicit state
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnionEnumConfigFooVisibility {
    is_explicit: bool,
}

impl Default for UnionEnumConfigFooVisibility {
    fn default() -> Self {
        Self { is_explicit: false } // Default is implicit
    }
}

/// Visibility marker for Bar variant - tracks explicit/implicit state
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnionEnumConfigBarVisibility {
    is_explicit: bool,
}

impl Default for UnionEnumConfigBarVisibility {
    fn default() -> Self {
        Self { is_explicit: false } // Default is implicit
    }
}

/// Impex for unit enum - each variant uses a visibility tuple variant
#[derive(Debug, Clone)]
pub enum UnionEnumConfigImpex {
    Foo(UnionEnumConfigFooVisibility),
    Bar(UnionEnumConfigBarVisibility),
}

// Custom Serialize - serialize as just the variant name string
impl ::serde::Serialize for UnionEnumConfigImpex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        match self {
            Self::Foo(_) => serializer.serialize_str("Foo"),
            Self::Bar(_) => serializer.serialize_str("Bar"),
        }
    }
}

// Custom Deserialize - deserialize from variant name string
impl<'de> ::serde::Deserialize<'de> for UnionEnumConfigImpex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Foo" => Ok(Self::Foo(UnionEnumConfigFooVisibility {
                is_explicit: true,
            })),
            "Bar" => Ok(Self::Bar(UnionEnumConfigBarVisibility {
                is_explicit: true,
            })),
            _ => Err(::serde::de::Error::unknown_variant(&s, &["Foo", "Bar"])),
        }
    }
}

impl<TW: ::impex::WrapperSettings> ::impex::IntoImpex<TW> for UnionEnumConfig {
    type Impex = UnionEnumConfigImpex;

    fn into_impex(self, is_explicit: bool) -> Self::Impex {
        match self {
            UnionEnumConfig::Foo => {
                UnionEnumConfigImpex::Foo(UnionEnumConfigFooVisibility { is_explicit })
            }
            UnionEnumConfig::Bar => {
                UnionEnumConfigImpex::Bar(UnionEnumConfigBarVisibility { is_explicit })
            }
        }
    }
}

impl<TW: ::impex::WrapperSettings> ::impex::Impex<TW> for UnionEnumConfigImpex {
    type Value = UnionEnumConfig;

    fn is_explicit(&self) -> bool {
        match self {
            UnionEnumConfigImpex::Foo(v) => v.is_explicit,
            UnionEnumConfigImpex::Bar(v) => v.is_explicit,
        }
    }

    fn into_value(self) -> Self::Value {
        match self {
            UnionEnumConfigImpex::Foo(_) => UnionEnumConfig::Foo,
            UnionEnumConfigImpex::Bar(_) => UnionEnumConfig::Bar,
        }
    }

    fn set_impex(&mut self, v: Self::Value, is_explicit: bool) {
        *self = match v {
            UnionEnumConfig::Foo => {
                UnionEnumConfigImpex::Foo(UnionEnumConfigFooVisibility { is_explicit })
            }
            UnionEnumConfig::Bar => {
                UnionEnumConfigImpex::Bar(UnionEnumConfigBarVisibility { is_explicit })
            }
        };
    }
}

impl Default for UnionEnumConfigImpex {
    fn default() -> Self {
        match UnionEnumConfig::default() {
            // Default gives is_explicit = false (implicit)
            UnionEnumConfig::Foo => UnionEnumConfigImpex::Foo(Default::default()),
            UnionEnumConfig::Bar => UnionEnumConfigImpex::Bar(Default::default()),
        }
    }
}

impl PartialEq for UnionEnumConfigImpex {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (UnionEnumConfigImpex::Foo(a), UnionEnumConfigImpex::Foo(b)) => a == b,
            (UnionEnumConfigImpex::Bar(a), UnionEnumConfigImpex::Bar(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for UnionEnumConfigImpex {}

#[cfg(feature = "visitor")]
impl<T> ::impex::Visitor<T> for UnionEnumConfigImpex {
    fn visit(&mut self, _ctx: &mut T) {
        // Unit enum has no fields to visit
    }
}

// ============================================================================
// MixedEnumConfig Impex Implementation (enum with unit + non-unit variants)
// ============================================================================

/// Visibility marker for Empty variant - tracks explicit/implicit state
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MixedEnumConfigEmptyVisibility {
    is_explicit: bool,
}

impl Default for MixedEnumConfigEmptyVisibility {
    fn default() -> Self {
        Self { is_explicit: false } // Default is implicit
    }
}

impl serde::Serialize for MixedEnumConfigEmptyVisibility {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("Empty")
    }
}

impl<'de> serde::Deserialize<'de> for MixedEnumConfigEmptyVisibility {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self { is_explicit: true })
    }
}

/// Impex type for mixed enum - unit variants use visibility tuple,
/// non-unit variants track is_explicit through their fields.
#[derive(Debug, Clone)]
pub enum MixedEnumConfigImpex<TW: ::impex::WrapperSettings = ::impex::DefaultWrapperSettings> {
    /// Unit variant uses visibility struct
    Empty(MixedEnumConfigEmptyVisibility),
    /// Named fields variant
    Named {
        value: <String as ::impex::IntoImpex<TW>>::Impex,
    },
    /// Tuple variant
    Tuple(<i32 as ::impex::IntoImpex<TW>>::Impex),
}

// Custom Serialize - unit variants serialize as strings, others as normal enum variants
impl<TW: ::impex::WrapperSettings> ::serde::Serialize for MixedEnumConfigImpex<TW>
where
    <String as ::impex::IntoImpex<TW>>::Impex: ::serde::Serialize,
    <i32 as ::impex::IntoImpex<TW>>::Impex: ::serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        match self {
            // Unit variant serializes as just "Empty"
            Self::Empty(_) => serializer.serialize_str("Empty"),
            // Named variant serializes as {"Named": {"value": ...}}
            Self::Named { value } => {
                use ::serde::ser::SerializeStructVariant;
                let mut sv =
                    serializer.serialize_struct_variant("MixedEnumConfigImpex", 1, "Named", 1)?;
                sv.serialize_field("value", value)?;
                sv.end()
            }
            // Tuple variant serializes as {"Tuple": ...}
            Self::Tuple(x) => {
                serializer.serialize_newtype_variant("MixedEnumConfigImpex", 2, "Tuple", x)
            }
        }
    }
}

// Custom Deserialize - unit variants from strings, others from map
impl<'de, TW: ::impex::WrapperSettings> ::serde::Deserialize<'de> for MixedEnumConfigImpex<TW>
where
    <String as ::impex::IntoImpex<TW>>::Impex: ::serde::Deserialize<'de>,
    <i32 as ::impex::IntoImpex<TW>>::Impex: ::serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        use ::serde::de::{self, MapAccess, Visitor};
        use std::fmt;
        use std::marker::PhantomData;

        struct MixedEnumVisitor<TW>(PhantomData<TW>);

        impl<'de, TW: ::impex::WrapperSettings> Visitor<'de> for MixedEnumVisitor<TW>
        where
            <String as ::impex::IntoImpex<TW>>::Impex: ::serde::Deserialize<'de>,
            <i32 as ::impex::IntoImpex<TW>>::Impex: ::serde::Deserialize<'de>,
        {
            type Value = MixedEnumConfigImpex<TW>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string or map for MixedEnumConfig variant")
            }

            // Handle unit variant as string: "Empty"
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "Empty" => Ok(MixedEnumConfigImpex::Empty(
                        MixedEnumConfigEmptyVisibility { is_explicit: true },
                    )),
                    _ => Err(de::Error::unknown_variant(
                        value,
                        &["Empty", "Named", "Tuple"],
                    )),
                }
            }

            // Handle non-unit variants as map: {"Named": {...}} or {"Tuple": ...}
            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let key: String = map
                    .next_key()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let result = match key.as_str() {
                    "Empty" => {
                        // Handle {"Empty": ...} format - ignore inner value
                        let _: ::serde::de::IgnoredAny = map.next_value()?;
                        Ok(MixedEnumConfigImpex::Empty(
                            MixedEnumConfigEmptyVisibility { is_explicit: true },
                        ))
                    }
                    "Named" => {
                        #[derive(::serde::Deserialize)]
                        #[serde(bound = "")]
                        struct NamedFields<TW: ::impex::WrapperSettings> {
                            value: <String as ::impex::IntoImpex<TW>>::Impex,
                        }
                        let fields: NamedFields<TW> = map.next_value()?;
                        Ok(MixedEnumConfigImpex::Named {
                            value: fields.value,
                        })
                    }
                    "Tuple" => {
                        let value: <i32 as ::impex::IntoImpex<TW>>::Impex = map.next_value()?;
                        Ok(MixedEnumConfigImpex::Tuple(value))
                    }
                    _ => Err(de::Error::unknown_variant(
                        &key,
                        &["Empty", "Named", "Tuple"],
                    )),
                };
                // Ensure no extra keys
                if map.next_key::<String>()?.is_some() {
                    return Err(de::Error::custom("expected single variant key"));
                }
                result
            }
        }

        deserializer.deserialize_any(MixedEnumVisitor(PhantomData))
    }
}

impl<TW: ::impex::WrapperSettings> ::impex::IntoImpex<TW> for MixedEnumConfig {
    type Impex = MixedEnumConfigImpex<TW>;

    fn into_impex(self, is_explicit: bool) -> Self::Impex {
        match self {
            MixedEnumConfig::Empty => {
                MixedEnumConfigImpex::Empty(MixedEnumConfigEmptyVisibility { is_explicit })
            }
            MixedEnumConfig::Named { value } => MixedEnumConfigImpex::Named {
                value: ::impex::IntoImpex::<TW>::into_impex(value, is_explicit),
            },
            MixedEnumConfig::Tuple(x) => {
                MixedEnumConfigImpex::Tuple(::impex::IntoImpex::<TW>::into_impex(x, is_explicit))
            }
        }
    }
}

impl<TW: ::impex::WrapperSettings> ::impex::Impex<TW> for MixedEnumConfigImpex<TW> {
    type Value = MixedEnumConfig;

    fn is_explicit(&self) -> bool {
        match self {
            // Unit variant: check our visibility struct
            MixedEnumConfigImpex::Empty(v) => v.is_explicit,
            // Non-unit variants: check if any field is explicit
            MixedEnumConfigImpex::Named { value } => ::impex::Impex::<TW>::is_explicit(value),
            MixedEnumConfigImpex::Tuple(x) => ::impex::Impex::<TW>::is_explicit(x),
        }
    }

    fn into_value(self) -> Self::Value {
        match self {
            MixedEnumConfigImpex::Empty(_) => MixedEnumConfig::Empty,
            MixedEnumConfigImpex::Named { value } => MixedEnumConfig::Named {
                value: ::impex::Impex::<TW>::into_value(value),
            },
            MixedEnumConfigImpex::Tuple(x) => {
                MixedEnumConfig::Tuple(::impex::Impex::<TW>::into_value(x))
            }
        }
    }

    fn set_impex(&mut self, v: Self::Value, is_explicit: bool) {
        *self = match v {
            MixedEnumConfig::Empty => {
                MixedEnumConfigImpex::Empty(MixedEnumConfigEmptyVisibility { is_explicit })
            }
            MixedEnumConfig::Named { value } => MixedEnumConfigImpex::Named {
                value: ::impex::IntoImpex::<TW>::into_impex(value, is_explicit),
            },
            MixedEnumConfig::Tuple(x) => {
                MixedEnumConfigImpex::Tuple(::impex::IntoImpex::<TW>::into_impex(x, is_explicit))
            }
        };
    }
}

impl<TW: ::impex::WrapperSettings> Default for MixedEnumConfigImpex<TW> {
    fn default() -> Self {
        let c = MixedEnumConfig::default();
        match c {
            MixedEnumConfig::Empty => MixedEnumConfigImpex::Empty(Default::default()),
            MixedEnumConfig::Named { value } => MixedEnumConfigImpex::Named {
                value: ::impex::IntoImpex::<TW>::into_implicit(value),
            },
            MixedEnumConfig::Tuple(x) => {
                MixedEnumConfigImpex::Tuple(::impex::IntoImpex::<TW>::into_implicit(x))
            }
        }
    }
}

impl<TW: ::impex::WrapperSettings> PartialEq for MixedEnumConfigImpex<TW>
where
    <String as ::impex::IntoImpex<TW>>::Impex: PartialEq,
    <i32 as ::impex::IntoImpex<TW>>::Impex: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MixedEnumConfigImpex::Empty(a), MixedEnumConfigImpex::Empty(b)) => a == b,
            (
                MixedEnumConfigImpex::Named { value: a },
                MixedEnumConfigImpex::Named { value: b },
            ) => a == b,
            (MixedEnumConfigImpex::Tuple(a), MixedEnumConfigImpex::Tuple(b)) => a == b,
            _ => false,
        }
    }
}

impl<TW: ::impex::WrapperSettings> Eq for MixedEnumConfigImpex<TW>
where
    <String as ::impex::IntoImpex<TW>>::Impex: Eq,
    <i32 as ::impex::IntoImpex<TW>>::Impex: Eq,
{
}

#[cfg(feature = "visitor")]
impl<T, TW: ::impex::WrapperSettings> ::impex::Visitor<T> for MixedEnumConfigImpex<TW>
where
    <String as ::impex::IntoImpex<TW>>::Impex: ::impex::Visitor<T>,
    <i32 as ::impex::IntoImpex<TW>>::Impex: ::impex::Visitor<T>,
{
    fn visit(&mut self, ctx: &mut T) {
        match self {
            MixedEnumConfigImpex::Empty { .. } => {
                // Unit variant has no fields to visit
            }
            MixedEnumConfigImpex::Named { value } => {
                ::impex::Visitor::<T>::visit(value, ctx);
            }
            MixedEnumConfigImpex::Tuple(x) => {
                ::impex::Visitor::<T>::visit(x, ctx);
            }
        }
    }
}

// ============================================================================
// StructWithUnitEnum Impex Implementation
// ============================================================================

impl Default for StructWithUnitEnum {
    fn default() -> Self {
        Self {
            unit_enum: UnionEnumConfig::default(),
            mixed_enum: MixedEnumConfig::default(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(default)]
pub struct StructWithUnitEnumImpex<TW: ::impex::WrapperSettings = ::impex::DefaultWrapperSettings> {
    // Unit enum Impex uses inherent method since it doesn't have TW parameter
    #[serde(skip_serializing_if = "::impex::Impex::<TW>::is_implicit")]
    pub unit_enum: <UnionEnumConfig as ::impex::IntoImpex<TW>>::Impex,
    #[serde(skip_serializing_if = "::impex::Impex::<TW>::is_implicit")]
    pub mixed_enum: <MixedEnumConfig as ::impex::IntoImpex<TW>>::Impex,
}

impl<TW: ::impex::WrapperSettings> ::impex::IntoImpex<TW> for StructWithUnitEnum {
    type Impex = StructWithUnitEnumImpex<TW>;

    fn into_impex(self, is_explicit: bool) -> Self::Impex {
        StructWithUnitEnumImpex {
            unit_enum: ::impex::IntoImpex::<TW>::into_impex(self.unit_enum, is_explicit),
            mixed_enum: ::impex::IntoImpex::<TW>::into_impex(self.mixed_enum, is_explicit),
        }
    }
}

impl<TW: ::impex::WrapperSettings> ::impex::Impex<TW> for StructWithUnitEnumImpex<TW> {
    type Value = StructWithUnitEnum;

    fn is_explicit(&self) -> bool {
        ::impex::Impex::<TW>::is_explicit(&self.unit_enum)
            || ::impex::Impex::<TW>::is_explicit(&self.mixed_enum)
    }

    fn into_value(self) -> Self::Value {
        StructWithUnitEnum {
            unit_enum: ::impex::Impex::<TW>::into_value(self.unit_enum),
            mixed_enum: ::impex::Impex::<TW>::into_value(self.mixed_enum),
        }
    }

    fn set_impex(&mut self, v: Self::Value, is_explicit: bool) {
        ::impex::Impex::<TW>::set_impex(&mut self.unit_enum, v.unit_enum, is_explicit);
        ::impex::Impex::<TW>::set_impex(&mut self.mixed_enum, v.mixed_enum, is_explicit);
    }
}

impl<TW: ::impex::WrapperSettings> Default for StructWithUnitEnumImpex<TW> {
    fn default() -> Self {
        let x = StructWithUnitEnum::default();
        Self {
            unit_enum: ::impex::IntoImpex::<TW>::into_implicit(x.unit_enum),
            mixed_enum: ::impex::IntoImpex::<TW>::into_implicit(x.mixed_enum),
        }
    }
}

impl<TW: ::impex::WrapperSettings> PartialEq for StructWithUnitEnumImpex<TW>
where
    <UnionEnumConfig as ::impex::IntoImpex<TW>>::Impex: PartialEq,
    <MixedEnumConfig as ::impex::IntoImpex<TW>>::Impex: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.unit_enum == other.unit_enum && self.mixed_enum == other.mixed_enum
    }
}

impl<TW: ::impex::WrapperSettings> Eq for StructWithUnitEnumImpex<TW>
where
    <UnionEnumConfig as ::impex::IntoImpex<TW>>::Impex: Eq,
    <MixedEnumConfig as ::impex::IntoImpex<TW>>::Impex: Eq,
{
}

#[cfg(feature = "visitor")]
impl<T, TW: ::impex::WrapperSettings> ::impex::Visitor<T> for StructWithUnitEnumImpex<TW>
where
    <UnionEnumConfig as ::impex::IntoImpex<TW>>::Impex: ::impex::Visitor<T>,
    <MixedEnumConfig as ::impex::IntoImpex<TW>>::Impex: ::impex::Visitor<T>,
{
    fn visit(&mut self, ctx: &mut T) {
        ::impex::Visitor::<T>::visit(&mut self.unit_enum, ctx);
        ::impex::Visitor::<T>::visit(&mut self.mixed_enum, ctx);
    }
}
