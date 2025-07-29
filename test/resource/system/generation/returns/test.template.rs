fn increment(x: &mut i32) -> i32 {
    let prev = *x;
    *x += 1;
    prev
}

fn double_in_place(x: &mut i32) -> i32 {
    let prev = *x;
    *x *= 2;
    prev
}
