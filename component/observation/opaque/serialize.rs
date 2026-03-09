use std::fmt;
use std::marker::PhantomData;

pub struct Json<'a, T: ?Sized>(PhantomData<&'a T>);

impl<T: ?Sized> fmt::Display for Json<'_, T> {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl<T: ?Sized> fmt::Debug for Json<'_, T> {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

#[must_use]
#[inline]
pub fn json<T: ?Sized>(_: &T) -> Json<'_, T> {
    Json(PhantomData)
}
