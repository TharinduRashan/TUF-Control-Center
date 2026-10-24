use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum HardwareError {
    #[error("Hardware interface not found: {0}")]
    NotFound(String),

    #[error("Permission denied accessing hardware: {0}")]
    PermissionDenied(String),

    #[error("Invalid value provided: {0}")]
    InvalidValue(String),

    #[error("Operation unsupported on this model: {0}")]
    Unsupported(String),

    #[error("D-Bus communication error: {0}")]
    DBus(String),

    #[error("I/O error: {0}")]
    Io(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Thermal safety protection triggered: {0}")]
    SafetyProtection(String),
}

impl From<std::io::Error> for HardwareError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => HardwareError::NotFound(err.to_string()),
            std::io::ErrorKind::PermissionDenied => HardwareError::PermissionDenied(err.to_string()),
            _ => HardwareError::Io(err.to_string()),
        }
    }
}
