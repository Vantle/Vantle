use reference::Reference;

#[must_use]
pub fn ordinal() -> Reference {
    Reference("data-ordinal")
}

#[must_use]
pub fn capacity() -> Reference {
    Reference("data-capacity")
}

#[must_use]
pub fn paged() -> Reference {
    Reference("data-paged")
}

#[must_use]
pub fn bound() -> Reference {
    Reference("data-scroll-bound")
}
