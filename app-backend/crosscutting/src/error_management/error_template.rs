use crate::error_management::error::Error;
use crate::error_management::error_kind::ErrorKind;

pub struct ErrorTemplate {
    pub code: &'static str,
    pub message: &'static str,
    pub kind: ErrorKind,
}

impl ErrorTemplate {
    pub fn instantiate(&self, parameters: Vec<String>) -> Error {
        Error {
            code: self.code.to_string(),
            message: self.message.to_string(),
            parameters,
            kind: self.kind,
        }
    }
}