pub trait Categorized {
    type Category;
    fn category(&self) -> &Self::Category;
}

pub trait Valued {
    type Value;
    fn value(&self) -> &Self::Value;
}

pub trait Contextualized {
    type Context;
    fn context(&self) -> &[Self::Context];
}
