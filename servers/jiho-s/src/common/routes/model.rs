#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

impl From<String> for ErrorResponse {
    fn from(value: String) -> Self {
        ErrorResponse {
            message: value,
        }
    }
}