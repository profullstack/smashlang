//! Common platform-agnostic traits and utilities for hardware interfaces
//!
//! This module defines the common interfaces that all platform-specific
//! implementations must adhere to, ensuring consistent behavior across
//! different operating systems and devices.

use async_trait::async_trait;
use std::sync::Arc;

use crate::error::HardwareError;
use crate::screen::{ScreenSource, Screenshot, ScreenshotOptions, ScreenRecorder, ScreenRecordingOptions, RecordingResult, SaveResult};
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
    async fn take_screenshot(&self, source_id: Option<&str>, options: ScreenshotOptions) -> Result<Screenshot>;
    
    /// Save a screenshot to a file
    async fn save_screenshot(&self, screenshot_data: &str, file_path: &str) -> Result<SaveResult>;
    
    /// Start recording the screen
    async fn start_recording(&self, options: ScreenRecordingOptions) -> Result<ScreenRecorder>;
    
    /// Stop recording and save to a file
    async fn stop_recording(&self, recorder_id: &str, file_path: &str) -> Result<RecordingResult>;
    
    /// Pause recording
    async fn pause_recording(&self, recorder_id: &str) -> Result<()>;
    
    /// Resume recording
    async fn resume_recording(&self, recorder_id: &str) -> Result<()>;
    
    /// Add a marker at the current position in the recording
    async fn add_marker(&self, recorder_id: &str, label: &str) -> Result<()>;
}

/// Factory for creating platform-specific implementations
pub fn create_screen_capture() -> Arc<dyn ScreenCapture> {
    #[cfg(target_os = "android")]
    {
        use crate::platform::android::AndroidScreenCapture;
        Arc::new(AndroidScreenCapture::new())
    }
    
    #[cfg(target_os = "ios")]
    {
        use crate::platform::ios::IOSScreenCapture;
        Arc::new(IOSScreenCapture::new())
    }
    
    #[cfg(target_os = "windows")]
    {
        use crate::platform::windows::WindowsScreenCapture;
        Arc::new(WindowsScreenCapture::new())
    }
    
    #[cfg(target_os = "macos")]
    {
        use crate::platform::macos::MacOSScreenCapture;
        Arc::new(MacOSScreenCapture::new())
    }
    
    #[cfg(target_os = "linux")]
    {
        use crate::platform::linux::LinuxScreenCapture;
        Arc::new(LinuxScreenCapture::new())
    }
    
    #[cfg(not(any(target_os = "android", target_os = "ios", target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        // Fallback for unsupported platforms
        use crate::platform::fallback::FallbackScreenCapture;
        Arc::new(FallbackScreenCapture::new())
    }
}
