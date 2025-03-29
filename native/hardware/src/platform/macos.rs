//! macOS-specific hardware implementations
//!
//! This module provides macOS-specific implementations for hardware interfaces
//! using native macOS APIs and libraries like AVFoundation and Core Media.

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use core_foundation::base::{CFGetTypeID, CFTypeRef};
use core_foundation::string::{CFString, CFStringRef};
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
use core_foundation::array::{CFArray, CFArrayRef};

use crate::error::HardwareError;
use crate::Result;
use crate::platform::common::*;

#[cfg(feature = "core-graphics")]
use core_graphics::display::{CGDisplay, CGDisplayBounds, CGMainDisplayID};

/// Camera implementation for macOS
pub mod camera {
    use super::*;
    use objc::{class, msg_send, sel, sel_impl};
    use objc::runtime::{Class, Object};
    
    #[cfg(feature = "avfoundation")]
    use avfoundation::av_capture_device as avcap;
    
    /// Check if camera hardware is available
    pub fn is_available() -> bool {
        #[cfg(feature = "avfoundation")]
        {
            // Use AVFoundation to check for camera devices
            let devices = avcap::devices_with_media_type(avcap::MediaType::Video);
            !devices.is_empty()
        }
        
        #[cfg(not(feature = "avfoundation"))]
        {
            // Fallback using Objective-C runtime
            unsafe {
                let av_capture_device_class = Class::get("AVCaptureDevice").unwrap();
                let devices: *mut Object = msg_send![av_capture_device_class, devicesWithMediaType:"vide"];
                let count: usize = msg_send![devices, count];
                count > 0
            }
        }
    }
    
    /// Get a list of available camera devices
    pub fn get_devices() -> Result<Vec<CameraDeviceInfo>> {
        let mut devices = Vec::new();
        
        #[cfg(feature = "avfoundation")]
        {
            let av_devices = avcap::devices_with_media_type(avcap::MediaType::Video);
            
            for (index, device) in av_devices.iter().enumerate() {
                let unique_id = device.unique_id().unwrap_or_else(|| format!("camera_{}", index));
                let localized_name = device.localized_name().unwrap_or_else(|| format!("Camera {}", index));
                
                let mut capabilities = Vec::new();
                capabilities.push("video".to_string());
                
                // Check if the device supports photos
                if device.supports_media_type(avcap::MediaType::Video) {
                    capabilities.push("photo".to_string());
                }
                
                // Check if the device has a microphone
                if device.supports_media_type(avcap::MediaType::Audio) {
                    capabilities.push("audio".to_string());
                }
                
                devices.push(CameraDeviceInfo {
                    id: unique_id,
                    label: localized_name,
                    index,
                    capabilities,
                });
            }
        }
        
        #[cfg(not(feature = "avfoundation"))]
        {
            // Fallback using Objective-C runtime
            unsafe {
                let av_capture_device_class = Class::get("AVCaptureDevice").unwrap();
                let av_devices: *mut Object = msg_send![av_capture_device_class, devicesWithMediaType:"vide"];
                let count: usize = msg_send![av_devices, count];
                
                for index in 0..count {
                    let device: *mut Object = msg_send![av_devices, objectAtIndex:index];
                    
                    let unique_id_nsstring: *mut Object = msg_send![device, uniqueID];
                    let localized_name_nsstring: *mut Object = msg_send![device, localizedName];
                    
                    let unique_id_cstr: *const i8 = msg_send![unique_id_nsstring, UTF8String];
                    let localized_name_cstr: *const i8 = msg_send![localized_name_nsstring, UTF8String];
                    
                    let unique_id = if !unique_id_cstr.is_null() {
                        std::ffi::CStr::from_ptr(unique_id_cstr).to_string_lossy().into_owned()
                    } else {
                        format!("camera_{}", index)
                    };
                    
                    let localized_name = if !localized_name_cstr.is_null() {
                        std::ffi::CStr::from_ptr(localized_name_cstr).to_string_lossy().into_owned()
                    } else {
                        format!("Camera {}", index)
                    };
                    
                    let mut capabilities = Vec::new();
                    capabilities.push("video".to_string());
                    capabilities.push("photo".to_string());
                    
                    devices.push(CameraDeviceInfo {
                        id: unique_id,
                        label: localized_name,
                        index,
                        capabilities,
                    });
                }
            }
        }
        
        Ok(devices)
    }
    
