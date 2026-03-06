#[derive(Clone, Copy)]
pub struct Reference(&'static str);

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

pub mod nav {
    use super::Reference;

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
}

pub mod label {
    use super::Reference;

    #[must_use]
    pub fn sidebar() -> Reference {
        Reference("sidebar-label")
    }

    #[must_use]
    pub fn outline() -> Reference {
        Reference("outline-label")
    }
}

pub mod footer {
    use super::Reference;

    #[must_use]
    pub fn icon() -> Reference {
        Reference("footer-icon")
    }
}

pub mod code {
    use super::Reference;

    #[must_use]
    pub fn block() -> Reference {
        Reference("code-block")
    }

    #[must_use]
    pub fn toolbar() -> Reference {
        Reference("code-toolbar")
    }

    #[must_use]
    pub fn source() -> Reference {
        Reference("code-source")
    }
}

pub mod button {
    use super::Reference;

    #[must_use]
    pub fn copy() -> Reference {
        Reference("copy-button")
    }

    #[must_use]
    pub fn theme() -> Reference {
        Reference("theme-toggle")
    }
}

pub mod syntax {
    use super::Reference;

    #[must_use]
    pub fn keyword() -> Reference {
        Reference("syntax-keyword")
    }

    #[must_use]
    pub fn entity() -> Reference {
        Reference("syntax-entity")
    }

    #[must_use]
    pub fn string() -> Reference {
        Reference("syntax-string")
    }

    #[must_use]
    pub fn comment() -> Reference {
        Reference("syntax-comment")
    }

    #[must_use]
    pub fn constant() -> Reference {
        Reference("syntax-constant")
    }

    #[must_use]
    pub fn storage() -> Reference {
        Reference("syntax-storage")
    }

    #[must_use]
    pub fn punctuation() -> Reference {
        Reference("syntax-punctuation")
    }

    #[must_use]
    pub fn variable() -> Reference {
        Reference("syntax-variable")
    }

    #[must_use]
    pub fn function() -> Reference {
        Reference("syntax-function")
    }

    #[must_use]
    pub fn operator() -> Reference {
        Reference("syntax-operator")
    }

    #[must_use]
    pub fn r#macro() -> Reference {
        Reference("syntax-macro")
    }
}

pub mod node {
    use super::Reference;

    #[must_use]
    pub fn attribute() -> Reference {
        Reference("node-attribute")
    }

    #[must_use]
    pub fn context() -> Reference {
        Reference("node-context")
    }

    #[must_use]
    pub fn group() -> Reference {
        Reference("node-group")
    }

    #[must_use]
    pub fn property() -> Reference {
        Reference("node-property")
    }

    #[must_use]
    pub fn array() -> Reference {
        Reference("node-array")
    }

    #[must_use]
    pub fn object() -> Reference {
        Reference("node-object")
    }

    #[must_use]
    pub fn value() -> Reference {
        Reference("node-value")
    }
}
