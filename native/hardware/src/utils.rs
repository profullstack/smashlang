//! Utility functions for SmashLang hardware interfaces

use std::path::Path;

use crate::error::HardwareError;
use crate::Result;

/// Check if a file path is valid and writable
pub fn check_file_path(path: &str) -> Result<()> {
    let path = Path::new(path);
    
    // Check if the parent directory exists
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            return Err(HardwareError::IoError(format!("Directory does not exist: {}", parent.display())));
        }
        
        // Check if the parent directory is writable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = parent.metadata() {
                let permissions = metadata.permissions();
                if permissions.mode() & 0o200 == 0 {
                    return Err(HardwareError::IoError(format!("Directory is not writable: {}", parent.display())));
                }
            }
        }
    }
    
    // Check if the file already exists and is writable
    if path.exists() {
        if let Ok(metadata) = path.metadata() {
            if metadata.is_dir() {
                return Err(HardwareError::IoError(format!("Path is a directory, not a file: {}", path.display())));
            }
            
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let permissions = metadata.permissions();
                if permissions.mode() & 0o200 == 0 {
                    return Err(HardwareError::IoError(format!("File is not writable: {}", path.display())));
                }
            }
        }
    }
    
    Ok(())
}

/// Get the file extension from a path
pub fn get_file_extension(path: &str) -> Option<String> {
    Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
}

/// Validate a file format against a list of supported formats
pub fn validate_format(format: &str, supported_formats: &[&str]) -> Result<()> {
    let format = format.to_lowercase();
    if !supported_formats.contains(&format.as_str()) {
        return Err(HardwareError::InvalidParameter(
            format!("Unsupported format: {}. Supported formats: {}", format, supported_formats.join(", "))
        ));
    }
    
    Ok(())
}

/// Convert a quality value (0.0-1.0) to a format-specific quality value
pub fn convert_quality(quality: f32, format: &str) -> u8 {
    let quality = quality.clamp(0.0, 1.0);
    
    match format.to_lowercase().as_str() {
        "jpeg" | "jpg" => (quality * 100.0) as u8,
        "webp" => (quality * 100.0) as u8,
        _ => 90, // Default quality for other formats
    }
}

/// Generate a unique filename with a timestamp
pub fn generate_unique_filename(prefix: &str, extension: &str) -> String {
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    format!("{}_{}_{}.{}", prefix, timestamp, uuid::Uuid::new_v4().to_simple(), extension)
}

/// Calculate the size of a file in bytes
pub fn get_file_size(path: &str) -> Result<u64> {
    let metadata = std::fs::metadata(path)
        .map_err(|e| HardwareError::IoError(format!("Failed to get file metadata: {}", e)))?;
    
    Ok(metadata.len())
}

/// Convert a base64 string to bytes
pub fn base64_to_bytes(base64_str: &str) -> Result<Vec<u8>> {
    base64::decode(base64_str)
        .map_err(|e| HardwareError::ProcessingError(format!("Failed to decode base64 data: {}", e)))
}

/// Convert bytes to a base64 string
pub fn bytes_to_base64(bytes: &[u8]) -> String {
    base64::encode(bytes)
}

/// Check if a directory exists and is writable
pub fn check_directory(dir_path: &str) -> Result<()> {
    let path = Path::new(dir_path);
    
    if !path.exists() {
        return Err(HardwareError::IoError(format!("Directory does not exist: {}", path.display())));
    }
    
    if !path.is_dir() {
        return Err(HardwareError::IoError(format!("Path is not a directory: {}", path.display())));
    }
    
    // Check if the directory is writable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = path.metadata() {
            let permissions = metadata.permissions();
            if permissions.mode() & 0o200 == 0 {
                return Err(HardwareError::IoError(format!("Directory is not writable: {}", path.display())));
            }
        }
    }
    
    Ok(())
}

/// Create a directory if it doesn't exist
pub fn create_directory_if_not_exists(dir_path: &str) -> Result<()> {
    let path = Path::new(dir_path);
    
    if !path.exists() {
        std::fs::create_dir_all(path)
            .map_err(|e| HardwareError::IoError(format!("Failed to create directory: {}", e)))?;
    }
    
    Ok(())
}
