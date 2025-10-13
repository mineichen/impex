#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct ImpexPrimitiveValue<T> {
    value: T,
    is_explicit: bool,
}

impl<T> ImpexPrimitiveValue<T> {
    pub fn make_defined(&mut self) -> &mut T {
        self.is_explicit = true;
        &mut self.value
    }
}

impl<T> std::ops::Deref for ImpexPrimitiveValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
#[cfg(feature = "serde")]
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

#[cfg(feature = "serde")]
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

impl<T: Default> Default for ImpexPrimitiveValue<T> {
    fn default() -> Self {
        Self {
            value: Default::default(),
            is_explicit: false,
        }
    }
}

pub trait IntoImpex: Sized {
    type Impex: Impex;

    fn into_impex(self, is_explicit: bool) -> Self::Impex;
    fn into_implicit(self) -> Self::Impex {
        self.into_impex(false)
    }
    fn into_explicit(self) -> Self::Impex {
        self.into_impex(true)
    }
}

pub trait Impex {
    type Value;

    fn is_explicit(&self) -> bool;
    fn is_implicit(&self) -> bool {
        !self.is_explicit()
    }
    fn into_value(self) -> Self::Value;
    fn set(&mut self, v: Self::Value);
}

impl<T: ImpexPrimitive> IntoImpex for T {
    type Impex = ImpexPrimitiveValue<Self>;

    fn into_impex(self, is_explicit: bool) -> Self::Impex {
        ImpexPrimitiveValue {
            value: self,
            is_explicit,
        }
    }
}

impl<T: ImpexPrimitive> Impex for ImpexPrimitiveValue<T> {
    type Value = T;

    fn is_explicit(&self) -> bool {
        self.is_explicit
    }

    fn into_value(self) -> Self::Value {
        self.value
    }
    fn set(&mut self, v: Self::Value) {
        *self.make_defined() = v;
    }
}

pub trait ImpexPrimitive: Sized {}
impl ImpexPrimitive for String {}
impl ImpexPrimitive for u32 {}
impl ImpexPrimitive for i32 {}
