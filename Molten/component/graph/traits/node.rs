pub trait Aliased {
    type Value;
    type Identity;
    type Error;
    fn value(&self, alias: Self::Identity) -> Result<&Self::Value, Self::Error>;
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
