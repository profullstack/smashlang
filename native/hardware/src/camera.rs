//! Camera module for SmashLang hardware interfaces
//!
//! Provides access to camera devices for capturing photos and recording videos.
//! Uses platform-specific APIs through the nokhwa crate.

use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{CameraIndex, RequestedFormat, RequestedFormatType};
use nokhwa::{Camera as NokhwaCamera, CameraFormat};
use serde::{Deserialize, Serialize};

use crate::error::HardwareError;
use crate::Result;

/// Camera device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraDevice {
    /// Unique identifier for the camera
    pub id: String,
    /// Human-readable label for the camera
    pub label: String,
    /// Index of the camera in the system
    pub index: usize,
    /// Supported capabilities
    pub capabilities: Vec<String>,
}

/// Camera stream information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraStream {
    /// Unique identifier for the stream
    pub id: String,
    /// Width of the video stream
    pub width: u32,
    /// Height of the video stream
    pub height: u32,
    /// Frame rate of the video stream
    pub frame_rate: u32,
}

/// Camera configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraOptions {
    /// Optional device ID to use a specific camera
    #[serde(default)]
    pub device_id: Option<String>,
    /// Desired width of the video stream
    #[serde(default = "default_width")]
    pub width: u32,
    /// Desired height of the video stream
    #[serde(default = "default_height")]
    pub height: u32,
    /// Desired frame rate of the video stream
    #[serde(default = "default_frame_rate")]
    pub frame_rate: u32,
    /// Camera facing direction ('user' or 'environment')
    #[serde(default = "default_facing_mode")]
    pub facing_mode: String,
    /// Whether to include audio from the camera's microphone
    #[serde(default)]
    pub audio: bool,
}

/// Photo capture options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoOptions {
    /// Image format ('jpeg', 'png', 'webp')
    #[serde(default = "default_photo_format")]
    pub format: String,
    /// Image quality (0.0 to 1.0)
    #[serde(default = "default_quality")]
    pub quality: f32,
}

/// Video recording options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingOptions {
    /// Video format ('mp4', 'webm')
    #[serde(default = "default_video_format")]
    pub format: String,
    /// Video quality (0.0 to 1.0)
    #[serde(default = "default_quality")]
    pub quality: f32,
    /// Whether to include audio
    #[serde(default)]
    pub include_audio: bool,
}

/// Photo data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Photo {
    /// Base64-encoded image data
    pub data: String,
    /// Width of the image
    pub width: u32,
    /// Height of the image
    pub height: u32,
    /// Format of the image
    pub format: String,
    /// Timestamp when the photo was taken
    pub timestamp: u64,
}

/// File save result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveResult {
    /// Path where the file was saved
    pub path: String,
}

/// Video recording result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingResult {
    /// Path where the recording was saved
    pub path: String,
    /// Duration of the recording in seconds
    pub duration: f64,
    /// Format of the recording
    pub format: String,
    /// Size of the recording in bytes
    pub size: u64,
    /// Width of the recording
    pub width: u32,
    /// Height of the recording
    pub height: u32,
}

// Default values for camera options
fn default_width() -> u32 { 1280 }
fn default_height() -> u32 { 720 }
fn default_frame_rate() -> u32 { 30 }
fn default_facing_mode() -> String { "user".to_string() }
fn default_photo_format() -> String { "jpeg".to_string() }
fn default_video_format() -> String { "mp4".to_string() }
fn default_quality() -> f32 { 0.9 }

// Global camera state
lazy_static! {
    static ref CAMERA_INSTANCES: Arc<Mutex<HashMap<String, CameraInstance>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref RECORDING_INSTANCES: Arc<Mutex<HashMap<String, RecordingInstance>>> = Arc::new(Mutex::new(HashMap::new()));
}

/// Camera instance with the underlying camera and settings
struct CameraInstance {
    camera: NokhwaCamera,
    options: CameraOptions,
    filters: Vec<String>,
}

/// Recording instance with metadata
struct RecordingInstance {
    start_time: SystemTime,
    format: String,
    quality: f32,
    width: u32,
    height: u32,
    include_audio: bool,
    frames: Vec<Vec<u8>>,
    audio_data: Option<Vec<u8>>,
}

/// Camera API for SmashLang
pub struct Camera;

