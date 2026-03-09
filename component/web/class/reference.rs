#[derive(Clone, Copy)]
pub struct Reference(pub &'static str);

impl Reference {
    #[must_use]
    pub fn name(self) -> &'static str {
        self.0
    }

    #[must_use]
    pub fn selector(self) -> String {
        format!(".{}", self.0)
    }
}

#[must_use]
pub fn layout() -> Reference {
    Reference("layout")
}

#[must_use]
pub fn sidebar() -> Reference {
    Reference("sidebar")
}

#[must_use]
pub fn outline() -> Reference {
    Reference("outline")
}

#[must_use]
pub fn subtitle() -> Reference {
    Reference("subtitle")
}

#[must_use]
pub fn center() -> Reference {
    Reference("center")
}

#[must_use]
pub fn enhanced() -> Reference {
    Reference("enhanced")
}

#[must_use]
pub fn hamburger() -> Reference {
    Reference("hamburger")
}

#[must_use]
pub fn active() -> Reference {
    Reference("active")
}

#[must_use]
pub fn open() -> Reference {
    Reference("open")
}
