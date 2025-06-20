#[cfg(test)]
mod tests {
    use component::mutate::{Delete, Insert};
    use component::query::Search;
    use resource::system::trie as data;
    use standard::case;
    use system::trie::Trie;

    #[case(&mut data::dense(), &['t', 'e', 'a', 'c', 'h', 'i', 'n', 'g'] => 1; "dense")]
    #[case(&mut data::deep(), &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'] => 1; "deep")]
    #[case(&mut data::wide(), &['r', 'u', 's', 't'] => 1; "wide")]
    #[case(&mut data::dense(), &['t', 'e', 's', 't'] => 2; "dense existing")]
    fn insert(trie: &mut Trie<char>, word: &[char]) -> usize {
        trie.insert(word).quantity()
    }

    #[case(&data::dense(), &['t', 'e', 'a', 'c', 'h'] => true; "dense")]
    #[case(&data::dense(), &['t', 'e', 's', 't', 'i', 'n', 'g'] => true; "dense deeper")]
    #[case(&data::deep(), &['a', 'b', 'c', 'd', 'e', 'f'] => true; "deep")]
    #[case(&data::deep(), &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'] => true; "deep deepest")]
    #[case(&data::wide(), &['r', 'u', 'n', 'n', 'i', 'n', 'g'] => true; "wide")]
    #[case(&data::wide(), &['w', 'r', 'o', 'n', 'g'] => false; "wide missing")]
    #[case(&data::dense(), &['t', 'e', 'a', 'c', 'h', 'e', 'r', 's'] => false; "dense missing")]
    #[case(&data::deep(), &['a', 'b', 'c', 'x', 'y', 'z'] => false; "deep missing")]
    fn search(trie: &Trie<char>, word: &[char]) -> bool {
        trie.search(word).is_some()
    }

    #[case(&mut data::dense(), &['t', 'e', 'a', 'c', 'h'] => true; "dense")]
    #[case(&mut data::dense(), &['t', 'e', 's', 't', 'i', 'n', 'g'] => true; "dense deeper")]
    #[case(&mut data::dense(), &['t', 'e', 'a', 'c', 'h', 'e', 'r'] => true; "dense deepest")]
    #[case(&mut data::deep(), &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'] => true; "deep")]
    #[case(&mut data::deep(), &['a', 'b', 'c', 'q', 'r', 's'] => true; "deep branch")]
    #[case(&mut data::deep(), &['a', 'b', 'p'] => true; "deep leaf")]
    #[case(&mut data::wide(), &['r', 'u', 'n'] => true; "wide")]
    #[case(&mut data::wide(), &['w', 'r', 'i', 't', 'e'] => true; "wide different branch")]
    #[case(&mut data::wide(), &['w', 'o', 'r', 'k'] => true; "wide leaf")]
    #[case(&mut data::dense(), &['t', 'e', 'a', 'c', 'h', 'i', 'n', 'g'] => false; "dense missing")]
    #[case(&mut data::deep(), &['a', 'b', 'c', 'x', 'y', 'z'] => false; "deep missing")]
    #[case(&mut data::wide(), &['w', 'r', 'o', 'n', 'g'] => false; "wide missing")]
    #[case(&mut data::dense(), &['t', 'e', 's'] => false; "dense prefix")]
    #[case(&mut data::deep(), &['a', 'b', 'c'] => false; "deep prefix")]
    #[case(&mut data::wide(), &['r'] => false; "wide prefix")]
    fn delete(trie: &mut Trie<char>, word: &[char]) -> bool {
        trie.delete(word).is_some()
    }
}
