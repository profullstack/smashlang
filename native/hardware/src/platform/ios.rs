//! iOS-specific implementation for screen recording
//!
//! This module provides screen recording capabilities for iOS devices
//! using the ReplayKit framework through Objective-C bindings.

use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use uuid::Uuid;
use log::{info, warn, error};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

#[cfg(target_os = "ios")]
use {
    objc::{runtime::{Class, Object}, msg_send, sel, sel_impl},
    objc_foundation::{INSString, NSString},
    objc_id::{Id, ShareId},
};

use crate::error::HardwareError;
use crate::screen::{ScreenSource, ScreenshotData, RecordingOptions};
use crate::Result;
use crate::platform::common::ScreenCapture;

/// iOS implementation of screen capture using ReplayKit
pub struct IOSScreenCapture {
    recording_instances: Arc<Mutex<HashMap<String, String>>>,
}

impl IOSScreenCapture {
    /// Create a new instance of IOSScreenCapture
    pub fn new() -> Self {
        Self {
            recording_instances: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    #[cfg(target_os = "ios")]
    fn is_replay_kit_available() -> bool {
        unsafe {
            let replay_kit_class = Class::get("RPScreenRecorder").unwrap();
            let shared_recorder: *mut Object = msg_send![replay_kit_class, sharedRecorder];
            let is_available: bool = msg_send![shared_recorder, isAvailable];
            is_available
        }
    }
    
    #[cfg(target_os = "ios")]
    fn request_replay_kit_permission(&self) -> Result<bool> {
        unsafe {
            let replay_kit_class = match Class::get("RPScreenRecorder") {
                Some(class) => class,
                None => return Err(HardwareError::DeviceAccessError("ReplayKit not available".to_string())),
            };
            
            let shared_recorder: *mut Object = msg_send![replay_kit_class, sharedRecorder];
            
            // Create a semaphore to wait for the async permission request
            let semaphore = std::sync::Arc::new(std::sync::Mutex::new(false));
            let semaphore_clone = semaphore.clone();
            
            // Request permission
            let _: () = msg_send![shared_recorder, requestRecordingPermissionWithHandler:^(BOOL granted) {
                let mut permission = semaphore_clone.lock().unwrap();
                *permission = granted;
            }];
            
            // Wait for a reasonable amount of time (5 seconds max)
            let start = std::time::Instant::now();
            let timeout = std::time::Duration::from_secs(5);
            
            loop {
                {
                    let permission = semaphore.lock().unwrap();
                    if *permission {
                        return Ok(true);
                    }
                }
                
                if start.elapsed() > timeout {
                    break;
                }
                
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            
            // If we timed out, assume permission was denied
            Err(HardwareError::PermissionDenied("Screen recording permission denied or timed out".to_string()))
        }
    }
    
    #[cfg(target_os = "ios")]
    fn get_screen_dimensions(&self) -> (u32, u32) {
        unsafe {
            let ui_screen_class = Class::get("UIScreen").unwrap();
            let main_screen: *mut Object = msg_send![ui_screen_class, mainScreen];
            let bounds: objc::runtime::NSRect = msg_send![main_screen, bounds];
            
            // Get the scale to account for Retina displays
            let scale: f64 = msg_send![main_screen, scale];
            
            let width = (bounds.size.width * scale) as u32;
            let height = (bounds.size.height * scale) as u32;
            
            (width, height)
        }
    }
}

#[async_trait]
impl ScreenCapture for IOSScreenCapture {
    async fn is_available(&self) -> bool {
        #[cfg(target_os = "ios")]
        {
            Self::is_replay_kit_available()
        }
        
        #[cfg(not(target_os = "ios"))]
        {
            false // Not available on non-iOS platforms
        }
    }
    
    async fn request_permission(&self) -> Result<bool> {
        #[cfg(target_os = "ios")]
        {
            self.request_replay_kit_permission()
        }
        
        #[cfg(not(target_os = "ios"))]
        {
            Err(HardwareError::UnsupportedPlatform("iOS screen recording is not available on this platform".to_string()))
        }
    }
    
    async fn get_sources(&self, source_type: Option<&str>) -> Result<Vec<ScreenSource>> {
        #[cfg(target_os = "ios")]
        {
            let mut sources = Vec::new();
            let type_filter = source_type.unwrap_or("all");
            
            // On iOS, only the full screen is available through ReplayKit
            if type_filter == "all" || type_filter == "screen" {
                let (width, height) = self.get_screen_dimensions();
                sources.push(ScreenSource {
                    id: "screen_0".to_string(),
                    name: format!("iOS Display ({}x{})", width, height),
                    source_type: "screen".to_string(),
                    thumbnail: None,
                });
            }
            
            Ok(sources)
        }
        
        #[cfg(not(target_os = "ios"))]
        {
            Err(HardwareError::UnsupportedPlatform("iOS screen recording is not available on this platform".to_string()))
        }
    }
    
    async fn take_screenshot(&self, source_id: Option<&str>) -> Result<ScreenshotData> {
        #[cfg(target_os = "ios")]
        {
            unsafe {
                let ui_application_class = Class::get("UIApplication").unwrap();
                let shared_application: *mut Object = msg_send![ui_application_class, sharedApplication];
                let key_window: *mut Object = msg_send![shared_application, keyWindow];
                
                if key_window.is_null() {
                    return Err(HardwareError::DeviceAccessError("No key window available".to_string()));
                }
                
                // Get the root view controller
                let root_view_controller: *mut Object = msg_send![key_window, rootViewController];
                if root_view_controller.is_null() {
                    return Err(HardwareError::DeviceAccessError("No root view controller available".to_string()));
                }
                
                // Get the view
                let view: *mut Object = msg_send![root_view_controller, view];
                if view.is_null() {
                    return Err(HardwareError::DeviceAccessError("No view available".to_string()));
                }
                
                // Create a UIGraphicsImageRenderer to capture the view
                let ui_graphics_image_renderer_format_class = Class::get("UIGraphicsImageRendererFormat").unwrap();
                let format: *mut Object = msg_send![ui_graphics_image_renderer_format_class, defaultFormat];
                
                let ui_graphics_image_renderer_class = Class::get("UIGraphicsImageRenderer").unwrap();
                let bounds: objc::runtime::NSRect = msg_send![view, bounds];
                let renderer: *mut Object = msg_send![ui_graphics_image_renderer_class, alloc];
                let renderer: *mut Object = msg_send![renderer, initWithBounds:bounds format:format];
                
                // Capture the image
                let image: *mut Object = msg_send![renderer, imageWithActions:^(void *ctx) {
                    let _: () = msg_send![view, drawViewHierarchyInRect:bounds afterScreenUpdates:YES];
                }];
                
                if image.is_null() {
                    return Err(HardwareError::DeviceAccessError("Failed to capture screenshot".to_string()));
                }
                
                // Convert to PNG data
                let ui_image_png_representation: u64 = 0; // UIImagePNGRepresentation
                let data: *mut Object = msg_send![Class::get("UIImagePNGRepresentation").unwrap(), UIImagePNGRepresentation:image];
                
                if data.is_null() {
                    return Err(HardwareError::DeviceAccessError("Failed to convert screenshot to PNG".to_string()));
                }
                
                // Get the bytes
                let bytes: *const u8 = msg_send![data, bytes];
                let length: usize = msg_send![data, length];
                
                let bytes_slice = std::slice::from_raw_parts(bytes, length);
                let base64_data = BASE64.encode(bytes_slice);
                
                let (width, height) = self.get_screen_dimensions();
                
                Ok(ScreenshotData {
                    data: base64_data,
                    width,
                    height,
                    format: "png".to_string(),
                })
            }
        }
        
        #[cfg(not(target_os = "ios"))]
        {
            Err(HardwareError::UnsupportedPlatform("iOS screen recording is not available on this platform".to_string()))
        }
    }
    
    async fn save_screenshot(&self, source_id: Option<&str>, file_path: &str, format: Option<&str>) -> Result<String> {
        let path = Path::new(file_path);
        
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Take a screenshot first
        let screenshot = self.take_screenshot(source_id).await?;
        
        // Decode base64 data
        let bytes = match BASE64.decode(&screenshot.data) {
            Ok(data) => data,
            Err(e) => return Err(HardwareError::DeviceAccessError(format!("Failed to decode screenshot data: {}", e))),
        };
        
        // Write to file
        match fs::write(path, &bytes) {
            Ok(_) => Ok(file_path.to_string()),
            Err(e) => Err(HardwareError::FileSystemError(format!("Failed to save screenshot: {}", e))),
        }
    }
    
    async fn start_recording(&self, source_id: Option<&str>, options: Option<RecordingOptions>) -> Result<String> {
        #[cfg(target_os = "ios")]
        {
            unsafe {
                let replay_kit_class = match Class::get("RPScreenRecorder") {
                    Some(class) => class,
                    None => return Err(HardwareError::DeviceAccessError("ReplayKit not available".to_string())),
                };
                
                let shared_recorder: *mut Object = msg_send![replay_kit_class, sharedRecorder];
                
                // Check if recording is available
                let is_available: bool = msg_send![shared_recorder, isAvailable];
                if !is_available {
                    return Err(HardwareError::DeviceAccessError("Screen recording is not available".to_string()));
                }
                
                // Check if already recording
                let is_recording: bool = msg_send![shared_recorder, isRecording];
                if is_recording {
                    return Err(HardwareError::DeviceAccessError("Already recording screen".to_string()));
                }
                
                // Generate a unique ID for this recording
                let recorder_id = Uuid::new_v4().to_string();
                
                // Get dimensions
                let (width, height) = self.get_screen_dimensions();
                let frame_rate = options.frame_rate.unwrap_or(30);
                
                // Create a semaphore to wait for the async start recording operation
                let semaphore = std::sync::Arc::new(std::sync::Mutex::new(false));
                let semaphore_clone = semaphore.clone();
                let error_message = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
                let error_message_clone = error_message.clone();
                
                // Start recording
                let _: () = msg_send![shared_recorder, startRecordingWithHandler:^(NSError *error) {
                    let mut result = semaphore_clone.lock().unwrap();
                    
                    if !error.is_null() {
                        let description: *mut Object = msg_send![error, localizedDescription];
                        let desc_str = NSString::from_retained_ptr(description);
                        let mut err_msg = error_message_clone.lock().unwrap();
                        *err_msg = desc_str.as_str().to_string();
                        *result = false;
                    } else {
                        *result = true;
                    }
                }];
                
                // Wait for a reasonable amount of time (5 seconds max)
                let start = std::time::Instant::now();
                let timeout = std::time::Duration::from_secs(5);
                
                loop {
                    {
                        let result = semaphore.lock().unwrap();
                        if *result {
                            // Recording started successfully
                            // Create recording instance
                            let instance = RecordingInstance {
                                id: recorder_id.clone(),
                                width,
                                height,
                                frame_rate,
                                start_time: std::time::SystemTime::now(),
                                paused: false,
                                markers: Vec::new(),
                            };
                            
                            // Store in our map
                            self.recording_instances.lock().unwrap().insert(recorder_id.clone(), recorder_id.clone());
                            
                            return Ok(recorder_id);
                        }
                    }
                    
                    if start.elapsed() > timeout {
                        break;
                    }
                    
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
                
                // If we timed out or got an error
                let err_msg = error_message.lock().unwrap();
                if err_msg.is_empty() {
                    Err(HardwareError::DeviceAccessError("Failed to start screen recording (timed out)".to_string()))
                } else {
                    Err(HardwareError::DeviceAccessError(format!("Failed to start screen recording: {}", *err_msg)))
                }
            }
        }
        
        #[cfg(not(target_os = "ios"))]
        {
            Err(HardwareError::UnsupportedPlatform("iOS screen recording is not available on this platform".to_string()))
        }
    }
    
    async fn stop_recording(&self, recording_id: &str, file_path: &str) -> Result<String> {
        #[cfg(target_os = "ios")]
        {
            unsafe {
                let replay_kit_class = match Class::get("RPScreenRecorder") {
                    Some(class) => class,
                    None => return Err(HardwareError::DeviceAccessError("ReplayKit not available".to_string())),
                };
                
                let shared_recorder: *mut Object = msg_send![replay_kit_class, sharedRecorder];
                
                // Check if recording
                let is_recording: bool = msg_send![shared_recorder, isRecording];
                if !is_recording {
                    return Err(HardwareError::DeviceAccessError("Not currently recording screen".to_string()));
                }
                
                // Create a semaphore to wait for the async stop recording operation
                let semaphore = std::sync::Arc::new(std::sync::Mutex::new(false));
                let semaphore_clone = semaphore.clone();
                let error_message = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
                let error_message_clone = error_message.clone();
                
                // Create an NSString for the file path
                let ns_file_path = NSString::from_str(file_path);
                let url_class = Class::get("NSURL").unwrap();
                let file_url: *mut Object = msg_send![url_class, fileURLWithPath:ns_file_path];
                
                // Stop recording and save to file
                let _: () = msg_send![shared_recorder, stopRecordingWithOutputURL:file_url handler:^(RPPreviewViewController *previewViewController, NSError *error) {
                    let mut result = semaphore_clone.lock().unwrap();
                    
                    if !error.is_null() {
                        let description: *mut Object = msg_send![error, localizedDescription];
                        let desc_str = NSString::from_retained_ptr(description);
                        let mut err_msg = error_message_clone.lock().unwrap();
                        *err_msg = desc_str.as_str().to_string();
                        *result = false;
                    } else {
                        *result = true;
                    }
                }];
                
                // Wait for a reasonable amount of time (5 seconds max)
                let start = std::time::Instant::now();
                let timeout = std::time::Duration::from_secs(5);
                
                loop {
                    {
                        let result = semaphore.lock().unwrap();
                        if *result {
                            // Recording stopped successfully
                            // Get recording instance
                            let mut instances = self.recording_instances.lock().unwrap();
                            let instance = match instances.remove(recorder_id) {
                                Some(instance) => instance,
                                None => return Err(HardwareError::InvalidParameter(format!("Recording with ID {} not found", recorder_id))),
                            };
                            
                            // Calculate duration
                            let duration = instance.start_time.elapsed().unwrap_or_default();
                            
                            return Ok(RecordingResult {
                                path: file_path.to_string(),
                                duration: duration.as_secs_f64(),
                                width: instance.width,
                                height: instance.height,
                                frame_rate: instance.frame_rate,
                                format: "mp4".to_string(),
                                markers: instance.markers,
                                success: true,
                            });
                        }
                    }
                    
                    if start.elapsed() > timeout {
                        break;
                    }
                    
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
                
                // If we timed out or got an error
                let err_msg = error_message.lock().unwrap();
                if err_msg.is_empty() {
                    Err(HardwareError::DeviceAccessError("Failed to stop screen recording (timed out)".to_string()))
                } else {
                    Err(HardwareError::DeviceAccessError(format!("Failed to stop screen recording: {}", *err_msg)))
                }
            }
        }
        
        #[cfg(not(target_os = "ios"))]
        {
            Err(HardwareError::UnsupportedPlatform("iOS screen recording is not available on this platform".to_string()))
        }
    }
    
    async fn pause_recording(&self, recorder_id: &str) -> Result<()> {
        #[cfg(target_os = "ios")]
        {
            // ReplayKit doesn't support pausing recordings directly
            // We'll just update our internal state
            let mut instances = self.recording_instances.lock().unwrap();
            if let Some(instance) = instances.get_mut(recorder_id) {
                instance.paused = true;
                Ok(())
            } else {
                Err(HardwareError::InvalidParameter(format!("Recording with ID {} not found", recorder_id)))
            }
        }
        
        #[cfg(not(target_os = "ios"))]
        {
            Err(HardwareError::UnsupportedPlatform("iOS screen recording is not available on this platform".to_string()))
        }
    }
    
    async fn resume_recording(&self, recorder_id: &str) -> Result<()> {
        #[cfg(target_os = "ios")]
        {
            // ReplayKit doesn't support pausing recordings directly
            // We'll just update our internal state
            let mut instances = self.recording_instances.lock().unwrap();
            if let Some(instance) = instances.get_mut(recorder_id) {
                instance.paused = false;
                Ok(())
            } else {
                Err(HardwareError::InvalidParameter(format!("Recording with ID {} not found", recorder_id)))
            }
        }
        
        #[cfg(not(target_os = "ios"))]
        {
            Err(HardwareError::UnsupportedPlatform("iOS screen recording is not available on this platform".to_string()))
        }
    }
    
    async fn add_marker(&self, recorder_id: &str, label: &str) -> Result<()> {
        #[cfg(target_os = "ios")]
        {
            // ReplayKit doesn't support markers directly
            // We'll just update our internal state
            let mut instances = self.recording_instances.lock().unwrap();
            if let Some(instance) = instances.get_mut(recorder_id) {
                let elapsed = instance.start_time.elapsed().unwrap_or_default();
                instance.markers.push(RecordingMarker {
                    time: elapsed.as_secs_f64(),
                    label: label.to_string(),
                });
                Ok(())
            } else {
                Err(HardwareError::InvalidParameter(format!("Recording with ID {} not found", recorder_id)))
            }
        }
        
        #[cfg(not(target_os = "ios"))]
        {
            Err(HardwareError::UnsupportedPlatform("iOS screen recording is not available on this platform".to_string()))
        }
    }
}
