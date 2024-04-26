use std::fmt;
use serde::Serialize;

#[derive(Copy, Clone, Serialize)]
pub enum ErrorKind {
    Technical,
    Functional,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)  // Simplistic implementation, using Debug's output
    }
}

impl fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Functional => write!(f, "Functional Error"),
            ErrorKind::Technical => write!(f, "Technical Error"),
        }
    }
}