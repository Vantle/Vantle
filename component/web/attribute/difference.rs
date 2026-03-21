use reference::Reference;

#[must_use]
pub fn marker() -> Reference {
    Reference("data-difference")
}

#[must_use]
pub fn bound() -> Reference {
    Reference("data-difference-bound")
}

#[must_use]
pub fn hidden() -> Reference {
    Reference("data-difference-hidden")
}
