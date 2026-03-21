use reference::Reference;

#[must_use]
pub fn logo() -> Reference {
    Reference(&["navigation", "logo"])
}

#[must_use]
pub fn links() -> Reference {
    Reference(&["navigation", "links"])
}

#[must_use]
pub fn dropdown() -> Reference {
    Reference(&["navigation", "dropdown"])
}

#[must_use]
pub fn menu() -> Reference {
    Reference(&["navigation", "menu"])
}

#[must_use]
pub fn nested() -> Reference {
    Reference(&["navigation", "nested"])
}
