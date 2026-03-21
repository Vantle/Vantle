use reference::Reference;

#[must_use]
pub fn attribute() -> Reference {
    Reference(&["node", "attribute"])
}

#[must_use]
pub fn context() -> Reference {
    Reference(&["node", "context"])
}

#[must_use]
pub fn group() -> Reference {
    Reference(&["node", "group"])
}

#[must_use]
pub fn property() -> Reference {
    Reference(&["node", "property"])
}

#[must_use]
pub fn array() -> Reference {
    Reference(&["node", "array"])
}

#[must_use]
pub fn object() -> Reference {
    Reference(&["node", "object"])
}

#[must_use]
pub fn value() -> Reference {
    Reference(&["node", "value"])
}
