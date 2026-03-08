fn sort(size: usize) -> usize {
    let mut state: u64 = 0xDEAD_BEEF_CAFE_BABE;
    let mut data = (0..size)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        })
        .collect::<Vec<_>>();
    data.sort_unstable();
    data.len()
}
