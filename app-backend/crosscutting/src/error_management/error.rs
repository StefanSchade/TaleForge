use std::fmt;
use std::fmt::{Debug, Formatter};
use serde::Serialize;
use crate::error_management::error_kind::ErrorKind;

#[derive(Serialize)]
pub struct Error {
    pub code: String,
    pub message: String,
    pub parameters: Vec<String>,
    pub kind: ErrorKind,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {} (Code: {}, Parameters: {:?})",
            self.kind, self.message, self.code, self.parameters
        )
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Error")
            .field("code", &self.code)
            .field("message", &self.message)
            .field("parameters", &self.parameters)
            .field("kind", &self.kind)
            .finish()
    }
}

impl std::error::Error for Error {}

