#[derive(Copy, Clone)]
pub enum ErrorKind {
    Technical,
    Functional,
}

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

pub struct Error {
    pub code: String,
    pub message: String,
    pub parameters: Vec<String>,
    pub kind: ErrorKind,
}

pub static DATABASE_CONNECTION_FAILURE: ErrorTemplate = ErrorTemplate {
    code: "database_connection_failure",
    message: "Failed to connect to the database.",
    kind: ErrorKind::Technical,
};
