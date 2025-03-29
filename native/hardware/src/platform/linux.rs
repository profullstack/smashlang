//! Linux-specific hardware implementations
//!
//! This module provides Linux-specific implementations for hardware interfaces
//! using native Linux APIs and libraries, with support for both X11 and Wayland.

use std::path::Path;
use std::process::Command;
use std::env;

use crate::error::HardwareError;
use crate::Result;
use crate::platform::common::*;

#[cfg(feature = "x11")]
use x11rb::connection::Connection;
#[cfg(feature = "x11")]
use x11rb::protocol::xproto::*;

// Wayland support
#[cfg(feature = "wayland")]
use wayland_client::{Display, GlobalManager, protocol::wl_registry};
#[cfg(feature = "wayland")]
use wayland_protocols::wlr::unstable::screencopy::v1::client::zwlr_screencopy_manager_v1;
#[cfg(feature = "wayland")]
use wayland_protocols::wlr::unstable::screencopy::v1::client::zwlr_screencopy_frame_v1;

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

/// Enum representing the display server type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayServer {
    X11,
    Wayland,
    Unknown,
}

/// Detect the display server type (X11 or Wayland)
pub fn get_display_server_type() -> DisplayServer {
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        DisplayServer::Wayland
    } else if std::env::var("DISPLAY").is_ok() {
        DisplayServer::X11
    } else {
        DisplayServer::Unknown
    }
}

/// Camera implementation for Linux
pub mod camera {
    use super::*;
    use libv4l::{context, v4l2};
    use std::collections::HashMap;
    
    /// Check if camera hardware is available
    pub fn is_available() -> bool {
        // Check for video devices in /dev
        if let Ok(entries) = std::fs::read_dir("/dev") {
            entries
                .filter_map(Result::ok)
                .any(|entry| {
                    let name = entry.file_name();
                    let name_str = name.to_string_lossy();
                    name_str.starts_with("video")
                })
        } else {
            false
        }
    }
    
    /// Get a list of available camera devices
    pub fn get_devices() -> Result<Vec<CameraDeviceInfo>> {
        let mut devices = Vec::new();
        
        // Scan /dev/video* devices
        if let Ok(entries) = std::fs::read_dir("/dev") {
            for entry in entries.filter_map(Result::ok) {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                
                if name_str.starts_with("video") {
                    let path = entry.path();
                    let path_str = path.to_string_lossy();
                    
                    // Try to get device info using v4l2
                    if let Ok(ctx) = context::Context::open(&path_str) {
                        if let Ok(caps) = v4l2::capabilities(&ctx) {
                            let device_name = String::from_utf8_lossy(&caps.card).trim_end_matches('\0').to_string();
                            let device_id = path_str.to_string();
                            
                            // Check if it's a capture device
                            if caps.capabilities & v4l2::Capabilities::DEVICE_CAPS as u32 != 0 &&
                               caps.device_caps & v4l2::Capabilities::VIDEO_CAPTURE as u32 != 0 {
                                let mut capabilities = Vec::new();
                                capabilities.push("video".to_string());
                                
                                // Extract index from device name
                                let index = name_str
                                    .trim_start_matches("video")
                                    .parse::<usize>()
                                    .unwrap_or(0);
                                
                                devices.push(CameraDeviceInfo {
                                    id: device_id,
                                    label: device_name,
                                    index,
                                    capabilities,
                                });
                            }
                        }
                    }
                }
            }
        }
        
        Ok(devices)
    }
    
    /// Request permission to use the camera
    /// On Linux, this is typically handled by the OS when the camera is accessed
    pub async fn request_permission() -> Result<bool> {
        // For desktop Linux, we don't typically need explicit permission
        // Just check if we can access video devices
        Ok(is_available())
    }
}

/// Microphone implementation for Linux
pub mod microphone {
    use super::*;
    use alsa::{device_name::HintIter, Direction, PCM};
    
    /// Check if microphone hardware is available
    pub fn is_available() -> bool {
        // Check if we can access ALSA devices
        if let Ok(hints) = HintIter::new(None, "pcm") {
            for hint in hints {
                if let Some(hint) = hint {
                    if let Some(ioid) = hint.ioid {
                        if ioid == Direction::Capture {
                            return true;
                        }
                    }
                }
            }
        }
        
        false
    }
    
