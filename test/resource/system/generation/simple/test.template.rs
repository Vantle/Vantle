fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub mod math {
    pub mod operators {
        pub fn multiply(x: i32, y: i32) -> i32 {
            x * y
        }

        pub fn power(base: i32, exp: i32) -> i32 {
            base.pow(exp as u32)
        }
    }
}
