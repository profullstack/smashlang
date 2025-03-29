//! Windows-specific hardware implementations
//!
//! This module provides Windows-specific implementations for hardware interfaces
//! using Windows APIs like Media Foundation, DirectShow, and WinAPI.

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use crate::error::HardwareError;
use crate::Result;
use crate::platform::common::*;

// Windows API imports
use winapi::um::winuser;
use winapi::um::wingdi;
use winapi::shared::windef::{HWND, RECT};
use winapi::shared::minwindef::{DWORD, BOOL, TRUE, FALSE};
use winreg::RegKey;
use winreg::enums::*;

#[cfg(feature = "windows")]
use windows::Win32::Media::MediaFoundation as mf;
#[cfg(feature = "windows")]
use windows::Win32::Media::Audio as audio;
#[cfg(feature = "windows")]
use windows::Win32::Devices::HumanInterfaceDevice as hid;

/// Camera implementation for Windows
pub mod camera {
    use super::*;
    
    #[cfg(feature = "windows")]
    use windows::Win32::Media::MediaFoundation::{MFCreateAttributes, MFEnumDeviceSources};
    
    /// Check if camera hardware is available
    pub fn is_available() -> bool {
        #[cfg(feature = "windows")]
        {
            // Use Media Foundation to check for video capture devices
            unsafe {
                // Initialize Media Foundation
                if let Err(_) = mf::MFStartup(mf::MF_VERSION, 0) {
                    return false;
                }
                
                // Create attribute store
                let mut attributes = None;
                if let Err(_) = MFCreateAttributes(&mut attributes, 1) {
                    mf::MFShutdown().ok();
                    return false;
                }
                
                let attributes = attributes.unwrap();
                
                // Set device type to video capture
                if let Err(_) = attributes.SetGUID(
                    &mf::MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE,
                    &mf::MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_GUID
                ) {
                    mf::MFShutdown().ok();
                    return false;
                }
                
                // Enumerate devices
                let mut devices = None;
                let mut count = 0;
                if let Err(_) = MFEnumDeviceSources(attributes, &mut devices, &mut count) {
                    mf::MFShutdown().ok();
                    return false;
                }
                
                let has_devices = count > 0;
                
                // Cleanup
                mf::MFShutdown().ok();
                
                has_devices
            }
        }
        
        #[cfg(not(feature = "windows"))]
        {
            // Fallback - check registry for video devices
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            if let Ok(devices) = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control\\Class\\{6BDD1FC6-810F-11D0-BEC7-08002BE2092F}") {
                // This registry key contains video capture devices
                if let Ok(subkeys) = devices.enum_keys().collect::<Result<Vec<_>, _>>() {
                    return !subkeys.is_empty();
                }
            }
            
            false
        }
    }
    
    /// Get a list of available camera devices
    pub fn get_devices() -> Result<Vec<CameraDeviceInfo>> {
        let mut devices = Vec::new();
        
        #[cfg(feature = "windows")]
        {
            unsafe {
                // Initialize Media Foundation
                if let Err(e) = mf::MFStartup(mf::MF_VERSION, 0) {
                    return Err(HardwareError::PlatformError(format!("Failed to initialize Media Foundation: {:?}", e)));
                }
                
                // Create attribute store
                let mut attributes = None;
                if let Err(e) = MFCreateAttributes(&mut attributes, 1) {
                    mf::MFShutdown().ok();
                    return Err(HardwareError::PlatformError(format!("Failed to create attributes: {:?}", e)));
                }
                
                let attributes = attributes.unwrap();
                
                // Set device type to video capture
                if let Err(e) = attributes.SetGUID(
                    &mf::MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE,
                    &mf::MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_GUID
                ) {
                    mf::MFShutdown().ok();
                    return Err(HardwareError::PlatformError(format!("Failed to set attribute: {:?}", e)));
                }
                
                // Enumerate devices
                let mut device_array = None;
                let mut count = 0;
                if let Err(e) = MFEnumDeviceSources(attributes, &mut device_array, &mut count) {
                    mf::MFShutdown().ok();
                    return Err(HardwareError::PlatformError(format!("Failed to enumerate devices: {:?}", e)));
                }
                
                if count > 0 && device_array.is_some() {
                    let device_array = device_array.unwrap();
                    
                    for i in 0..count {
                        let device = device_array.GetItem(i);
                        
                        // Get device ID
                        let mut device_id = windows::core::PWSTR::null();
                        let mut id_length = 0;
                        if device.GetString(
                            &mf::MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_SYMBOLIC_LINK,
                            &mut device_id,
                            &mut id_length
                        ).is_ok() {
                            let id = device_id.to_string().unwrap_or_else(|_| format!("camera_{}", i));
                            
                            // Get friendly name
                            let mut friendly_name = windows::core::PWSTR::null();
                            let mut name_length = 0;
                            let label = if device.GetString(
                                &mf::MF_DEVSOURCE_ATTRIBUTE_FRIENDLY_NAME,
                                &mut friendly_name,
                                &mut name_length
                            ).is_ok() {
                                friendly_name.to_string().unwrap_or_else(|_| format!("Camera {}", i))
                            } else {
                                format!("Camera {}", i)
                            };
                            
                            let mut capabilities = Vec::new();
                            capabilities.push("video".to_string());
                            capabilities.push("photo".to_string());
                            
                            devices.push(CameraDeviceInfo {
                                id,
                                label,
                                index: i as usize,
                                capabilities,
                            });
                        }
                    }
                }
                
                // Cleanup
                mf::MFShutdown().ok();
            }
        }
        
        #[cfg(not(feature = "windows"))]
        {
            // Fallback - check registry for video devices
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            if let Ok(devices_key) = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control\\Class\\{6BDD1FC6-810F-11D0-BEC7-08002BE2092F}") {
                if let Ok(subkeys) = devices_key.enum_keys().collect::<Result<Vec<_>, _>>() {
                    for (index, key_name) in subkeys.iter().enumerate() {
                        if let Ok(device_key) = devices_key.open_subkey(key_name) {
                            let id = key_name.clone();
                            let label = device_key.get_value("FriendlyName")
                                .unwrap_or_else(|_| format!("Camera {}", index));
                            
                            let mut capabilities = Vec::new();
                            capabilities.push("video".to_string());
                            capabilities.push("photo".to_string());
                            
                            devices.push(CameraDeviceInfo {
                                id,
                                label,
                                index,
                                capabilities,
                            });
                        }
                    }
                }
            }
        }
        
        Ok(devices)
    }
    
    /// Request permission to use the camera
    /// On Windows, this is typically handled by the OS when the camera is accessed
    pub async fn request_permission() -> Result<bool> {
        // For Windows, we don't typically need explicit permission
        // Just check if cameras are available
        Ok(is_available())
    }
}

