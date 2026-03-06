pub enum Value {
    Token(Token),
    Calculation(Calculation),
    Concrete(Concrete),
    Keyword(Keyword),
    Composite(Vec<Value>),
    Literal(String),
}

pub enum Token {
    Scale(i32),
    Half(i32),
    Palette(Palette),
    Custom(String),
}

pub enum Palette {
    Text,
    Secondary,
    Background,
    Code,
    CodeText,
    Accent,
    Hover,
    Border,
    Navigation,
    Stripe,
    Keyword,
    Entity,
    Literal,
    Comment,
    Constant,
    Storage,
    Punctuation,
    Variable,
    Function,
    Operator,
    Macro,
}

pub enum Concrete {
    Zero,
    Rem(f32),
    Px(i32),
    Percent(f32),
    Em(f32),
    Seconds(f32),
}

pub enum Keyword {
    Auto,
    None,
    Inherit,
    Initial,
    Transparent,
    Current,
}

pub struct Calculation {
    terms: Vec<Term>,
}

struct Term {
    operation: Operation,
    value: Value,
}

enum Operation {
    Identity,
    Add,
    Subtract,
}

impl Value {
    #[must_use]
    pub fn render(&self) -> String {
        match self {
            Self::Token(token) => token.render(),
            Self::Calculation(calculation) => calculation.render(),
            Self::Concrete(concrete) => concrete.render(),
            Self::Keyword(keyword) => keyword.render().into(),
            Self::Composite(values) => values
                .iter()
                .map(Self::render)
                .collect::<Vec<_>>()
                .join(" "),
            Self::Literal(literal) => literal.clone(),
        }
    }
}

impl Token {
    #[must_use]
    pub fn scale(n: i32) -> Value {
        Value::Token(Self::Scale(n))
    }

    #[must_use]
    pub fn half(n: i32) -> Value {
        Value::Token(Self::Half(n))
    }

    #[must_use]
    pub fn palette(palette: Palette) -> Value {
        Value::Token(Self::Palette(palette))
    }

    #[must_use]
    pub fn custom(name: &str) -> Value {
        Value::Token(Self::Custom(name.into()))
    }

    fn render(&self) -> String {
        match self {
            Self::Scale(n) => {
                let label = if *n < 0 {
                    format!("n{}", n.abs())
                } else {
                    n.to_string()
                };
                format!("var(--scale-{label})")
            }
            Self::Half(n) => {
                let label = if *n < 0 {
                    format!("n{}", n.abs())
                } else {
                    n.to_string()
                };
                format!("var(--scale-{label}h)")
            }
            Self::Palette(palette) => format!("var(--{})", palette.render()),
            Self::Custom(name) => format!("var(--{name})"),
        }
    }
}

impl Palette {
    fn render(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Secondary => "text-secondary",
            Self::Background => "background",
            Self::Code => "code-background",
            Self::CodeText => "code-text",
            Self::Accent => "accent",
            Self::Hover => "accent-hover",
            Self::Border => "border",
            Self::Navigation => "nav-background",
            Self::Stripe => "table-stripe",
            Self::Keyword => "syntax-keyword",
            Self::Entity => "syntax-entity",
            Self::Literal => "syntax-string",
            Self::Comment => "syntax-comment",
            Self::Constant => "syntax-constant",
            Self::Storage => "syntax-storage",
            Self::Punctuation => "syntax-punctuation",
            Self::Variable => "syntax-variable",
            Self::Function => "syntax-function",
            Self::Operator => "syntax-operator",
            Self::Macro => "syntax-macro",
        }
    }
}

impl Concrete {
    #[must_use]
    pub fn zero() -> Value {
        Value::Concrete(Self::Zero)
    }

    #[must_use]
    pub fn rem(n: f32) -> Value {
        Value::Concrete(Self::Rem(n))
    }

    #[must_use]
    pub fn px(n: i32) -> Value {
        Value::Concrete(Self::Px(n))
    }

    #[must_use]
    pub fn percent(n: f32) -> Value {
        Value::Concrete(Self::Percent(n))
    }

    #[must_use]
    pub fn em(n: f32) -> Value {
        Value::Concrete(Self::Em(n))
    }

    #[must_use]
    pub fn seconds(n: f32) -> Value {
        Value::Concrete(Self::Seconds(n))
    }

    fn render(&self) -> String {
        match self {
            Self::Zero => "0".into(),
            Self::Rem(n) => format!("{n}rem"),
            Self::Px(n) => format!("{n}px"),
            Self::Percent(n) => format!("{n}%"),
            Self::Em(n) => format!("{n}em"),
            Self::Seconds(n) => format!("{n}s"),
        }
    }
}

impl Keyword {
    fn render(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::None => "none",
            Self::Inherit => "inherit",
            Self::Initial => "initial",
            Self::Transparent => "transparent",
            Self::Current => "currentColor",
        }
    }
}

impl Calculation {
    #[must_use]
    pub fn start<V>(value: V) -> Self
    where
        V: Into<Value>,
    {
        Self {
            terms: vec![Term {
                operation: Operation::Identity,
                value: value.into(),
            }],
        }
    }

    #[must_use]
    pub fn plus<V>(mut self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.terms.push(Term {
            operation: Operation::Add,
            value: value.into(),
        });
        self
    }

    #[must_use]
    pub fn minus<V>(mut self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.terms.push(Term {
            operation: Operation::Subtract,
            value: value.into(),
        });
        self
    }

    fn render(&self) -> String {
        let mut output = String::from("calc(");
        for (index, term) in self.terms.iter().enumerate() {
            if index > 0 {
                match term.operation {
                    Operation::Add => output.push_str(" + "),
                    Operation::Subtract => output.push_str(" - "),
                    Operation::Identity => {}
                }
            }
            let rendered = term.value.render();
            output.push_str(&rendered);
        }
        output.push(')');
        output
    }
}

impl From<Calculation> for Value {
    fn from(calculation: Calculation) -> Self {
        Self::Calculation(calculation)
    }
}

impl From<Keyword> for Value {
    fn from(keyword: Keyword) -> Self {
        Self::Keyword(keyword)
    }
}

impl From<&str> for Value {
    fn from(literal: &str) -> Self {
        Self::Literal(literal.into())
    }
}

impl From<&String> for Value {
    fn from(literal: &String) -> Self {
        Self::Literal(literal.clone())
    }
}

impl From<String> for Value {
    fn from(literal: String) -> Self {
        Self::Literal(literal)
    }
}

impl<V1, V2> From<(V1, V2)> for Value
where
    V1: Into<Value>,
    V2: Into<Value>,
{
    fn from((a, b): (V1, V2)) -> Self {
        Self::Composite(vec![a.into(), b.into()])
    }
}

impl<V1, V2, V3> From<(V1, V2, V3)> for Value
where
    V1: Into<Value>,
    V2: Into<Value>,
    V3: Into<Value>,
{
    fn from((a, b, c): (V1, V2, V3)) -> Self {
        Self::Composite(vec![a.into(), b.into(), c.into()])
    }
}

impl<V1, V2, V3, V4> From<(V1, V2, V3, V4)> for Value
where
    V1: Into<Value>,
    V2: Into<Value>,
    V3: Into<Value>,
    V4: Into<Value>,
{
    fn from((a, b, c, d): (V1, V2, V3, V4)) -> Self {
        Self::Composite(vec![a.into(), b.into(), c.into(), d.into()])
    }
}

#[must_use]
pub fn composite(values: Vec<Value>) -> Value {
    Value::Composite(values)
}
