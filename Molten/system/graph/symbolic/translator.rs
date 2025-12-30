pub use consume;
pub use error;
pub use view;

use component::graph::symbolic::translator::rule::Rules as Data;

pub type Result<T> = error::Result<T>;

pub trait Rules: view::Rules + consume::Rules {}

impl Rules for Data<u8> {}
