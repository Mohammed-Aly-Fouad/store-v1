use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn error(status: u16, message: String) -> Self {
        Self {
            status,
            message,
            data: None,
        }
    }
}