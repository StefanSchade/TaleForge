use crate::error_management::error_kind::ErrorKind;
use crate::error_management::error_template::ErrorTemplate;

pub static DATABASE_CONNECTION_FAILURE: ErrorTemplate = ErrorTemplate {
    code: "database_connection_failure",
    message: "Failed to connect to the database.",
    kind: ErrorKind::Technical,
};
