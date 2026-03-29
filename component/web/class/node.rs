use reference::Reference;

#[must_use]
pub fn alias() -> Reference {
    Reference(&["node", "alias"])
}

#[must_use]
pub fn array() -> Reference {
    Reference(&["node", "array"])
}

#[must_use]
pub fn attribute() -> Reference {
    Reference(&["node", "attribute"])
}

#[must_use]
pub fn attributes() -> Reference {
    Reference(&["node", "attributes"])
}

#[must_use]
pub fn block() -> Reference {
    Reference(&["node", "block"])
}

#[must_use]
pub fn constant() -> Reference {
    Reference(&["node", "constant"])
}

#[must_use]
pub fn context() -> Reference {
    Reference(&["node", "context"])
}

#[must_use]
pub fn r#enum() -> Reference {
    Reference(&["node", "enum"])
}

#[must_use]
pub fn expression() -> Reference {
    Reference(&["node", "expression"])
}

#[must_use]
pub fn field() -> Reference {
    Reference(&["node", "field"])
}

#[must_use]
pub fn function() -> Reference {
    Reference(&["node", "function"])
}

#[must_use]
pub fn generics() -> Reference {
    Reference(&["node", "generics"])
}

#[must_use]
pub fn group() -> Reference {
    Reference(&["node", "group"])
}

#[must_use]
pub fn r#impl() -> Reference {
    Reference(&["node", "impl"])
}

#[must_use]
pub fn r#let() -> Reference {
    Reference(&["node", "let"])
}

#[must_use]
pub fn r#macro() -> Reference {
    Reference(&["node", "macro"])
}

#[must_use]
pub fn r#mod() -> Reference {
    Reference(&["node", "mod"])
}

#[must_use]
pub fn object() -> Reference {
    Reference(&["node", "object"])
}

#[must_use]
pub fn parameter() -> Reference {
    Reference(&["node", "parameter"])
}

#[must_use]
pub fn parameters() -> Reference {
    Reference(&["node", "parameters"])
}

#[must_use]
pub fn path() -> Reference {
    Reference(&["node", "path"])
}

#[must_use]
pub fn pattern() -> Reference {
    Reference(&["node", "pattern"])
}

#[must_use]
pub fn property() -> Reference {
    Reference(&["node", "property"])
}

#[must_use]
pub fn statement() -> Reference {
    Reference(&["node", "statement"])
}

#[must_use]
pub fn r#static() -> Reference {
    Reference(&["node", "static"])
}

#[must_use]
pub fn r#struct() -> Reference {
    Reference(&["node", "struct"])
}

#[must_use]
pub fn r#trait() -> Reference {
    Reference(&["node", "trait"])
}

#[must_use]
pub fn typed() -> Reference {
    Reference(&["node", "type"])
}

#[must_use]
pub fn r#use() -> Reference {
    Reference(&["node", "use"])
}

#[must_use]
pub fn value() -> Reference {
    Reference(&["node", "value"])
}

#[must_use]
pub fn variant() -> Reference {
    Reference(&["node", "variant"])
}

#[must_use]
pub fn visibility() -> Reference {
    Reference(&["node", "visibility"])
}

#[must_use]
pub fn r#where() -> Reference {
    Reference(&["node", "where"])
}
