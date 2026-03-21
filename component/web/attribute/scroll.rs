use reference::Reference;

#[must_use]
pub fn scrolled() -> Reference {
    Reference("data-scrolled")
}

#[must_use]
pub fn animate() -> Reference {
    Reference("data-animate")
}

#[must_use]
pub fn visible() -> Reference {
    Reference("data-visible")
}

#[must_use]
pub fn shadow() -> Reference {
    Reference("data-shadow")
}
