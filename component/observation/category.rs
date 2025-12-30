use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub enum State {
    Created,
    Existing,
}

#[derive(Clone, Debug, Serialize)]
pub enum Match {
    Found,
    None,
}
