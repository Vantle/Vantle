pub use outline;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Effect {
    Outline(outline::Outline),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Effected<T> {
    pub inner: T,
    pub effects: Vec<Effect>,
}

impl<T> Effected<T> {
    #[must_use]
    pub fn effect(mut self, effect: impl Into<Effect>) -> Self {
        self.effects.push(effect.into());
        self
    }

    #[must_use]
    pub fn clear(mut self) -> Self {
        self.effects.clear();
        self
    }

    #[must_use]
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl From<outline::Outline> for Effect {
    fn from(outline: outline::Outline) -> Self {
        Effect::Outline(outline)
    }
}
