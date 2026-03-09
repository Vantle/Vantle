use reference::Reference;

#[must_use]
pub fn logo() -> Reference {
    Reference("nav-logo")
}

#[must_use]
pub fn links() -> Reference {
    Reference("nav-links")
}

#[must_use]
pub fn dropdown() -> Reference {
    Reference("nav-dropdown")
}

#[must_use]
pub fn menu() -> Reference {
    Reference("nav-dropdown-menu")
}

#[must_use]
pub fn nested() -> Reference {
    Reference("nav-nested")
}
