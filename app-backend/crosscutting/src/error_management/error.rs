use crate::error_management::error_kind::ErrorKind;

pub struct Error {
    pub code: String,
    pub message: String,
    pub parameters: Vec<String>,
    pub kind: ErrorKind,
}
