use serde::Serialize;

#[derive(Serialize)]
pub struct HTTPError {
    pub error: String,
}
