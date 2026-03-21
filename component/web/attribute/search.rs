use reference::Reference;

#[must_use]
pub fn search() -> Reference {
    Reference("data-search")
}

#[must_use]
pub fn function() -> Reference {
    Reference("data-function")
}

#[must_use]
pub fn tags() -> Reference {
    Reference("data-tags")
}

#[must_use]
pub fn hidden() -> Reference {
    Reference("data-hidden")
}

#[must_use]
pub fn counter() -> Reference {
    Reference("data-counter")
}

#[must_use]
pub fn empty() -> Reference {
    Reference("data-empty")
}

#[must_use]
pub fn keyboard() -> Reference {
    Reference("data-keyboard")
}