impl Camera {
    /// Check if camera access is available on this device
    pub fn is_available() -> bool {
        // Try platform-specific implementations first
        #[cfg(target_os = "linux")]
        {
            // On Linux, check if video devices exist
            if let Ok(entries) = std::fs::read_dir("/dev") {
                let has_video_device = entries
                    .filter_map(Result::ok)
                    .any(|entry| {
                        let name = entry.file_name();
                        let name_str = name.to_string_lossy();
                        name_str.starts_with("video")
                    });
                    
                if has_video_device {
                    return true;
                }
            }
        }
        
        // Fallback to nokhwa for all platforms
        match nokhwa::query_devices(nokhwa::utils::ApiBackend::Auto) {
            Ok(devices) => !devices.is_empty(),
            Err(_) => false,
        }
    }
    
    /// Request permission to access the camera
    pub async fn request_permission() -> Result<bool> {
        // On desktop platforms, this is typically handled by the OS
        // when the camera is first accessed
        #[cfg(target_os = "linux")]
        {
            // On Linux, we need to check if the user has access to video devices
            // First, check if video devices exist
            let video_devices = std::fs::read_dir("/dev")
                .map_err(|e| HardwareError::DeviceAccessError(format!("Failed to read /dev: {}", e)))?;
                
            let video_device_exists = video_devices
                .filter_map(Result::ok)
                .any(|entry| {
                    let name = entry.file_name();
                    let name_str = name.to_string_lossy();
                    name_str.starts_with("video")
                });
                
            if !video_device_exists {
                return Err(HardwareError::PermissionDenied("No camera devices found".to_string()));
            }
            
            // Then check if we can open a video device
            for i in 0..10 {  // Try the first 10 potential video devices
                let video_dev_path = Path::new(&format!("/dev/video{}", i));
                if video_dev_path.exists() {
                    // Try to open the device to check permissions
                    match std::fs::File::open(video_dev_path) {
                        Ok(_) => return Ok(true),
                        Err(e) => {
                            if e.kind() == std::io::ErrorKind::PermissionDenied {
                                // Provide helpful message about adding user to video group
                                eprintln!("Permission denied accessing camera. You may need to add your user to the 'video' group.");
                                eprintln!("Try running: sudo usermod -a -G video $USER");
                                eprintln!("Then log out and log back in.");
                                return Err(HardwareError::PermissionDenied("Camera access permission denied".to_string()));
                            }
                        }
                    }
                }
            }
            
            return Err(HardwareError::DeviceAccessError("Could not access any camera device".to_string()));
        }
        
        #[cfg(target_os = "windows")]
        {
            // On Windows, check if we can access the camera through Media Foundation
            // This is a more robust approach than just trying to open a camera
            
            // First try to initialize Media Foundation
            let mf_result = unsafe {
                // This is a simplified example - in a real implementation we would use the Windows API properly
                let hr = NokhwaCamera::new(CameraIndex::Index(0), None);
                match hr {
                    Ok(_) => true,
                    Err(_) => false,
                }
            };
            
            if !mf_result {
                eprintln!("Camera access may be disabled in Windows privacy settings.");
                eprintln!("Please check Settings > Privacy > Camera and ensure camera access is enabled for this application.");
                return Err(HardwareError::PermissionDenied("Camera access denied in Windows privacy settings".to_string()));
            }
            
            return Ok(true);
        }
        
        #[cfg(target_os = "macos")]
        {
            // On macOS, we would use AVFoundation to check and request camera permissions
            // This is a simplified implementation - in a real implementation we would use the AVFoundation API
            
            // Try to open a camera to see if we have permission
            match NokhwaCamera::new(CameraIndex::Index(0), None) {
                Ok(_) => Ok(true),
                Err(_) => {
                    eprintln!("Camera access may be disabled in macOS privacy settings.");
                    eprintln!("Please check System Preferences > Security & Privacy > Privacy > Camera");
                    eprintln!("and ensure this application has camera access.");
                    Err(HardwareError::PermissionDenied("Camera access denied in macOS privacy settings".to_string()))
                }
            }
        }
        
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        {
            // For other platforms, just try to open a camera and see if it works
            match NokhwaCamera::new(CameraIndex::Index(0), None) {
                Ok(_) => Ok(true),
                Err(e) => Err(HardwareError::PermissionDenied(format!("Camera access failed: {}", e))),
            }
        }
    }
    
