use reference::Reference;

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
