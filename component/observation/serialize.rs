use serde::Serialize;
use std::fmt;

pub struct Json<'a, T: ?Sized>(pub &'a T);

impl<T: Serialize + ?Sized> fmt::Display for Json<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_json::to_string(self.0) {
            Ok(s) => f.write_str(&s),
            Err(_) => f.write_str("null"),
        }
    }
}

impl<T: Serialize + ?Sized> fmt::Debug for Json<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[must_use]
#[inline]
pub fn json<T: Serialize + ?Sized>(value: &T) -> Json<'_, T> {
    Json(value)
}
