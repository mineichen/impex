use std::fmt::Debug;

mod primitive;

pub use primitive::*;

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
    type PrimitiveWrapper<T: ImpexPrimitive> = ImpexPrimitiveValue<T>;

    fn create_primitive<T: ImpexPrimitive>(
        value: T,
        is_explicit: bool,
    ) -> Self::PrimitiveWrapper<T> {
        ImpexPrimitiveValue::new(value, is_explicit)
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

pub trait Impex<TW /* = DefaultWrapperSettings*/> {
    type Value;

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
