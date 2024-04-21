// In your new crosscutting crate:
pub enum ErrorKind {
    Technical,
    Functional,
}

pub struct Error {
    pub code: String,
    pub parameters: Vec<String>,
    pub kind: ErrorKind,
}

impl Error {
    pub fn new(code: String, parameters: Vec<String>, kind: ErrorKind) -> Self {
        Error { code, parameters, kind }
    }
}