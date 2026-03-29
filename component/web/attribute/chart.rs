use reference::Reference;

#[must_use]
pub fn chart() -> Reference {
    Reference("data-chart")
}

#[must_use]
pub fn bound() -> Reference {
    Reference("data-chart-bound")
}
