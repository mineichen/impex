use serde::de::DeserializeOwned;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct MaybeExplicit<T> {
    value: T,
    is_explicit: bool,
}

impl<T> MaybeExplicit<T> {
    pub fn make_defined(&mut self) -> &mut T {
        self.is_explicit = true;
        &mut self.value
    }

    pub fn set(&mut self, v: T) {
        *self.make_defined() = v;
    }
}

impl<T> std::ops::Deref for MaybeExplicit<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<T: serde::Serialize> serde::Serialize for MaybeExplicit<T> {
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
impl<'de, T: DeserializeOwned> serde::Deserialize<'de> for MaybeExplicit<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(deserializer).map(|value| MaybeExplicit {
            value,
            is_explicit: true,
        })
    }
}

impl<T: Default> Default for MaybeExplicit<T> {
    fn default() -> Self {
        Self {
            value: Default::default(),
            is_explicit: false,
        }
    }
}

pub trait IntoImpex {
    type Explicit: Impex;
    fn into_implicit(self) -> Self::Explicit;
}

pub trait Impex {
    type Value;

    fn is_explicit(&self) -> bool;
    fn is_implicit(&self) -> bool {
        !self.is_explicit()
    }
    fn into_value(self) -> Self::Value;
}

impl IntoImpex for u32 {
    type Explicit = MaybeExplicit<Self>;

    fn into_implicit(self) -> Self::Explicit {
        MaybeExplicit {
            value: self,
            is_explicit: false,
        }
    }
}

impl Impex for MaybeExplicit<u32> {
    type Value = u32;

    fn is_explicit(&self) -> bool {
        self.is_explicit
    }

    fn into_value(self) -> Self::Value {
        self.value
    }
}
impl IntoImpex for String {
    type Explicit = MaybeExplicit<Self>;

    fn into_implicit(self) -> Self::Explicit {
        MaybeExplicit {
            value: self,
            is_explicit: false,
        }
    }
}
impl Impex for MaybeExplicit<String> {
    type Value = String;

    fn is_explicit(&self) -> bool {
        self.is_explicit
    }

    fn into_value(self) -> Self::Value {
        self.value
    }
}