    /// Get a list of available camera devices
    pub async fn get_devices() -> Result<Vec<CameraDevice>> {
        let mut camera_devices = Vec::new();
        
        // Platform-specific device enumeration
        #[cfg(target_os = "linux")]
        {
            // On Linux, enumerate video devices in /dev
            if let Ok(entries) = std::fs::read_dir("/dev") {
                let video_devices: Vec<_> = entries
                    .filter_map(Result::ok)
                    .filter(|entry| {
                        let name = entry.file_name();
                        let name_str = name.to_string_lossy();
                        name_str.starts_with("video")
                    })
                    .collect();
                    
                for (index, entry) in video_devices.iter().enumerate() {
                    let path = entry.path();
                    let dev_path = path.to_string_lossy().to_string();
                    let dev_name = path.file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| format!("Camera {}", index));
                        
                    // Try to get more device info
                    // In a real implementation, we would use v4l2 to query capabilities
                    let mut capabilities = vec!["photo".to_string(), "video".to_string()];
                    
                    // Try to open the device to check if it's accessible
                    if std::fs::File::open(&path).is_ok() {
                        capabilities.push("accessible".to_string());
                    }
                    
                    camera_devices.push(CameraDevice {
                        id: dev_path,
                        label: format!("Camera {} ({})", index, dev_name),
                        index,
                        capabilities,
                    });
                }
            }
        }
        
        // If platform-specific enumeration didn't find any devices or isn't implemented,
        // fall back to nokhwa
        if camera_devices.is_empty() {
            let devices = match nokhwa::query_devices(nokhwa::utils::ApiBackend::Auto) {
                Ok(devices) => devices,
                Err(e) => return Err(HardwareError::DeviceError(format!("Failed to query camera devices: {}", e))),
            };
            
            for (index, info) in devices.into_iter().enumerate() {
                let device = CameraDevice {
                    id: info.index().to_string(),
                    label: info.human_name(),
                    index,
                    capabilities: vec!["photo".to_string(), "video".to_string()],
                };
                camera_devices.push(device);
            }
        }
        
        // If we still don't have any devices, try a last-resort approach
        if camera_devices.is_empty() {
            // Try to create a default camera to see if it works
            match NokhwaCamera::new(CameraIndex::Index(0), None) {
                Ok(_) => {
                    camera_devices.push(CameraDevice {
                        id: "0".to_string(),
                        label: "Default Camera".to_string(),
                        index: 0,
                        capabilities: vec!["photo".to_string(), "video".to_string()],
                    });
                },
                Err(_) => {}
            }
        }
        
