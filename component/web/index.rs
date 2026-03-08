#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Context {
    Vantle,
    Molten,
    Autotest,
    Generation,
}

impl Context {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Vantle => "vantle",
            Self::Molten => "molten",
            Self::Autotest => "autotest",
            Self::Generation => "generation",
        }
    }
}

pub struct Index {
    pub href: String,
    pub context: Context,
    pub identifier: &'static str,
}

impl Index {
    #[must_use]
    pub fn new(root: &str, destination: &str, context: Context, identifier: &'static str) -> Self {
        Self {
            href: format!("{root}{destination}"),
            context,
            identifier,
        }
    }

    #[must_use]
    pub fn fragment(&self, anchor: &str) -> String {
        format!("{}#{anchor}", self.href)
    }
}