    /// Request permission to use the camera
    pub async fn request_permission() -> Result<bool> {
        // On macOS 10.14+, the system will automatically prompt for permission
        // when the camera is accessed for the first time
        // We can check the current authorization status
        
        #[cfg(feature = "avfoundation")]
        {
            use avfoundation::av_capture_device as avcap;
            
            match avcap::authorization_status_for_media_type(avcap::MediaType::Video) {
                avcap::AuthorizationStatus::Authorized => Ok(true),
                avcap::AuthorizationStatus::NotDetermined => {
                    // Request authorization (this will show the system prompt)
                    let authorized = avcap::request_access_for_media_type(avcap::MediaType::Video).await;
                    Ok(authorized)
                },
                _ => Ok(false),
            }
        }
        
        #[cfg(not(feature = "avfoundation"))]
        {
            // Fallback - just check if cameras are available
            // The system will prompt for permission when we try to use them
            Ok(is_available())
        }
    }
}

/// Microphone implementation for macOS
pub mod microphone {
    use super::*;
    use objc::{class, msg_send, sel, sel_impl};
    use objc::runtime::{Class, Object};
    
    #[cfg(feature = "avfoundation")]
    use avfoundation::av_capture_device as avcap;
    
    /// Check if microphone hardware is available
    pub fn is_available() -> bool {
        #[cfg(feature = "avfoundation")]
        {
            // Use AVFoundation to check for audio devices
            let devices = avcap::devices_with_media_type(avcap::MediaType::Audio);
            !devices.is_empty()
        }
        
        #[cfg(not(feature = "avfoundation"))]
        {
            // Fallback using Objective-C runtime
            unsafe {
                let av_capture_device_class = Class::get("AVCaptureDevice").unwrap();
                let devices: *mut Object = msg_send![av_capture_device_class, devicesWithMediaType:"soun"];
                let count: usize = msg_send![devices, count];
                count > 0
            }
        }
    }
    
    /// Get a list of available microphone devices
    pub fn get_devices() -> Result<Vec<MicrophoneDeviceInfo>> {
        let mut devices = Vec::new();
        
        #[cfg(feature = "avfoundation")]
        {
            let av_devices = avcap::devices_with_media_type(avcap::MediaType::Audio);
            
            for (index, device) in av_devices.iter().enumerate() {
                let unique_id = device.unique_id().unwrap_or_else(|| format!("microphone_{}", index));
                let localized_name = device.localized_name().unwrap_or_else(|| format!("Microphone {}", index));
                
                let mut capabilities = Vec::new();
                capabilities.push("audio".to_string());
                
                devices.push(MicrophoneDeviceInfo {
                    id: unique_id,
                    label: localized_name,
                    index,
                    capabilities,
                });
            }
        }
        
        #[cfg(not(feature = "avfoundation"))]
        {
            // Fallback using Objective-C runtime
            unsafe {
                let av_capture_device_class = Class::get("AVCaptureDevice").unwrap();
                let av_devices: *mut Object = msg_send![av_capture_device_class, devicesWithMediaType:"soun"];
                let count: usize = msg_send![av_devices, count];
                
                for index in 0..count {
                    let device: *mut Object = msg_send![av_devices, objectAtIndex:index];
                    
                    let unique_id_nsstring: *mut Object = msg_send![device, uniqueID];
                    let localized_name_nsstring: *mut Object = msg_send![device, localizedName];
                    
                    let unique_id_cstr: *const i8 = msg_send![unique_id_nsstring, UTF8String];
                    let localized_name_cstr: *const i8 = msg_send![localized_name_nsstring, UTF8String];
                    
                    let unique_id = if !unique_id_cstr.is_null() {
                        std::ffi::CStr::from_ptr(unique_id_cstr).to_string_lossy().into_owned()
                    } else {
                        format!("microphone_{}", index)
                    };
                    
                    let localized_name = if !localized_name_cstr.is_null() {
                        std::ffi::CStr::from_ptr(localized_name_cstr).to_string_lossy().into_owned()
                    } else {
                        format!("Microphone {}", index)
                    };
                    
                    let mut capabilities = Vec::new();
                    capabilities.push("audio".to_string());
                    
                    devices.push(MicrophoneDeviceInfo {
                        id: unique_id,
                        label: localized_name,
                        index,
                        capabilities,
                    });
                }
            }
        }
        
        Ok(devices)
    }
    
