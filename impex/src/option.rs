use crate::{Impex, IntoImpex, WrapperSettings};

/// Impex wrapper for Option that tracks explicit/implicit state for None values.
/// - `Some(value)`: explicit/implicit determined by inner value
/// - `None`: can be explicit (JSON had `null`) or implicit (field was missing)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionImpex<T> {
    Some(T),
    /// None with bool tracking if it was explicitly set to null
    None(bool),
}

impl<T> OptionImpex<T> {
    pub fn explicit_none() -> Self {
        Self::None(true)
    }

    pub fn implicit_none() -> Self {
        Self::None(false)
    }

    pub fn is_some(&self) -> bool {
        matches!(self, Self::Some(_))
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Self::None(_))
    }

    pub fn as_ref(&self) -> Option<&T> {
        match self {
            OptionImpex::Some(x) => Some(x),
            OptionImpex::None(_) => None,
        }
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
        match self {
            OptionImpex::Some(x) => Some(x),
            OptionImpex::None(_) => None,
        }
    }
}

/// Inherent is_explicit/is_implicit for DefaultWrapperSettings (most common case)
impl<T: Impex<crate::DefaultWrapperSettings>> OptionImpex<T> {
    pub fn is_explicit(&self) -> bool {
        match self {
            OptionImpex::Some(value) => value.is_explicit(),
            OptionImpex::None(is_explicit) => *is_explicit,
        }
    }

    pub fn is_implicit(&self) -> bool {
        !self.is_explicit()
    }
}

impl<T: Default> Default for OptionImpex<T> {
    fn default() -> Self {
        Self::implicit_none()
    }
}

impl<TW: WrapperSettings, T: IntoImpex<TW>> IntoImpex<TW> for Option<T> {
    type Impex = OptionImpex<T::Impex>;

    fn into_impex(self, is_explicit: bool) -> Self::Impex {
        match self {
            Some(value) => OptionImpex::Some(value.into_impex(is_explicit)),
            None => OptionImpex::None(is_explicit),
        }
    }
}

impl<TW: WrapperSettings, T: Impex<TW>> Impex<TW> for OptionImpex<T>
where
    T::Value: IntoImpex<TW, Impex = T>,
{
    type Value = Option<T::Value>;

    fn is_explicit(&self) -> bool {
        match self {
            OptionImpex::Some(value) => value.is_explicit(),
            OptionImpex::None(is_explicit) => *is_explicit,
        }
    }

    fn into_value(self) -> Self::Value {
        match self {
            OptionImpex::Some(value) => Some(value.into_value()),
            OptionImpex::None(_) => None,
        }
    }

    fn set_impex(&mut self, v: Self::Value, is_explicit: bool) {
        *self = match v {
            Some(value) => OptionImpex::Some(value.into_impex(is_explicit)),
            None => OptionImpex::None(is_explicit),
        };
    }
}

impl<T: serde::Serialize> serde::Serialize for OptionImpex<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            OptionImpex::Some(value) => value.serialize(serializer),
            OptionImpex::None(_) => serializer.serialize_none(),
        }
    }
}

impl<'de, T: serde::de::Deserialize<'de>> serde::Deserialize<'de> for OptionImpex<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // When deserializing, if the field is present (even as null), it's explicit
        let value = Option::<T>::deserialize(deserializer)?;
        Ok(match value {
            Some(v) => OptionImpex::Some(v),
            None => OptionImpex::explicit_none(), // Deserialized null = explicit none
        })
    }
}

#[cfg(feature = "visitor")]
impl<T, U> crate::Visitor<T> for OptionImpex<U>
where
    U: crate::Visitor<T>,
{
    fn visit(&mut self, ctx: &mut T) {
        if let OptionImpex::Some(inner) = self {
            inner.visit(ctx);
        }
    }
}
