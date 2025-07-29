pub trait Categorized {
    type Category;
    fn category(&self) -> &Self::Category;
}

pub trait Contextualized {
    type Context;
    fn context(&self) -> &[Self::Context];
}
