fn increment(x: &mut i32) -> i32 {
    let prev = *x;
    *x += 1;
    prev // return original value, mutated value is original + 1
}

fn double_in_place(x: &mut i32) -> i32 {
    let prev = *x;
    *x *= 2; // mutate in place
    prev // return original value, mutated value is doubled
}
