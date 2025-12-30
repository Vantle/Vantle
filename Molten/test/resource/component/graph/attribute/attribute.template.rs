use assemble::Assemble;
use component::graph::attribute::{Assembler, Attribute, Category};

mod category {
    use super::Category;

    pub fn attribute(value: String) -> Category<String> {
        Category::Attribute(value)
    }

    pub fn context() -> Category<String> {
        Category::Context
    }

    pub fn group() -> Category<String> {
        Category::Group
    }

    pub fn partition() -> Category<String> {
        Category::Partition
    }

    pub fn void() -> Category<String> {
        Category::Void
    }
}

mod assembler {
    use super::{Assemble, Assembler, Attribute, Category};

    pub fn new(category: Category<String>) -> Attribute<String> {
        Assembler::new(category).assemble()
    }

    pub fn empty() -> Attribute<String> {
        Assembler::empty().assemble()
    }

    pub fn context(category: Category<String>, attribute: Attribute<String>) -> Attribute<String> {
        let mut assembler = Assembler::new(category);
        assembler.context(attribute);
        assembler.assemble()
    }

    pub fn then(
        category: Category<String>,
        attributes: Vec<Attribute<String>>,
    ) -> Attribute<String> {
        let mut assembler = Assembler::new(category);
        for attribute in attributes {
            let _ = assembler.then(attribute);
        }
        assembler.assemble()
    }

    pub fn category(value: Category<String>) -> Attribute<String> {
        Assembler::empty().category(value).assemble()
    }
}
