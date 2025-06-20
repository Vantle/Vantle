#[cfg(test)]
mod tests {
    use component::order;
    use component::query::Strategy;
    use resource::system::trie as data;
    use standard::case;
    use system::trie::Trie;

    #[case(&data::dense() => vec!["tea", "team", "test", "teach", "tested", "teacher", "testing"]; "dense")]
    #[case(&data::deep() => vec!["abp", "amno", "abcqrs", "abcdefxyz", "abcdefghij"]; "deep")]
    #[case(&data::wide() => vec!["run", "read", "ride", "walk", "work", "right", "write", "reading", "running"]; "wide")]
    #[case(&data::repeated() => vec!["hi", "hello", "hello", "hello", "world", "world"]; "repeated")]
    fn breadth(trie: &Trie<char>) -> Vec<String> {
        trie.traverse::<order::Breadth>()
            .map(|chars| chars.into_iter().collect())
            .collect()
    }

    #[case(&data::dense() => vec!["tea", "teach", "teacher", "team", "test", "tested", "testing"]; "dense")]
    #[case(&data::deep() => vec!["abcdefghij", "abcdefxyz", "abcqrs", "abp", "amno"]; "deep")]
    #[case(&data::wide() => vec!["read", "reading", "ride", "right", "run", "running", "walk", "work", "write"]; "wide")]
    #[case(&data::repeated() => vec!["hello", "hello", "hello", "hi", "world", "world"]; "repeated")]
    fn depth(trie: &Trie<char>) -> Vec<String> {
        trie.traverse::<order::Depth>()
            .map(|chars| chars.into_iter().collect())
            .collect()
    }
}
