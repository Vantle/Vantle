pub trait Identified {
    type Identity;
    type Stream;
    fn transitions(&self, label: &Self::Identity) -> Self::Stream;
}
