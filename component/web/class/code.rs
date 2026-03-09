use reference::Reference;

#[must_use]
pub fn block() -> Reference {
    Reference("code-block")
}

#[must_use]
pub fn toolbar() -> Reference {
    Reference("code-toolbar")
}

#[must_use]
pub fn source() -> Reference {
    Reference("code-source")
}
