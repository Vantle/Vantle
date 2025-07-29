pub trait Linked {
    type Element;
    type Stream: IntoIterator<Item = Self::Element>;

    fn descendents(&self, from: &Self::Element) -> Option<Self::Stream>;
    fn predecessors(&self, from: &Self::Element) -> Option<Self::Stream>;
}
