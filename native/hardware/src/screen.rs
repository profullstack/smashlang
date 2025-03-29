//! Screen module for SmashLang hardware interfaces
//!
//! Provides capabilities for capturing screenshots and recording screen content.
//! Uses platform-specific APIs for desktop (Windows, macOS, Linux) and mobile (Android, iOS) platforms.

use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};
use async_trait::async_trait;

use crate::error::HardwareError;
use crate::Result;
use crate::platform;
use crate::platform::common::ScreenCapture;

/// Screen source information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenSource {
    /// Unique identifier for the screen source
    pub id: String,
    /// Human-readable name for the screen source
    pub name: String,
    /// Type of source ('screen', 'window', 'application')
    pub source_type: String,
    /// Thumbnail image data (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
}

/// Screenshot data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotData {
    /// Base64-encoded image data
    pub data: String,
    /// Width of the image
    pub width: u32,
    /// Height of the image
    pub height: u32,
    /// Format of the image
    pub format: String,
}

/// Recording options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingOptions {
    /// Desired width of the recording
    pub width: Option<u32>,
    /// Desired height of the recording
    pub height: Option<u32>,
    /// Desired frame rate of the recording
    pub frame_rate: Option<u32>,
    /// Whether to include the mouse cursor
    pub capture_mouse_cursor: Option<bool>,
    /// Whether to highlight mouse clicks
    pub capture_clicks: Option<bool>,
    /// Whether to include system audio
    pub capture_audio: Option<bool>,
    /// Video format ('mp4', 'webm', 'gif')
    pub format: Option<String>,
    /// Video quality (0.0 to 1.0)
    pub quality: Option<f32>,
}

/// Screen API for SmashLang
pub struct Screen;

impl Screen {
    /// Check if screen recording is available on this device
    pub async fn is_available() -> bool {
        let screen_capture = create_screen_capture();
        screen_capture.is_available().await
    }
    
    /// Request permission to record the screen
    pub async fn request_permission() -> Result<bool> {
        let screen_capture = create_screen_capture();
        screen_capture.request_permission().await
    }
    
    /// Get a list of available screen sources (displays, windows, applications)
    pub async fn get_sources(source_type: Option<&str>) -> Result<Vec<ScreenSource>> {
        let screen_capture = create_screen_capture();
        screen_capture.get_sources(source_type).await
    }
    
    /// Take a screenshot of the specified source
    pub async fn take_screenshot(source_id: Option<&str>) -> Result<ScreenshotData> {
        let screen_capture = create_screen_capture();
        screen_capture.take_screenshot(source_id).await
    }
    
    /// Save a screenshot to a file
    pub async fn save_screenshot(source_id: Option<&str>, file_path: &str, format: Option<&str>) -> Result<String> {
        let screen_capture = create_screen_capture();
        screen_capture.save_screenshot(source_id, file_path, format).await
    }
    
    /// Start recording the screen
    pub async fn start_recording(source_id: Option<&str>, options: Option<RecordingOptions>) -> Result<String> {
        let screen_capture = create_screen_capture();
        screen_capture.start_recording(source_id, options).await
    }
    
    /// Stop recording the screen
    pub async fn stop_recording(recording_id: &str) -> Result<String> {
        let screen_capture = create_screen_capture();
        screen_capture.stop_recording(recording_id).await
    }
    
    /// Pause recording the screen
    pub async fn pause_recording(recording_id: &str) -> Result<bool> {
        let screen_capture = create_screen_capture();
        screen_capture.pause_recording(recording_id).await
    }
    
    /// Resume recording the screen
    pub async fn resume_recording(recording_id: &str) -> Result<bool> {
        let screen_capture = create_screen_capture();
        screen_capture.resume_recording(recording_id).await
    }
    
    /// Add a marker to the recording
    pub async fn add_marker(recording_id: &str, marker_name: &str) -> Result<bool> {
        let screen_capture = create_screen_capture();
        screen_capture.add_marker(recording_id, marker_name).await
    }
}

/// Create a platform-specific screen capture implementation
fn create_screen_capture() -> Box<dyn ScreenCapture> {
    #[cfg(target_os = "android")]
    {
        Box::new(platform::android::AndroidScreenCapture::new())
    }
    #[cfg(target_os = "ios")]
    {
        Box::new(platform::ios::IOSScreenCapture::new())
    }
    #[cfg(target_os = "windows")]
    {
        Box::new(platform::windows::WindowsScreenCapture::new())
    }
    #[cfg(target_os = "macos")]
    {
        Box::new(platform::macos::MacOSScreenCapture::new())
    }
    #[cfg(target_os = "linux")]
    {
        Box::new(platform::linux::LinuxScreenCapture::new())
    }
    #[cfg(not(any(target_os = "android", target_os = "ios", target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Box::new(platform::common::DefaultScreenCapture::new())
    }
}
