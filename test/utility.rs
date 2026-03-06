pub fn equal<T: PartialEq>(left: &T, right: &T) -> bool {
    left == right
}

pub fn unwrap<T, E>(result: Result<T, E>) -> T
where
    E: miette::Diagnostic + std::error::Error + Send + Sync + 'static,
{
    match result {
        Ok(value) => value,
        Err(error) => {
            eprintln!("{:?}", miette::Report::new(error));
            panic!("expected Ok result");
        }
    }
}
