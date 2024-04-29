use crate::error_management::error_kind::ErrorKind;
use crate::error_management::error_template::ErrorTemplate;

pub static DATABASE_CONNECTION_FAILURE: ErrorTemplate = ErrorTemplate {
    code: "database_connection_failure",
    message: "Failed to connect to the database.",
    kind: ErrorKind::Technical,
};

pub static NO_ENTRY_FOUND: ErrorTemplate = ErrorTemplate {
    code: "no_entry_found",
    message: "No {} found for {}.",
    kind: ErrorKind::Functional,
};
pub static BOUT_NOT_RUNNING: ErrorTemplate = ErrorTemplate {
    code: "bout_not_running",
    message: "The bout_id: {} has status {}.",
    kind: ErrorKind::Functional,
};