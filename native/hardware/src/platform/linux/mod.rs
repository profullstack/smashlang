//! Linux platform module for hardware interfaces

use std::path::Path;
use std::process::Command;
use std::env;

use crate::error::HardwareError;
use crate::Result;
use crate::platform::common::*;

// Re-export the display server type and detection function
pub use super::DisplayServer;
pub use super::get_display_server_type;

// Sub-modules
pub mod camera;
pub mod microphone;
pub mod screen;
pub mod input;

/// Check if a device path exists
pub fn device_exists(path: &str) -> bool {
    Path::new(path).exists()
}

/// Check if a kernel module is loaded
pub fn module_loaded(module_name: &str) -> bool {
    if let Ok(output) = Command::new("lsmod").output() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        output_str.contains(module_name)
    } else {
        false
    }
}
