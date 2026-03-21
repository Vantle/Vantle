use reference::Reference;

#[must_use]
pub fn copy() -> Reference {
    Reference(&["copy", "button"])
}

#[must_use]
pub fn theme() -> Reference {
    Reference(&["theme", "toggle"])
}