/// Microphone implementation for Windows
pub mod microphone {
    use super::*;
    
    #[cfg(feature = "windows")]
    use windows::Win32::Media::Audio::{IMMDeviceEnumerator, MMDeviceEnumerator};
    #[cfg(feature = "windows")]
    use windows::Win32::Media::Audio::eCapture;
    
    /// Check if microphone hardware is available
    pub fn is_available() -> bool {
        #[cfg(feature = "windows")]
        {
            // Use Windows Audio APIs to check for audio capture devices
            unsafe {
                let mut device_enumerator: Option<IMMDeviceEnumerator> = None;
                let result = MMDeviceEnumerator::CoCreateInstance(
                    None,
                    None,
                    audio::CLSCTX_ALL
                );
                
                if let Ok(enumerator) = result {
                    let mut device_collection = None;
                    if enumerator.EnumAudioEndpoints(
                        eCapture,
                        audio::DEVICE_STATE_ACTIVE,
                        &mut device_collection
                    ).is_ok() {
                        if let Some(collection) = device_collection {
                            let mut count = 0;
                            if collection.GetCount(&mut count).is_ok() {
                                return count > 0;
                            }
                        }
                    }
                }
                
                false
            }
        }
        
        #[cfg(not(feature = "windows"))]
        {
            // Fallback - check registry for audio devices
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            if let Ok(devices) = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control\\Class\\{4D36E96C-E325-11CE-BFC1-08002BE10318}") {
                // This registry key contains audio devices
                if let Ok(subkeys) = devices.enum_keys().collect::<Result<Vec<_>, _>>() {
                    return !subkeys.is_empty();
                }
            }
            
            false
        }
    }
    
    /// Get a list of available microphone devices
    pub fn get_devices() -> Result<Vec<MicrophoneDeviceInfo>> {
        let mut devices = Vec::new();
        
        #[cfg(feature = "windows")]
        {
            unsafe {
                let mut device_enumerator: Option<IMMDeviceEnumerator> = None;
                let result = MMDeviceEnumerator::CoCreateInstance(
                    None,
                    None,
                    audio::CLSCTX_ALL
                );
                
                if let Ok(enumerator) = result {
                    let mut device_collection = None;
                    if let Err(e) = enumerator.EnumAudioEndpoints(
                        eCapture,
                        audio::DEVICE_STATE_ACTIVE,
                        &mut device_collection
                    ) {
                        return Err(HardwareError::PlatformError(format!("Failed to enumerate audio endpoints: {:?}", e)));
                    }
                    
                    if let Some(collection) = device_collection {
                        let mut count = 0;
                        if let Err(e) = collection.GetCount(&mut count) {
                            return Err(HardwareError::PlatformError(format!("Failed to get device count: {:?}", e)));
                        }
                        
                        for i in 0..count {
                            let mut device = None;
                            if collection.Item(i, &mut device).is_ok() {
                                if let Some(device) = device {
                                    let mut id_str = None;
                                    if device.GetId(&mut id_str).is_ok() {
                                        if let Some(id_str) = id_str {
                                            let id = id_str.to_string().unwrap_or_else(|_| format!("microphone_{}", i));
                                            
                                            // Get device properties
                                            let mut props = None;
                                            if device.OpenPropertyStore(audio::STGM_READ, &mut props).is_ok() {
                                                if let Some(props) = props {
                                                    let friendly_name_key = audio::PKEY_Device_FriendlyName;
                                                    let mut prop_variant = windows::Win32::System::Com::PROPVARIANT::default();
                                                    
                                                    let label = if props.GetValue(&friendly_name_key, &mut prop_variant).is_ok() {
                                                        // Extract string from PROPVARIANT
                                                        if prop_variant.vt == windows::Win32::System::Variant::VT_LPWSTR as u16 {
                                                            let pwsz = prop_variant.Anonymous.Anonymous.Anonymous.pwszVal;
                                                            if !pwsz.is_null() {
                                                                windows::core::PWSTR(pwsz).to_string().unwrap_or_else(|_| format!("Microphone {}", i))
                                                            } else {
                                                                format!("Microphone {}", i)
                                                            }
                                                        } else {
                                                            format!("Microphone {}", i)
                                                        }
                                                    } else {
                                                        format!("Microphone {}", i)
                                                    };
                                                    
                                                    let mut capabilities = Vec::new();
                                                    capabilities.push("audio".to_string());
                                                    
                                                    devices.push(MicrophoneDeviceInfo {
                                                        id,
                                                        label,
                                                        index: i as usize,
                                                        capabilities,
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
            }
        }
        
        #[cfg(not(feature = "windows"))]
        {
            // Fallback - check registry for audio devices
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            if let Ok(devices_key) = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control\\Class\\{4D36E96C-E325-11CE-BFC1-08002BE10318}") {
                if let Ok(subkeys) = devices_key.enum_keys().collect::<Result<Vec<_>, _>>() {
                    for (index, key_name) in subkeys.iter().enumerate() {
                        if let Ok(device_key) = devices_key.open_subkey(key_name) {
                            // Check if this is a capture device (microphone)
                            let device_type: u32 = device_key.get_value("DeviceType").unwrap_or(0);
                            if device_type == 1 { // 1 is for capture devices
                                let id = key_name.clone();
                                let label = device_key.get_value("FriendlyName")
                                    .unwrap_or_else(|_| format!("Microphone {}", index));
                                
                                let mut capabilities = Vec::new();
                                capabilities.push("audio".to_string());
                                
                                devices.push(MicrophoneDeviceInfo {
                                    id,
                                    label,
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
    
    /// Request permission to use the microphone
    /// On Windows, this is typically handled by the OS when the microphone is accessed
    pub async fn request_permission() -> Result<bool> {
        // For Windows, we don't typically need explicit permission
        // Just check if microphones are available
        Ok(is_available())
    }
}

/// Screen capture implementation for Windows
pub mod screen {
    use super::*;
    use std::mem;
    
    /// Check if screen capture is available
    pub fn is_available() -> bool {
        // On Windows, screen capture is generally available
        // if we're running in a graphical environment
        unsafe {
            let desktop_hwnd = winuser::GetDesktopWindow();
            desktop_hwnd != std::ptr::null_mut()
        }
    }
    
    /// Get a list of available screen sources
    pub fn get_sources() -> Result<Vec<ScreenSourceInfo>> {
        let mut sources = Vec::new();
        
        unsafe {
            // Get primary monitor info
            let primary_monitor = winuser::MonitorFromWindow(
                winuser::GetDesktopWindow(),
                winuser::MONITOR_DEFAULTTOPRIMARY
            );
            
            if !primary_monitor.is_null() {
                let mut monitor_info = winuser::MONITORINFOEXW {
                    monitorInfo: winuser::MONITORINFO {
                        cbSize: mem::size_of::<winuser::MONITORINFOEXW>() as DWORD,
                        rcMonitor: RECT { left: 0, top: 0, right: 0, bottom: 0 },
                        rcWork: RECT { left: 0, top: 0, right: 0, bottom: 0 },
                        dwFlags: 0,
                    },
                    szDevice: [0; 32],
                };
                
                if winuser::GetMonitorInfoW(primary_monitor, &mut monitor_info.monitorInfo as *mut _) != 0 {
                    let width = (monitor_info.monitorInfo.rcMonitor.right - monitor_info.monitorInfo.rcMonitor.left) as u32;
                    let height = (monitor_info.monitorInfo.rcMonitor.bottom - monitor_info.monitorInfo.rcMonitor.top) as u32;
                    let aspect_ratio = format!("{:.2}", width as f32 / height as f32);
                    
                    sources.push(ScreenSourceInfo {
                        id: "screen:primary".to_string(),
                        label: "Primary Monitor".to_string(),
                        source_type: "screen".to_string(),
                        width,
                        height,
                        aspect_ratio,
                        thumbnail: None,
                    });
                }
            }
            
            // Enumerate all monitors
            let mut monitor_index = 0;
            let monitor_enum_proc = Some(monitor_enum_proc);
            
            extern "system" fn monitor_enum_proc(
                monitor: HWND,
                _hdc: winapi::shared::windef::HDC,
                rect: *mut RECT,
                lparam: winapi::shared::minwindef::LPARAM
            ) -> BOOL {
                unsafe {
                    let sources_ptr = lparam as *mut Vec<ScreenSourceInfo>;
                    let sources = &mut *sources_ptr;
                    let index = sources.len();
                    
                    let mut monitor_info = winuser::MONITORINFOEXW {
                        monitorInfo: winuser::MONITORINFO {
                            cbSize: mem::size_of::<winuser::MONITORINFOEXW>() as DWORD,
                            rcMonitor: RECT { left: 0, top: 0, right: 0, bottom: 0 },
                            rcWork: RECT { left: 0, top: 0, right: 0, bottom: 0 },
                            dwFlags: 0,
                        },
                        szDevice: [0; 32],
                    };
                    
                    if winuser::GetMonitorInfoW(monitor, &mut monitor_info.monitorInfo as *mut _) != 0 {
                        // Skip if this is the primary monitor (already added)
                        if monitor_info.monitorInfo.dwFlags & winuser::MONITORINFOF_PRIMARY != 0 {
                            return TRUE;
                        }
                        
                        let rect = *rect;
                        let width = (rect.right - rect.left) as u32;
                        let height = (rect.bottom - rect.top) as u32;
                        let aspect_ratio = format!("{:.2}", width as f32 / height as f32);
                        
                        sources.push(ScreenSourceInfo {
                            id: format!("screen:{}", index),
                            label: format!("Monitor {}", index),
                            source_type: "screen".to_string(),
                            width,
                            height,
                            aspect_ratio,
                            thumbnail: None,
                        });
                    }
                    
                    TRUE
                }
            }
            
            winuser::EnumDisplayMonitors(
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                monitor_enum_proc,
                &mut sources as *mut _ as winapi::shared::minwindef::LPARAM
            );
            
            // Enumerate windows for window capture
            let mut window_index = 0;
            let window_enum_proc = Some(window_enum_proc);
            
            extern "system" fn window_enum_proc(
                hwnd: HWND,
                lparam: winapi::shared::minwindef::LPARAM
            ) -> BOOL {
                unsafe {
                    let sources_ptr = lparam as *mut Vec<ScreenSourceInfo>;
                    let sources = &mut *sources_ptr;
                    
                    // Skip invisible or minimized windows
                    if winuser::IsWindowVisible(hwnd) == 0 || winuser::IsIconic(hwnd) != 0 {
                        return TRUE;
                    }
                    
                    // Get window title
                    let mut title = [0u16; 256];
                    let len = winuser::GetWindowTextW(hwnd, title.as_mut_ptr(), title.len() as i32);
                    
                    if len > 0 {
                        let window_title = String::from_utf16_lossy(&title[..len as usize]);
                        
                        // Get window rect
                        let mut rect = RECT { left: 0, top: 0, right: 0, bottom: 0 };
                        if winuser::GetWindowRect(hwnd, &mut rect) != 0 {
                            let width = (rect.right - rect.left) as u32;
                            let height = (rect.bottom - rect.top) as u32;
                            
                            // Skip tiny windows
                            if width < 50 || height < 50 {
                                return TRUE;
                            }
                            
                            let aspect_ratio = format!("{:.2}", width as f32 / height as f32);
                            let index = sources.len();
                            
                            sources.push(ScreenSourceInfo {
                                id: format!("window:{:p}", hwnd),
                                label: format!("Window: {}", window_title),
                                source_type: "window".to_string(),
                                width,
                                height,
                                aspect_ratio,
                                thumbnail: None,
                            });
                        }
                    }
                    
                    TRUE
                }
            }
            
            winuser::EnumWindows(
                window_enum_proc,
                &mut sources as *mut _ as winapi::shared::minwindef::LPARAM
            );
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
    
    /// Request permission to capture the screen
    /// On Windows, this is typically handled by the OS when screen capture is initiated
    pub fn request_permission() -> Result<bool> {
        // For Windows, we don't typically need explicit permission
        // Just check if we're in a graphical environment
        Ok(is_available())
    }
}
