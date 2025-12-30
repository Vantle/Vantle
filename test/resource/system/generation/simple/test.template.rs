fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub mod math {
    pub mod operators {
        #[must_use]
        pub fn multiply(x: i32, y: i32) -> i32 {
            x * y
        }

        #[must_use]
        pub fn power(base: i32, exp: i32) -> i32 {
            base.pow(u32::try_from(exp).unwrap_or(0))
        }
    }
}
