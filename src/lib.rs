use serde::de::DeserializeOwned;

#[derive(PartialEq, Eq)]
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

    pub fn into_value(self) -> T {
        self.value
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

pub trait IntoExplicit {
    type Value: Explicit;
    fn into_implicit(self) -> Self::Value;
}

pub trait Explicit {
    fn is_explicit(&self) -> bool;
    fn is_implicit(&self) -> bool {
        !self.is_explicit()
    }
}

impl IntoExplicit for u32 {
    type Value = MaybeExplicit<Self>;

    fn into_implicit(self) -> Self::Value {
        MaybeExplicit {
            value: self,
            is_explicit: false,
        }
    }
}

impl Explicit for MaybeExplicit<u32> {
    fn is_explicit(&self) -> bool {
        self.is_explicit
    }
}
impl IntoExplicit for String {
    type Value = MaybeExplicit<Self>;

    fn into_implicit(self) -> Self::Value {
        MaybeExplicit {
            value: self,
            is_explicit: false,
        }
    }
}
impl Explicit for MaybeExplicit<String> {
    fn is_explicit(&self) -> bool {
        self.is_explicit
    }
}
