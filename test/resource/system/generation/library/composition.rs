use library::add;

fn add_three(a: i32, b: i32, c: i32) -> i32 {
    add(add(a, b), c)
}
