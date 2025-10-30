use std::num::NonZeroU8;

use impex::{Impex, ImpexPrimitive, WrapperSettings};

use crate::generated_struct::KeyStructConfigImpex;
// Switch between manual and generated implementations:
//use crate::manual_struct::KeyStructConfigImpex;

#[allow(unused)]
mod generated_struct;
#[allow(unused)]
mod manual_struct;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct MyPrimitiveValue<T> {
    is_explicit: bool,
    variable_name: Option<(NonZeroU8, [u8; 30])>,
    value: T,
}

impl<T> MyPrimitiveValue<T> {
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
}

impl<T> std::ops::Deref for MyPrimitiveValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: serde::Serialize> serde::Serialize for MyPrimitiveValue<T> {
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

impl<'de, T: serde::de::DeserializeOwned> serde::Deserialize<'de> for MyPrimitiveValue<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(deserializer).map(|value| MyPrimitiveValue {
            value,
            is_explicit: true,
            variable_name: None,
        })
    }
}

impl<T: ImpexPrimitive, TW: WrapperSettings> Impex<TW> for MyPrimitiveValue<T> {
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

#[derive(PartialEq, Eq, Debug, Default)]
pub struct MyWrapperSettings;

impl<'de> serde::de::Deserialize<'de> for MyWrapperSettings {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self)
    }
}

impl serde::Serialize for MyWrapperSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_none()
    }
}

impl WrapperSettings for MyWrapperSettings {
    type PrimitiveWrapper<T: ImpexPrimitive> = MyPrimitiveValue<T>;

    fn create_primitive<T: ImpexPrimitive>(
        value: T,
        is_explicit: bool,
    ) -> Self::PrimitiveWrapper<T> {
        MyPrimitiveValue {
            value,
            is_explicit,
            variable_name: None,
        }
    }
}

#[test]
fn custom_strategy() {
    let text = r#"{"num_cores":43}"#;
    let config: KeyStructConfigImpex<MyWrapperSettings> = serde_json::from_str(text).unwrap();
    assert!(config.num_cores.variable_name.is_none());
    assert!(config.num_cores.is_explicit());
    assert_eq!(*config.num_cores, 43);
}
