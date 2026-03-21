use reference::Reference;

#[must_use]
pub fn context() -> Reference {
    Reference("data-context")
}

#[must_use]
pub fn page() -> Reference {
    Reference("data-page")
}

#[must_use]
pub fn root() -> Reference {
    Reference("data-root")
}

#[must_use]
pub fn progress() -> Reference {
    Reference("data-progress")
}