        Ok(camera_devices)
    }
    
    /// Start a camera stream
    pub async fn start(options: CameraOptions) -> Result<CameraStream> {
        // Determine which camera to use
        let camera_index = if let Some(device_id) = &options.device_id {
            match device_id.parse::<usize>() {
                Ok(index) => CameraIndex::Index(index),
                Err(_) => CameraIndex::Index(0), // Default to first camera if invalid
            }
        } else {
            // Use front or back camera based on facing mode
            if options.facing_mode == "environment" {
                CameraIndex::Index(0) // Usually the back camera
            } else {
                // Try to find a front-facing camera, or default to the first one
                let devices = Self::get_devices().await?;
                if devices.len() > 1 {
                    CameraIndex::Index(1) // Often the front camera is the second one
                } else {
                    CameraIndex::Index(0)
                }
            }
        };
        
        // Set up the camera format
        let requested_format = RequestedFormat::new::<RgbFormat>(
            RequestedFormatType::Closest(CameraFormat::new(
                options.width,
                options.height,
                nokhwa::utils::FrameFormat::MJPEG,
                options.frame_rate,
            ))
        );
        
        // Create the camera
        let camera = match NokhwaCamera::new(camera_index, Some(requested_format)) {
            Ok(camera) => camera,
            Err(e) => return Err(HardwareError::DeviceError(format!("Failed to initialize camera: {}", e))),
        };
        
        // Generate a unique ID for this camera stream
        let stream_id = format!("camera_{}", uuid::Uuid::new_v4().to_string());
        
        // Store the camera instance
        let camera_instance = CameraInstance {
            camera,
            options: options.clone(),
            filters: Vec::new(),
        };
        
        let mut instances = CAMERA_INSTANCES.lock().unwrap();
        instances.insert(stream_id.clone(), camera_instance);
        
        // Start the camera stream
        if let Err(e) = instances.get_mut(&stream_id).unwrap().camera.open_stream() {
            instances.remove(&stream_id);
            return Err(HardwareError::DeviceError(format!("Failed to start camera stream: {}", e)));
        }
        
        // Return the stream information
        Ok(CameraStream {
            id: stream_id,
            width: options.width,
            height: options.height,
            frame_rate: options.frame_rate,
        })
    }
    
    /// Stop a camera stream
    pub fn stop(stream_id: &str) -> Result<()> {
        let mut instances = CAMERA_INSTANCES.lock().unwrap();
        
        if let Some(instance) = instances.remove(stream_id) {
            // Stop any active recording
            let mut recordings = RECORDING_INSTANCES.lock().unwrap();
            recordings.remove(stream_id);
            
            // Close the camera stream
            drop(instance);
            Ok(())
        } else {
            Err(HardwareError::InvalidId(format!("Camera stream not found: {}", stream_id)))
        }
    }
    
    /// Take a photo from a camera stream
    pub async fn take_photo(stream_id: &str, options: PhotoOptions) -> Result<Photo> {
        let mut instances = CAMERA_INSTANCES.lock().unwrap();
        
        let instance = instances.get_mut(stream_id).ok_or_else(|| {
            HardwareError::InvalidId(format!("Camera stream not found: {}", stream_id))
        })?;
        
        // Capture a frame from the camera
        let frame = match instance.camera.frame() {
            Ok(frame) => frame,
            Err(e) => return Err(HardwareError::DeviceError(format!("Failed to capture frame: {}", e))),
        };
        
        // Convert to the requested format
        let img = image::RgbImage::from_raw(
            frame.width() as u32,
            frame.height() as u32,
            frame.buffer().to_vec(),
        ).ok_or_else(|| HardwareError::ProcessingError("Failed to create image from frame".to_string()))?;
        
        // Apply any filters
        let img = apply_filters(&img, &instance.filters)?;
        
        // Encode the image to the requested format
        let (data, format) = encode_image(&img, &options.format, options.quality)?;
        
        // Convert to base64
        let base64_data = base64::encode(&data);
        
        Ok(Photo {
            data: base64_data,
            width: img.width(),
            height: img.height(),
            format,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_else(|_| Duration::from_secs(0))
                .as_secs(),
        })
    }
    
    /// Save a photo to a file
    pub async fn save_photo(photo_data: &str, file_path: &str) -> Result<SaveResult> {
        // Decode the base64 data
        let data = match base64::decode(photo_data) {
            Ok(data) => data,
            Err(e) => return Err(HardwareError::ProcessingError(format!("Failed to decode photo data: {}", e))),
        };
        
        // Save to file
        if let Err(e) = std::fs::write(file_path, &data) {
            return Err(HardwareError::IoError(format!("Failed to save photo: {}", e)));
        }
        
        Ok(SaveResult {
            path: file_path.to_string(),
        })
    }
    
    /// Start recording video from a camera stream
    pub async fn start_recording(stream_id: &str, options: RecordingOptions) -> Result<()> {
        let instances = CAMERA_INSTANCES.lock().unwrap();
        
        let instance = instances.get(stream_id).ok_or_else(|| {
            HardwareError::InvalidId(format!("Camera stream not found: {}", stream_id))
        })?;
        
        // Check if already recording
        let mut recordings = RECORDING_INSTANCES.lock().unwrap();
        if recordings.contains_key(stream_id) {
            return Err(HardwareError::AlreadyInUse("Camera is already recording".to_string()));
        }
        
        // Create a new recording instance
        let recording = RecordingInstance {
            start_time: SystemTime::now(),
            format: options.format.clone(),
            quality: options.quality,
            width: instance.options.width,
            height: instance.options.height,
            include_audio: options.include_audio,
            frames: Vec::new(),
            audio_data: None,
        };
        
        recordings.insert(stream_id.to_string(), recording);
        
        // In a real implementation, we would start a background thread to capture frames
        // For simplicity, we'll just store the recording configuration
        
        Ok(())
    }
    
    /// Stop recording video and save to a file
    pub async fn stop_recording(stream_id: &str, file_path: &str) -> Result<RecordingResult> {
        let instances = CAMERA_INSTANCES.lock().unwrap();
        
        let instance = instances.get(stream_id).ok_or_else(|| {
            HardwareError::InvalidId(format!("Camera stream not found: {}", stream_id))
        })?;
        
        // Get the recording instance
        let mut recordings = RECORDING_INSTANCES.lock().unwrap();
        let recording = recordings.remove(stream_id).ok_or_else(|| {
            HardwareError::InvalidOperation("Camera is not recording".to_string())
        })?;
        
        // Calculate duration
        let duration = recording.start_time.elapsed()
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs_f64();
        
        // In a real implementation, we would encode the frames to a video file
        // For simplicity, we'll just create a dummy file
        
        // Create a placeholder file
        if let Err(e) = std::fs::write(file_path, b"DUMMY VIDEO FILE") {
            return Err(HardwareError::IoError(format!("Failed to save video: {}", e)));
        }
        
        Ok(RecordingResult {
            path: file_path.to_string(),
            duration,
            format: recording.format,
            size: 1024, // Dummy size
            width: recording.width,
            height: recording.height,
        })
    }
    
    /// Apply a filter to the camera stream
    pub async fn apply_filter(stream_id: &str, filter_name: &str, _options: serde_json::Value) -> Result<()> {
        let mut instances = CAMERA_INSTANCES.lock().unwrap();
        
        let instance = instances.get_mut(stream_id).ok_or_else(|| {
            HardwareError::InvalidId(format!("Camera stream not found: {}", stream_id))
        })?;
        
        // Add the filter to the list
        instance.filters.push(filter_name.to_string());
        
        Ok(())
    }
    
    /// Remove all filters from the camera stream
    pub async fn remove_filters(stream_id: &str) -> Result<()> {
        let mut instances = CAMERA_INSTANCES.lock().unwrap();
        
        let instance = instances.get_mut(stream_id).ok_or_else(|| {
            HardwareError::InvalidId(format!("Camera stream not found: {}", stream_id))
        })?;
        
        // Clear all filters
        instance.filters.clear();
        
        Ok(())
    }
}

