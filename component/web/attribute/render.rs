use reference::Reference;

#[must_use]
pub fn language() -> Reference {
    Reference("data-language")
}

#[must_use]
pub fn depth() -> Reference {
    Reference("data-depth")
}

#[must_use]
pub fn source() -> Reference {
    Reference("data-source")
}
