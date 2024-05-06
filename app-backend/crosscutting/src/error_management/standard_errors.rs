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

pub static PLAYER_NOT_REGISTERED: ErrorTemplate = ErrorTemplate {
    code: "player_not_registered",
    message: "The player: {} is not registered for bout {}.",
    kind: ErrorKind::Functional,
};

pub static UNEXPECTED_ERROR: ErrorTemplate = ErrorTemplate {
    code: "unexpected error",
    message: "Error: {} My comment: {}.",
    kind: ErrorKind::Functional,
};

pub static MANDATORY_FIELD_MISSING: ErrorTemplate = ErrorTemplate {
    code: "mandatory_field_missing",
    message: "Request {} requires filed {}.",
    kind: ErrorKind::Functional,
};
pub static ID_MUST_BE_POSITIVE_INT: ErrorTemplate = ErrorTemplate {
    code: "id_must_be_postive_int",
    message: "the value {} is {} but must be a positive int64",
    kind: ErrorKind::Functional,
};

pub static DATABASE_ACCESS_ERROR: ErrorTemplate = ErrorTemplate {
    code: "db_access_error",
    message: "Error: {} My comment: {}.",
    kind: ErrorKind::Functional,
};
