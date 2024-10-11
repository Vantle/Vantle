pub trait Identified {
    type Value;
    type Identity;
    type Error;
    fn value(&self, label: Self::Identity) -> Result<&Self::Value, Self::Error>;
    fn label(&self, value: &Self::Value) -> Result<Self::Identity, Self::Error>;
}
