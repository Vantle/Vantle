use reference::Reference;

#[must_use]
pub fn expanded() -> Reference {
    Reference("data-expanded")
}

#[must_use]
pub fn backdrop() -> Reference {
    Reference("data-backdrop")
}

#[must_use]
pub fn bound() -> Reference {
    Reference("data-expand-bound")
}

#[must_use]
pub fn keyboard() -> Reference {
    Reference("data-expand-keyboard")
}

#[must_use]
pub fn lock() -> Reference {
    Reference("data-expand-lock")
}
