use component::graph::symbolic::translator::Translation;

fn new(initial: u64, terminal: u64, elements: Vec<String>) -> Translation<String> {
    Translation::new(initial, terminal, elements)
}

fn initial(translation: Translation<String>) -> u64 {
    translation.initial()
}

fn terminal(translation: Translation<String>) -> u64 {
    translation.terminal()
}

fn elements(translation: Translation<String>) -> Vec<String> {
    translation.elements().clone()
}

fn length(translation: Translation<String>) -> usize {
    translation.length()
}

mod translation {
    use super::Translation;

    pub mod u8 {
        use super::Translation;

        pub fn characterize(translation: Translation<u8>) -> Translation<char> {
            translation.characterize()
        }
    }

    pub mod char {
        use super::Translation;

        pub fn parsed(translation: Translation<char>) -> String {
            translation.parsed()
        }
    }
}
