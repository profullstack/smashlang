//! Error handling for SmashLang hardware interfaces

use std::fmt;
use thiserror::Error;

/// Hardware interface error types
#[derive(Error, Debug)]
pub enum HardwareError {
    /// Permission denied for hardware access
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    /// Device not found or invalid ID
    #[error("Invalid ID: {0}")]
    InvalidId(String),
    
    /// Device is already in use
    #[error("Device already in use: {0}")]
    AlreadyInUse(String),
    
    /// Device error
    #[error("Device error: {0}")]
    DeviceError(String),
    
    /// Invalid operation
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    
    /// Unsupported operation
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
    
    /// Timeout error
    #[error("Operation timed out: {0}")]
    Timeout(String),
    
    /// I/O error
    #[error("I/O error: {0}")]
    IoError(String),
    
    /// Processing error
    #[error("Processing error: {0}")]
    ProcessingError(String),
    
    /// Invalid parameter
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
    
    /// Other error
    #[error("Error: {0}")]
    Other(String),
}

impl fmt::Display for HardwareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HardwareError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            HardwareError::InvalidId(msg) => write!(f, "Invalid ID: {}", msg),
            HardwareError::AlreadyInUse(msg) => write!(f, "Device already in use: {}", msg),
            HardwareError::DeviceError(msg) => write!(f, "Device error: {}", msg),
            HardwareError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            HardwareError::UnsupportedOperation(msg) => write!(f, "Unsupported operation: {}", msg),
            HardwareError::Timeout(msg) => write!(f, "Operation timed out: {}", msg),
            HardwareError::IoError(msg) => write!(f, "I/O error: {}", msg),
            HardwareError::ProcessingError(msg) => write!(f, "Processing error: {}", msg),
            HardwareError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
            HardwareError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

/// Result type for hardware operations
pub type Result<T> = std::result::Result<T, HardwareError>;

/// Convert a standard error to a hardware error
pub fn to_hardware_error<E: std::error::Error>(err: E, context: &str) -> HardwareError {
    HardwareError::Other(format!("{}: {}", context, err))
}

/// Helper function to convert I/O errors
pub fn io_error_to_hardware_error(err: std::io::Error, context: &str) -> HardwareError {
    HardwareError::IoError(format!("{}: {}", context, err))
}