    /// Get a list of available microphone devices
    pub fn get_devices() -> Result<Vec<MicrophoneDeviceInfo>> {
        let mut devices = Vec::new();
        
        if let Ok(hints) = HintIter::new(None, "pcm") {
            for (index, hint) in hints.enumerate() {
                if let Some(hint) = hint {
                    // Check if this is a capture device
                    let is_capture = if let Some(ioid) = &hint.ioid {
                        *ioid == Direction::Capture
                    } else {
                        // If ioid is not specified, check if it supports capture
                        if let Some(name) = &hint.name {
                            if let Ok(pcm) = PCM::open(name, Direction::Capture, false) {
                                pcm.close().ok();
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    };
                    
                    if is_capture {
                        if let (Some(name), Some(desc)) = (&hint.name, &hint.desc) {
                            devices.push(MicrophoneDeviceInfo {
                                id: name.clone(),
                                label: desc.clone(),
                                index,
                                capabilities: vec!["audio".to_string()],
                            });
                        }
                    }
                }
            }
        }
        
        Ok(devices)
    }
    
    /// Request permission to use the microphone
    /// On Linux, this is typically handled by the OS when the microphone is accessed
    pub async fn request_permission() -> Result<bool> {
        // For desktop Linux, we don't typically need explicit permission
        // Just check if we can access audio devices
        Ok(is_available())
    }
}

/// Screen capture implementation for Linux
pub mod screen {
    use super::*;
    use std::collections::HashMap;
    
    /// Check if screen capture is available
    pub fn is_available() -> bool {
        // Check if we're running in a graphical environment
        std::env::var("DISPLAY").is_ok() || std::env::var("WAYLAND_DISPLAY").is_ok()
    }
    
    /// Detect the display server type (X11 or Wayland)
    pub fn detect_display_server() -> super::DisplayServer {
        super::get_display_server_type()
    }
    
    /// Get a list of available screen sources
    pub fn get_sources() -> Result<Vec<ScreenSourceInfo>> {
        let mut sources = Vec::new();
        let display_server = detect_display_server();
        
        match display_server {
            DisplayServer::X11 => {
                #[cfg(feature = "x11")]
                {
                    // Use X11 to get screen information if available
                    if let Ok(display) = std::env::var("DISPLAY") {
                        if let Ok((conn, screen_num)) = x11rb::connect(None) {
                            let setup = conn.setup();
                            let screen = setup.roots.get(screen_num as usize).unwrap();
                            
                            // Add the whole screen as a source
                            sources.push(ScreenSourceInfo {
                                id: "screen:0".to_string(),
                                label: format!("Screen {}", screen_num),
                                source_type: "screen".to_string(),
                                width: screen.width_in_pixels as u32,
                                height: screen.height_in_pixels as u32,
                                aspect_ratio: format!("{:.2}", screen.width_in_pixels as f32 / screen.height_in_pixels as f32),
                                thumbnail: None,
                            });
                            
                            // Try to get information about individual windows
                            if let Ok(windows) = conn.query_tree(screen.root) {
                                for (i, window_id) in windows.children.iter().enumerate() {
                                    if let Ok(attrs) = conn.get_window_attributes(*window_id) {
                                        if attrs.map_state == MapState::VIEWABLE {
                                            if let Ok(geom) = conn.get_geometry(*window_id) {
                                                if geom.width > 0 && geom.height > 0 {
                                                    sources.push(ScreenSourceInfo {
                                                        id: format!("window:{}", window_id),
                                                        label: format!("Window {}", i),
                                                        source_type: "window".to_string(),
                                                        width: geom.width as u32,
                                                        height: geom.height as u32,
                                                        aspect_ratio: format!("{:.2}", geom.width as f32 / geom.height as f32),
                                                        thumbnail: None,
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            DisplayServer::Wayland => {
                #[cfg(feature = "wayland")]
                {
                    // Use Wayland protocols to get screen information
                    if let Ok(_) = std::env::var("WAYLAND_DISPLAY") {
                        // Connect to the Wayland display
                        if let Ok(display) = Display::connect_to_env() {
                            let globals = GlobalManager::new(&display);
                            
                            // Roundtrip to ensure we get all globals
                            display.roundtrip().map_err(|e| {
                                HardwareError::PlatformError(format!("Wayland roundtrip error: {}", e))
                            })?;
                            
                            // Check for wlr-screencopy protocol support
                            if globals.instantiate_exact::<zwlr_screencopy_manager_v1::ZwlrScreencopyManagerV1>(1).is_ok() {
                                // Try to get output information
                                // This is a simplified approach - in a real implementation, we would
                                // need to handle Wayland events properly to get output information
                                
                                // For now, we'll add a generic Wayland screen source
                                // In a complete implementation, we would enumerate all outputs
                                sources.push(ScreenSourceInfo {
                                    id: "wayland:0".to_string(),
                                    label: "Wayland Screen".to_string(),
                                    source_type: "screen".to_string(),
                                    width: 1920, // We would get the actual size from the output
                                    height: 1080, // We would get the actual size from the output
                                    aspect_ratio: "1.78".to_string(),
                                    thumbnail: None,
                                });
                                
                                // Note: Wayland doesn't provide a standard way to capture individual windows
                                // Some compositors might support it through custom protocols
                            }
                        }
                    }
                }
                
                // If Wayland support is not compiled in, try to get screen info using external tools
                #[cfg(not(feature = "wayland"))]
                {
                    // Try to get screen information using external tools like wlr-randr
                    if let Ok(output) = Command::new("wlr-randr").output() {
                        if output.status.success() {
                            let output_str = String::from_utf8_lossy(&output.stdout);
                            
                            // Parse the output to extract screen information
                            // This is a simplified approach - in a real implementation, we would
                            // parse the output more carefully
                            
                            // For now, just add a generic Wayland screen source
                            sources.push(ScreenSourceInfo {
                                id: "wayland:0".to_string(),
                                label: "Wayland Screen".to_string(),
                                source_type: "screen".to_string(),
                                width: 1920, // Default assumption
                                height: 1080, // Default assumption
                                aspect_ratio: "1.78".to_string(),
                                thumbnail: None,
                            });
                        }
                    }
                }
            },
            DisplayServer::Unknown => {
                // No known display server detected
            }
        }
        
        // If no sources were found, add a generic screen source
        if sources.is_empty() {
            sources.push(ScreenSourceInfo {
                id: "screen:0".to_string(),
                label: "Primary Screen".to_string(),
                source_type: "screen".to_string(),
                width: 1920, // Default assumption
                height: 1080, // Default assumption
                aspect_ratio: "1.78".to_string(),
                thumbnail: None,
            });
        }
        
        Ok(sources)
    }
    
    /// Take a screenshot
    pub async fn take_screenshot(source_id: Option<&str>) -> Result<Vec<u8>> {
        let display_server = detect_display_server();
        
        match display_server {
            DisplayServer::X11 => {
                #[cfg(feature = "x11")]
                {
                    // Use X11 to take a screenshot
                    // This is a simplified implementation
                    if let Ok((conn, screen_num)) = x11rb::connect(None) {
                        let setup = conn.setup();
                        let screen = setup.roots.get(screen_num as usize).unwrap();
                        
                        // Use an external tool like scrot or import for the actual capture
                        // This is a temporary solution - in a real implementation, we would
                        // use X11 APIs directly to capture the screen
                        let temp_file = format!("/tmp/smashlang_screenshot_{}.png", std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs());
                        
                        let status = Command::new("import")
                            .arg("-window")
                            .arg("root")
                            .arg(&temp_file)
                            .status();
                        
                        if let Ok(status) = status {
                            if status.success() {
                                if let Ok(data) = std::fs::read(&temp_file) {
                                    // Clean up the temporary file
                                    let _ = std::fs::remove_file(&temp_file);
                                    return Ok(data);
                                }
                            }
                        }
                    }
                }
            },
            DisplayServer::Wayland => {
                #[cfg(feature = "wayland")]
                {
                    // Use Wayland protocols to take a screenshot
                    // This is a simplified implementation
                    if let Ok(_) = std::env::var("WAYLAND_DISPLAY") {
                        // For Wayland, we'll use an external tool like grim for now
                        // In a real implementation, we would use the wlr-screencopy protocol
                        let temp_file = format!("/tmp/smashlang_screenshot_{}.png", std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs());
                        
                        let mut cmd = Command::new("grim");
                        
                        // If a specific output is specified, use it
                        if let Some(source_id) = source_id {
                            if source_id.starts_with("wayland:") {
                                // Extract the output name from the source_id
                                // In a real implementation, we would parse this properly
                                cmd.arg("-o").arg("eDP-1"); // Example output name
                            }
                        }
                        
                        cmd.arg(&temp_file);
                        
                        let status = cmd.status();
                        
                        if let Ok(status) = status {
                            if status.success() {
                                if let Ok(data) = std::fs::read(&temp_file) {
                                    // Clean up the temporary file
                                    let _ = std::fs::remove_file(&temp_file);
                                    return Ok(data);
                                }
                            }
                        }
                    }
                }
                
                // If Wayland support is not compiled in or fails, try using external tools
                let temp_file = format!("/tmp/smashlang_screenshot_{}.png", std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs());
                
                // Try grim first (for Wayland)
                let status = Command::new("grim")
                    .arg(&temp_file)
                    .status();
                
                if let Ok(status) = status {
                    if status.success() {
                        if let Ok(data) = std::fs::read(&temp_file) {
                            // Clean up the temporary file
                            let _ = std::fs::remove_file(&temp_file);
                            return Ok(data);
                        }
                    }
                }
            },
            DisplayServer::Unknown => {
                // No known display server detected
                return Err(HardwareError::PlatformError("No supported display server detected".to_string()));
            }
        }
        
        // Fallback to using screenshot-rs
        #[cfg(feature = "screenshot-rs")]
        {
            use screenshot_rs::Screen;
            
            if let Ok(screens) = Screen::all() {
                if !screens.is_empty() {
                    // Use the first screen by default, or parse the source_id to get a specific screen
                    let screen_index = if let Some(source_id) = source_id {
                        source_id.trim_start_matches("screen:").parse::<usize>().unwrap_or(0)
                    } else {
                        0
                    };
                    
                    let screen = screens.get(screen_index).unwrap_or(&screens[0]);
                    if let Ok(image) = screen.capture() {
                        let mut buffer = Vec::new();
                        if image.save_to_png(&mut buffer).is_ok() {
                            return Ok(buffer);
                        }
                    }
                }
            }
        }
        
        Err(HardwareError::PlatformError("Failed to capture screenshot".to_string()))
    }
    
    /// Request permission to capture the screen
    /// On Linux, this is typically handled by the OS when screen capture is initiated
    pub fn request_permission() -> Result<bool> {
        // For desktop Linux, we don't typically need explicit permission
        // Just check if we're in a graphical environment
        Ok(is_available())
    }
}
