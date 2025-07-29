/// Trait for types that can provide values by their numeric aliases/IDs
pub trait Valued {
    type Value;
    type Identity;
    type Error;
    /// Get a value by its alias/ID
    fn value(&self, alias: Self::Identity) -> Result<&Self::Value, Self::Error>;
}

/// Trait for types that can provide aliases/IDs for values
pub trait Aliased {
    type Value;
    type Identity;
    type Error;
    /// Get an alias/ID for a value
    fn alias(&self, value: &Self::Value) -> Result<Self::Identity, Self::Error>;
}

pub trait Scaled {
    type Magnitude;
    fn scale(&self, basis: &Self::Magnitude) -> Self::Magnitude;
}

pub trait Queryable {
    fn subset(&self, basis: &Self) -> Option<&Self>;
    fn superset(&self, basis: &Self) -> Option<&Self>;
    fn joint(&self, basis: &Self) -> Option<&Self>;
    fn disjoint(&self, basis: &Self) -> Option<&Self>;
    fn isomorphic(&self, basis: &Self) -> Option<&Self>;
}

pub trait Translatable
where
    Self: Sized,
{
    fn join(&self, basis: &Self) -> Option<Self>;
    fn intersect(&self, basis: &Self) -> Option<Self>;
    fn diverge(&self, basis: &Self) -> Option<Self>;
}
