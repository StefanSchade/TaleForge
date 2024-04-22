use crate::error_management::error_kind::ErrorKind;
use crate::error_management::error_template::ErrorTemplate;

pub struct Error {
    pub code: String,
    pub message: String,
    pub parameters: Vec<String>,
    pub kind: ErrorKind,

}
