fn sort(size: usize) -> usize {
    let mut data = (0..size).rev().collect::<Vec<_>>();
    data.sort_unstable();
    data.len()
}
