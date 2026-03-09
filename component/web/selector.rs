pub enum Selector {
    Tag(Tag),
    Universal,
    Class(String),
    Id(String),
    Attribute(Attribute),
    Pseudo(Box<Selector>, Pseudo),
    Descendant(Box<Selector>, Box<Selector>),
    Child(Box<Selector>, Box<Selector>),
    Compound(Vec<Selector>),
    Group(Vec<Selector>),
    Literal(String),
}

pub enum Tag {
    Html,
    Body,
    Nav,
    Main,
    Aside,
    Footer,
    Section,
    Div,
    Span,
    P,
    A,
    Img,
    Hr,
    Br,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Ul,
    Ol,
    Li,
    Dl,
    Dt,
    Dd,
    Table,
    Thead,
    Tbody,
    Tr,
    Th,
    Td,
    Pre,
    Code,
    Strong,
    Em,
    Blockquote,
    Button,
    Svg,
}

pub enum Pseudo {
    Hover,
    Focus,
    FocusWithin,
    FirstChild,
    NthChild(String),
    Active,
}

pub struct Attribute {
    name: String,
    value: Option<String>,
}

#[must_use]
pub fn tag(element: Tag) -> Selector {
    Selector::Tag(element)
}

#[must_use]
pub fn class(reference: reference::Reference) -> Selector {
    Selector::Class(reference.name().into())
}

#[must_use]
pub fn id(name: &str) -> Selector {
    Selector::Id(name.into())
}

#[must_use]
pub fn attribute(name: &str, value: &str) -> Selector {
    Selector::Attribute(Attribute {
        name: name.into(),
        value: Some(value.into()),
    })
}

#[must_use]
pub fn present(name: &str) -> Selector {
    Selector::Attribute(Attribute {
        name: name.into(),
        value: None,
    })
}

#[must_use]
pub fn universal() -> Selector {
    Selector::Universal
}

#[must_use]
pub fn group(selectors: Vec<Selector>) -> Selector {
    Selector::Group(selectors)
}

impl Selector {
    #[must_use]
    pub fn descendant(self, other: Selector) -> Selector {
        Selector::Descendant(Box::new(self), Box::new(other))
    }

    #[must_use]
    pub fn child(self, other: Selector) -> Selector {
        Selector::Child(Box::new(self), Box::new(other))
    }

    #[must_use]
    pub fn pseudo(self, pseudo: Pseudo) -> Selector {
        Selector::Pseudo(Box::new(self), pseudo)
    }

    #[must_use]
    pub fn and(self, other: Selector) -> Selector {
        match self {
            Selector::Compound(mut items) => {
                items.push(other);
                Selector::Compound(items)
            }
            _ => Selector::Compound(vec![self, other]),
        }
    }

    #[must_use]
    pub fn attribute(self, name: &str, value: &str) -> Selector {
        self.and(Selector::Attribute(Attribute {
            name: name.into(),
            value: Some(value.into()),
        }))
    }

    #[must_use]
    pub fn render(&self) -> String {
        match self {
            Self::Tag(element) => element.render().into(),
            Self::Universal => "*".into(),
            Self::Class(name) => format!(".{name}"),
            Self::Id(name) => format!("#{name}"),
            Self::Attribute(attr) => attr.render(),
            Self::Pseudo(base, pseudo) => {
                format!("{}:{}", base.render(), pseudo.render())
            }
            Self::Descendant(parent, child) => {
                format!("{} {}", parent.render(), child.render())
            }
            Self::Child(parent, child) => {
                format!("{} > {}", parent.render(), child.render())
            }
            Self::Compound(items) => items.iter().map(Self::render).collect::<String>(),
            Self::Group(items) => items
                .iter()
                .map(Self::render)
                .collect::<Vec<_>>()
                .join(", "),
            Self::Literal(raw) => raw.clone(),
        }
    }
}

impl Tag {
    fn render(&self) -> &'static str {
        match self {
            Self::Html => "html",
            Self::Body => "body",
            Self::Nav => "nav",
            Self::Main => "main",
            Self::Aside => "aside",
            Self::Footer => "footer",
            Self::Section => "section",
            Self::Div => "div",
            Self::Span => "span",
            Self::P => "p",
            Self::A => "a",
            Self::Img => "img",
            Self::Hr => "hr",
            Self::Br => "br",
            Self::H1 => "h1",
            Self::H2 => "h2",
            Self::H3 => "h3",
            Self::H4 => "h4",
            Self::H5 => "h5",
            Self::H6 => "h6",
            Self::Ul => "ul",
            Self::Ol => "ol",
            Self::Li => "li",
            Self::Dl => "dl",
            Self::Dt => "dt",
            Self::Dd => "dd",
            Self::Table => "table",
            Self::Thead => "thead",
            Self::Tbody => "tbody",
            Self::Tr => "tr",
            Self::Th => "th",
            Self::Td => "td",
            Self::Pre => "pre",
            Self::Code => "code",
            Self::Strong => "strong",
            Self::Em => "em",
            Self::Blockquote => "blockquote",
            Self::Button => "button",
            Self::Svg => "svg",
        }
    }
}

impl Pseudo {
    fn render(&self) -> String {
        match self {
            Self::Hover => "hover".into(),
            Self::Focus => "focus".into(),
            Self::FocusWithin => "focus-within".into(),
            Self::FirstChild => "first-child".into(),
            Self::NthChild(expression) => format!("nth-child({expression})"),
            Self::Active => "active".into(),
        }
    }
}

impl Attribute {
    fn render(&self) -> String {
        match &self.value {
            Some(value) => format!("[{}=\"{}\"]", self.name, value),
            None => format!("[{}]", self.name),
        }
    }
}

impl From<&str> for Selector {
    fn from(literal: &str) -> Self {
        Self::Literal(literal.into())
    }
}

impl From<String> for Selector {
    fn from(literal: String) -> Self {
        Self::Literal(literal)
    }
}

impl From<reference::Reference> for Selector {
    fn from(reference: reference::Reference) -> Self {
        Self::Class(reference.name().into())
    }
}
