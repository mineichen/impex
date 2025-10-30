use std::fmt::Debug;

use crate::Impex;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct ImpexPrimitiveValue<T> {
    value: T,
    is_explicit: bool,
}

impl<T> ImpexPrimitiveValue<T> {
    pub(crate) fn new(value: T, is_explicit: bool) -> Self {
        Self { value, is_explicit }
    }

    pub fn make_explicit(&mut self) -> &mut T {
        self.is_explicit = true;
        &mut self.value
    }

    pub fn is_explicit(&self) -> bool {
        self.is_explicit
    }

    pub fn is_implicit(&self) -> bool {
        !self.is_explicit
    }

    pub fn set_explicit(&mut self, value: T) {
        self.is_explicit = true;
        self.value = value;
    }

    pub fn into_value(self) -> T {
        self.value
    }
}

impl<T> std::ops::Deref for ImpexPrimitiveValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: serde::Serialize> serde::Serialize for ImpexPrimitiveValue<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.is_explicit {
            self.value.serialize(serializer)
        } else {
            serializer.serialize_none()
        }
    }
}

impl<'de, T: serde::de::DeserializeOwned> serde::Deserialize<'de> for ImpexPrimitiveValue<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(deserializer).map(|value| ImpexPrimitiveValue {
            value,
            is_explicit: true,
        })
    }
}

impl<T: ImpexPrimitive, TW> Impex<TW> for ImpexPrimitiveValue<T> {
    type Value = T;

    fn is_explicit(&self) -> bool {
        self.is_explicit
    }

    fn into_value(self) -> Self::Value {
        self.value
    }
    fn set_impex(&mut self, v: Self::Value, is_explicit: bool) {
        self.is_explicit = is_explicit;
        self.value = v;
    }
}

pub trait ImpexPrimitive:
    Sized + serde::de::DeserializeOwned + PartialEq + Eq + serde::Serialize + Debug + Default + Clone
{
}

impl ImpexPrimitive for String {}
impl ImpexPrimitive for u32 {}
impl ImpexPrimitive for i32 {}
impl ImpexPrimitive for i64 {}

///
/// Wraps a normal value so it can be turned into a impex, even if the type doesn't implement IntoImpex
/// This should be a field attribute #[impex(primitive)] in the macro
///
#[derive(serde::Deserialize, PartialEq, Eq, serde::Serialize, std::fmt::Debug, Default, Clone)]
pub struct PrimitiveWrapper<T>(pub T);
impl<T: serde::de::DeserializeOwned + PartialEq + Eq + serde::Serialize + Debug + Default + Clone>
    ImpexPrimitive for PrimitiveWrapper<T>
{
}
