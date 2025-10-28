use std::{fmt::Debug, marker::PhantomData};

//use serde::de::DeserializeOwned;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct ImpexPrimitiveValue<T, TW> {
    value: T,
    is_explicit: bool,
    wrapper: PhantomData<TW>,
}

impl<T, TW> ImpexPrimitiveValue<T, TW> {
    pub fn make_explicit(&mut self) -> &mut T {
        self.is_explicit = true;
        &mut self.value
    }
}

impl<T, TW> std::ops::Deref for ImpexPrimitiveValue<T, TW> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
#[cfg(feature = "serde")]
impl<T: serde::Serialize, TW: WrapperSettings> serde::Serialize for ImpexPrimitiveValue<T, TW> {
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

#[cfg(feature = "serde")]
impl<'de, T: serde::de::DeserializeOwned, TW: WrapperSettings> serde::Deserialize<'de>
    for ImpexPrimitiveValue<T, TW>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(deserializer).map(|value| ImpexPrimitiveValue {
            value,
            is_explicit: true,
            wrapper: PhantomData,
        })
    }
}

impl<T: Default, TW> Default for ImpexPrimitiveValue<T, TW> {
    fn default() -> Self {
        Self {
            value: Default::default(),
            is_explicit: false,
            wrapper: PhantomData,
        }
    }
}

pub trait WrapperSettings: Sized + Default {
    type PrimitiveWrapper<T: ImpexPrimitive>: Impex<Self, Value = T>
        + serde::de::DeserializeOwned
        + PartialEq
        + Eq
        + serde::Serialize
        + Debug
        + Default;
    fn create_primitive<T: ImpexPrimitive>(
        value: T,
        is_explicit: bool,
    ) -> Self::PrimitiveWrapper<T>;
}
#[derive(PartialEq, Eq, Debug, Default)]
pub struct DefaultWrapperSettings;

#[cfg(feature = "serde")]
impl<'de> serde::de::Deserialize<'de> for DefaultWrapperSettings {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self)
    }
}

impl serde::Serialize for DefaultWrapperSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_none()
    }
}

impl WrapperSettings for DefaultWrapperSettings {
    type PrimitiveWrapper<T: ImpexPrimitive> = ImpexPrimitiveValue<T, Self>;

    fn create_primitive<T: ImpexPrimitive>(
        value: T,
        is_explicit: bool,
    ) -> Self::PrimitiveWrapper<T> {
        ImpexPrimitiveValue {
            value,
            is_explicit,
            wrapper: PhantomData,
        }
    }
}

pub trait IntoImpex<TW: WrapperSettings /* = DefaultWrapperSettings*/>: Sized {
    type Impex: Impex<TW, Value = Self>;

    fn into_impex(self, is_explicit: bool) -> Self::Impex;
    fn into_implicit(self) -> Self::Impex {
        self.into_impex(false)
    }
    fn into_explicit(self) -> Self::Impex {
        self.into_impex(true)
    }
}

pub trait Impex<TW: WrapperSettings /* = DefaultWrapperSettings*/> {
    type Value: IntoImpex<TW>;

    fn is_explicit(&self) -> bool;
    fn is_implicit(&self) -> bool {
        !self.is_explicit()
    }
    fn into_value(self) -> Self::Value;
    /// Sets all values explicitly
    fn set_impex(&mut self, v: Self::Value, is_explicit: bool);
    fn set_explicit(&mut self, v: Self::Value) {
        self.set_impex(v, true);
    }
    fn set_implicit(&mut self, v: Self::Value) {
        self.set_impex(v, true);
    }
}

impl<T: ImpexPrimitive, TW: WrapperSettings> IntoImpex<TW> for T {
    type Impex = TW::PrimitiveWrapper<Self>;

    fn into_impex(self, is_explicit: bool) -> Self::Impex {
        TW::create_primitive(self, is_explicit)
    }
}

impl<T: ImpexPrimitive, TW: WrapperSettings> Impex<TW> for ImpexPrimitiveValue<T, TW> {
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
    Sized + serde::de::DeserializeOwned + PartialEq + Eq + serde::Serialize + Debug + Default
{
}
impl ImpexPrimitive for String {}
impl ImpexPrimitive for u32 {}
impl ImpexPrimitive for i32 {}
impl ImpexPrimitive for i64 {}
