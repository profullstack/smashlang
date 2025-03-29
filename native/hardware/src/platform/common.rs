//! Common platform-agnostic traits and utilities for hardware interfaces
//!
//! This module defines the common interfaces that all platform-specific
//! implementations must adhere to, ensuring consistent behavior across
//! different operating systems and devices.

use async_trait::async_trait;
use std::sync::Arc;

use crate::error::HardwareError;
use crate::screen::{ScreenSource, ScreenshotData, RecordingOptions};
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

/// Default implementation for unsupported platforms
pub struct DefaultScreenCapture;

impl DefaultScreenCapture {
    pub fn new() -> Self {
        DefaultScreenCapture
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
