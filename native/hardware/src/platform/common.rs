//! Common platform-agnostic traits and utilities for hardware interfaces
//!
//! This module defines the common interfaces that all platform-specific
//! implementations must adhere to, ensuring consistent behavior across
//! different operating systems and devices.

use async_trait::async_trait;
use std::sync::Arc;
use std::collections::HashMap;

use crate::error::HardwareError;
use crate::screen::{ScreenSource, ScreenshotData, RecordingOptions};
use crate::input::{InputEvent, TouchPoint};
use crate::Result;

/// Common trait for screen recording capabilities across all platforms
#[async_trait]
pub trait ScreenCapture: Send + Sync {
    /// Check if screen recording is available on this device
    async fn is_available(&self) -> bool;
    
    /// Request permission to record the screen
    async fn request_permission(&self) -> Result<bool>;
    
    /// Get a list of available screen sources (displays, windows, applications)
    async fn get_sources(&self, source_type: Option<&str>) -> Result<Vec<ScreenSource>>;
    
    /// Take a screenshot
    async fn take_screenshot(&self, source_id: Option<&str>) -> Result<ScreenshotData>;
    
    /// Save a screenshot to a file
    async fn save_screenshot(&self, source_id: Option<&str>, file_path: &str, format: Option<&str>) -> Result<String>;
    
    /// Start recording the screen
    async fn start_recording(&self, source_id: Option<&str>, options: Option<RecordingOptions>) -> Result<String>;
    
    /// Stop recording the screen
    async fn stop_recording(&self, recording_id: &str) -> Result<String>;
    
    /// Pause recording
    async fn pause_recording(&self, recording_id: &str) -> Result<bool>;
    
    /// Resume recording
    async fn resume_recording(&self, recording_id: &str) -> Result<bool>;
    
    /// Add a marker at the current position in the recording
    async fn add_marker(&self, recording_id: &str, marker_name: &str) -> Result<bool>;
}

/// Common trait for input device capabilities across all platforms
#[async_trait]
pub trait InputDevice: Send + Sync {
    /// Check if the specified input device type is available
    fn is_available(&self, device_type: &str) -> bool;
    
    /// Register for input events
    async fn register_events(&self, device_types: Vec<String>) -> Result<String>;
    
    /// Unregister from input events
    fn unregister_events(&self, registration_id: &str) -> Result<bool>;
    
    /// Simulate input event
    async fn simulate_input(&self, event: InputEvent) -> Result<bool>;
    
    /// Get current keyboard state
    fn get_keyboard_state(&self) -> Result<HashMap<String, bool>>;
    
    /// Get current mouse position
    fn get_mouse_position(&self) -> Result<(f32, f32)>;
    
    /// Get current touch points
    fn get_touch_points(&self) -> Result<Vec<TouchPoint>>;
}

/// Default implementation for unsupported platforms
pub struct DefaultScreenCapture;

impl DefaultScreenCapture {
    pub fn new() -> Self {
        DefaultScreenCapture
    }
}

/// Default implementation for unsupported platforms
pub struct DefaultInputDevice;

impl DefaultInputDevice {
    pub fn new() -> Self {
        DefaultInputDevice
    }
}

#[async_trait]
impl InputDevice for DefaultInputDevice {
    fn is_available(&self, _device_type: &str) -> bool {
        false
    }
    
    async fn register_events(&self, _device_types: Vec<String>) -> Result<String> {
        Err(HardwareError::UnsupportedOperation("Input events are not supported on this platform".to_string()))
    }
    
    fn unregister_events(&self, _registration_id: &str) -> Result<bool> {
        Err(HardwareError::UnsupportedOperation("Input events are not supported on this platform".to_string()))
    }
    
    async fn simulate_input(&self, _event: InputEvent) -> Result<bool> {
        Err(HardwareError::UnsupportedOperation("Input simulation is not supported on this platform".to_string()))
    }
    
    fn get_keyboard_state(&self) -> Result<HashMap<String, bool>> {
        Err(HardwareError::UnsupportedOperation("Keyboard state is not supported on this platform".to_string()))
    }
    
    fn get_mouse_position(&self) -> Result<(f32, f32)> {
        Err(HardwareError::UnsupportedOperation("Mouse position is not supported on this platform".to_string()))
    }
    
    fn get_touch_points(&self) -> Result<Vec<TouchPoint>> {
        Err(HardwareError::UnsupportedOperation("Touch input is not supported on this platform".to_string()))
    }
}

#[async_trait]
impl ScreenCapture for DefaultScreenCapture {
    async fn is_available(&self) -> bool {
        false // Not available on unsupported platforms
    }
    
    async fn request_permission(&self) -> Result<bool> {
        Err(HardwareError::UnsupportedOperation("Screen recording is not supported on this platform".to_string()))
    }
    
    async fn get_sources(&self, _source_type: Option<&str>) -> Result<Vec<ScreenSource>> {
        Err(HardwareError::UnsupportedOperation("Screen recording is not supported on this platform".to_string()))
    }
    
    async fn take_screenshot(&self, _source_id: Option<&str>) -> Result<ScreenshotData> {
        Err(HardwareError::UnsupportedOperation("Screen recording is not supported on this platform".to_string()))
    }
    
    async fn save_screenshot(&self, _source_id: Option<&str>, _file_path: &str, _format: Option<&str>) -> Result<String> {
        Err(HardwareError::UnsupportedOperation("Screen recording is not supported on this platform".to_string()))
    }
    
    async fn start_recording(&self, _source_id: Option<&str>, _options: Option<RecordingOptions>) -> Result<String> {
        Err(HardwareError::UnsupportedOperation("Screen recording is not supported on this platform".to_string()))
    }
    
    async fn stop_recording(&self, _recording_id: &str) -> Result<String> {
        Err(HardwareError::UnsupportedOperation("Screen recording is not supported on this platform".to_string()))
    }
    
    async fn pause_recording(&self, _recording_id: &str) -> Result<bool> {
        Err(HardwareError::UnsupportedOperation("Screen recording is not supported on this platform".to_string()))
    }
    
    async fn resume_recording(&self, _recording_id: &str) -> Result<bool> {
        Err(HardwareError::UnsupportedOperation("Screen recording is not supported on this platform".to_string()))
    }
    
    async fn add_marker(&self, _recording_id: &str, _marker_name: &str) -> Result<bool> {
        Err(HardwareError::UnsupportedOperation("Screen recording is not supported on this platform".to_string()))
    }
}
