#[derive(Clone, Copy)]
pub struct Reference(pub &'static [&'static str]);

impl Reference {
    #[must_use]
    pub fn words(self) -> &'static [&'static str] {
        self.0
    }

    #[must_use]
    pub fn selector(self) -> String {
        self.0.iter().fold(String::new(), |mut s, w| {
            s.push('.');
            s.push_str(w);
            s
        })
    }
}

impl std::fmt::Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, word) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str(" ")?;
            }
            f.write_str(word)?;
        }
        Ok(())
    }
}

#[must_use]
pub fn layout() -> Reference {
    Reference(&["layout"])
}

#[must_use]
pub fn sidebar() -> Reference {
    Reference(&["sidebar"])
}

#[must_use]
pub fn outline() -> Reference {
    Reference(&["outline"])
}

#[must_use]
pub fn subtitle() -> Reference {
    Reference(&["subtitle"])
}

#[must_use]
pub fn center() -> Reference {
    Reference(&["center"])
}

#[must_use]
pub fn enhanced() -> Reference {
    Reference(&["enhanced"])
}

#[must_use]
pub fn hamburger() -> Reference {
    Reference(&["hamburger"])
}

#[must_use]
pub fn active() -> Reference {
    Reference(&["active"])
}

#[must_use]
pub fn open() -> Reference {
    Reference(&["open"])
}