    /// Request permission to use the microphone
    pub async fn request_permission() -> Result<bool> {
        // On macOS 10.14+, the system will automatically prompt for permission
        // when the microphone is accessed for the first time
        // We can check the current authorization status
        
        #[cfg(feature = "avfoundation")]
        {
            use avfoundation::av_capture_device as avcap;
            
            match avcap::authorization_status_for_media_type(avcap::MediaType::Audio) {
                avcap::AuthorizationStatus::Authorized => Ok(true),
                avcap::AuthorizationStatus::NotDetermined => {
                    // Request authorization (this will show the system prompt)
                    let authorized = avcap::request_access_for_media_type(avcap::MediaType::Audio).await;
                    Ok(authorized)
                },
                _ => Ok(false),
            }
        }
        
        #[cfg(not(feature = "avfoundation"))]
        {
            // Fallback - just check if microphones are available
            // The system will prompt for permission when we try to use them
            Ok(is_available())
        }
    }
}

/// Screen capture implementation for macOS
pub mod screen {
    use super::*;
    
    /// Check if screen capture is available
    pub fn is_available() -> bool {
        #[cfg(feature = "core-graphics")]
        {
            // Check if we can access the main display
            let main_display_id = CGMainDisplayID();
            main_display_id != 0
        }
        
        #[cfg(not(feature = "core-graphics"))]
        {
            // Fallback check - just assume it's available on macOS
            true
        }
    }
    
    /// Get a list of available screen sources
    pub fn get_sources() -> Result<Vec<ScreenSourceInfo>> {
        let mut sources = Vec::new();
        
        #[cfg(feature = "core-graphics")]
        {
            // Get all active displays
            let online_displays = CGDisplay::active_displays().unwrap_or_default();
            
            for (i, display_id) in online_displays.iter().enumerate() {
                let display = CGDisplay::new(*display_id);
                let bounds = CGDisplayBounds(*display_id);
                
                let width = bounds.size.width as u32;
                let height = bounds.size.height as u32;
                let aspect_ratio = format!("{:.2}", width as f32 / height as f32);
                
                // Check if this is the main display
                let is_main = *display_id == CGMainDisplayID();
                let label = if is_main {
                    format!("Main Display ({})", i)
                } else {
                    format!("Display {}", i)
                };
                
                sources.push(ScreenSourceInfo {
                    id: format!("display:{}", display_id),
                    label,
                    source_type: "screen".to_string(),
                    width,
                    height,
                    aspect_ratio,
                    thumbnail: None,
                });
            }
        }
        
        // If no sources were found or Core Graphics is not available, add a generic screen source
        if sources.is_empty() {
            sources.push(ScreenSourceInfo {
                id: "screen:0".to_string(),
                label: "Main Display".to_string(),
                source_type: "screen".to_string(),
                width: 1920, // Default assumption
                height: 1080, // Default assumption
                aspect_ratio: "1.78".to_string(),
                thumbnail: None,
            });
        }
        
        Ok(sources)
    }
    
    /// Request permission to capture the screen
    pub fn request_permission() -> Result<bool> {
        // On macOS 10.15+, the system will automatically prompt for permission
        // when screen recording is accessed for the first time
        // We can't programmatically check the current authorization status for screen recording
        // So we'll just return true and let the system handle the permission prompt
        Ok(true)
    }
}
