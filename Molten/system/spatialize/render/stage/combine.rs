use field::Field;

pub trait Combine: Into<Field> {
    fn union(self, other: impl Into<Field>) -> Field
    where
        Self: Sized,
    {
        Field::Union {
            left: Box::new(self.into()),
            right: Box::new(other.into()),
        }
    }

    fn subtract(self, other: impl Into<Field>) -> Field
    where
        Self: Sized,
    {
        Field::Subtract {
            left: Box::new(self.into()),
            right: Box::new(other.into()),
        }
    }

    fn intersect(self, other: impl Into<Field>) -> Field
    where
        Self: Sized,
    {
        Field::Intersect {
            left: Box::new(self.into()),
            right: Box::new(other.into()),
        }
    }
}

impl<T: Into<Field>> Combine for T {}