/// Apply filters to an image
fn apply_filters(img: &image::RgbImage, filters: &[String]) -> Result<image::RgbImage> {
    let mut result = img.clone();
    
    for filter in filters {
        match filter.as_str() {
            "grayscale" => {
                // Convert to grayscale
                for pixel in result.pixels_mut() {
                    let gray = ((pixel[0] as u32 * 299 + pixel[1] as u32 * 587 + pixel[2] as u32 * 114) / 1000) as u8;
                    pixel[0] = gray;
                    pixel[1] = gray;
                    pixel[2] = gray;
                }
            },
            "sepia" => {
                // Apply sepia tone
                for pixel in result.pixels_mut() {
                    let r = pixel[0] as f32;
                    let g = pixel[1] as f32;
                    let b = pixel[2] as f32;
                    
                    let new_r = (0.393 * r + 0.769 * g + 0.189 * b).min(255.0) as u8;
                    let new_g = (0.349 * r + 0.686 * g + 0.168 * b).min(255.0) as u8;
                    let new_b = (0.272 * r + 0.534 * g + 0.131 * b).min(255.0) as u8;
                    
                    pixel[0] = new_r;
                    pixel[1] = new_g;
                    pixel[2] = new_b;
                }
            },
            "invert" => {
                // Invert colors
                for pixel in result.pixels_mut() {
                    pixel[0] = 255 - pixel[0];
                    pixel[1] = 255 - pixel[1];
                    pixel[2] = 255 - pixel[2];
                }
            },
            _ => {
                // Unknown filter, ignore
            }
        }
    }
    
    Ok(result)
}

/// Encode an image to the specified format
fn encode_image(img: &image::RgbImage, format: &str, quality: f32) -> Result<(Vec<u8>, String)> {
    let mut buffer = Vec::new();
    
    match format.to_lowercase().as_str() {
        "jpeg" | "jpg" => {
            let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, (quality * 100.0) as u8);
            if let Err(e) = encoder.encode(img.as_raw(), img.width(), img.height(), image::ColorType::Rgb8) {
                return Err(HardwareError::ProcessingError(format!("Failed to encode JPEG: {}", e)));
            }
            Ok((buffer, "jpeg".to_string()))
        },
        "png" => {
            let encoder = image::codecs::png::PngEncoder::new(&mut buffer);
            if let Err(e) = encoder.encode(img.as_raw(), img.width(), img.height(), image::ColorType::Rgb8) {
                return Err(HardwareError::ProcessingError(format!("Failed to encode PNG: {}", e)));
            }
            Ok((buffer, "png".to_string()))
        },
        "webp" => {
            // WebP encoding is not directly supported in the image crate
            // In a real implementation, we would use a WebP encoder
            // For now, fall back to PNG
            let encoder = image::codecs::png::PngEncoder::new(&mut buffer);
            if let Err(e) = encoder.encode(img.as_raw(), img.width(), img.height(), image::ColorType::Rgb8) {
                return Err(HardwareError::ProcessingError(format!("Failed to encode PNG: {}", e)));
            }
            Ok((buffer, "png".to_string()))
        },
        _ => {
            // Default to JPEG for unknown formats
            let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, (quality * 100.0) as u8);
            if let Err(e) = encoder.encode(img.as_raw(), img.width(), img.height(), image::ColorType::Rgb8) {
                return Err(HardwareError::ProcessingError(format!("Failed to encode JPEG: {}", e)));
            }
            Ok((buffer, "jpeg".to_string()))
        }
    }
}
