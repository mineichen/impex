use crate::{Impex, IntoImpex, WrapperSettings};

impl<TW: WrapperSettings, T: IntoImpex<TW>> IntoImpex<TW> for Vec<T> {
    type Impex = Vec<T::Impex>;

    fn into_impex(self, is_explicit: bool) -> Self::Impex {
        self.into_iter()
            .map(|x| x.into_impex(is_explicit))
            .collect()
    }
}

impl<TW: WrapperSettings, T: Impex<TW>> Impex<TW> for Vec<T>
where
    T::Value: IntoImpex<TW, Impex = T>,
{
    type Value = Vec<T::Value>;

    fn is_explicit(&self) -> bool {
        self.iter().any(|x| x.is_explicit())
    }

    fn into_value(self) -> Self::Value {
        self.into_iter().map(|x| x.into_value()).collect()
    }

    fn set_impex(&mut self, v: Self::Value, is_explicit: bool) {
        self.clear();
        self.extend(v.into_iter().map(|x| x.into_impex(is_explicit)));
    }
}

#[cfg(feature = "visitor")]
impl<T, U> crate::Visitor<T> for Vec<U>
where
    U: crate::Visitor<T>,
{
    fn visit(&mut self, ctx: &mut T) {
        self.iter_mut().for_each(|x| x.visit(ctx));
    }
}
