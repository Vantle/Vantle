use system::trie::Trie;

pub fn dense() -> Trie<char> {
    Trie::<char>::new(
        [
            "test", "testing", "tested", "tea", "team", "teach", "teacher",
        ]
        .map(|element| element.chars().collect::<Vec<_>>()),
    )
}

pub fn deep() -> Trie<char> {
    Trie::<char>::new(
        ["abcdefghij", "abcdefxyz", "abcqrs", "abp", "amno"]
            .map(|element| element.chars().collect::<Vec<_>>()),
    )
}

pub fn wide() -> Trie<char> {
    Trie::<char>::new(
        [
            "run", "running", "read", "reading", "ride", "right", "write", "walk", "work",
        ]
        .map(|element| element.chars().collect::<Vec<_>>()),
    )
}

pub fn repeated() -> Trie<char> {
    Trie::<char>::new(
        ["hello", "hello", "hello", "world", "world", "hi"]
            .map(|element| element.chars().collect::<Vec<_>>()),
    )
}
