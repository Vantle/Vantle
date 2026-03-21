use reference::Reference;

#[must_use]
pub fn label() -> Reference {
    Reference(&["label"])
}
