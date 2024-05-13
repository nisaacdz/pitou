use serde::Serialize;

#[derive(Serialize)]
pub struct ValueArg<T> {
    pub value: T,
}
