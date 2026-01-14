use crate::{Impex, IntoImpex, WrapperSettings};

impl<TW: WrapperSettings, T: IntoImpex<TW>, const SIZE: usize> IntoImpex<TW> for [T; SIZE] {
    type Impex = [T::Impex; SIZE];

    fn into_impex(self, is_explicit: bool) -> Self::Impex {
        self.map(|v| v.into_impex(is_explicit))
    }
}

impl<TW, T: Impex<TW>, const SIZE: usize> Impex<TW> for [T; SIZE] {
    type Value = [T::Value; SIZE];

    fn is_explicit(&self) -> bool {
        self.iter().any(|x| x.is_explicit())
    }

    fn into_value(self) -> Self::Value {
        self.map(Impex::into_value)
    }

    fn set_impex(&mut self, v: Self::Value, is_explicit: bool) {
        self.iter_mut().zip(v).for_each(|(target, value)| {
            target.set_impex(value, is_explicit);
        });
    }
}

#[cfg(feature = "visitor")]
impl<T, U, const SIZE: usize> crate::Visitor<T> for [U; SIZE]
where
    U: crate::Visitor<T>,
{
    fn visit(&mut self, ctx: &mut T) {
        self.iter_mut().for_each(|x| x.visit(ctx));
    }
}
