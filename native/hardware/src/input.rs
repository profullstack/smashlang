//! Input module for SmashLang hardware interfaces
//!
//! Provides capabilities for keyboard, mouse, and touch input across platforms.
//! Uses platform-specific APIs for desktop (Windows, macOS, Linux) and mobile (Android, iOS) platforms.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use async_trait::async_trait;

use crate::error::HardwareError;
use crate::Result;
use crate::platform;
use crate::platform::common::InputDevice;

/// Key code information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyCode {
    /// Platform-independent key code
    pub code: String,
    /// Key name (e.g., "a", "enter", "shift")
    pub key: String,
    /// Key location (e.g., "standard", "left", "right")
    pub location: String,
}

/// Mouse button information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseButton {
    /// Button index
    pub button: u8,
    /// Button name (e.g., "left", "right", "middle")
    pub name: String,
}

/// Touch point information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TouchPoint {
    /// Touch identifier
    pub id: u32,
    /// X coordinate
    pub x: f32,
    /// Y coordinate
    pub y: f32,
    /// Touch force (0.0 to 1.0, if available)
    pub force: Option<f32>,
    /// Touch radius (if available)
    pub radius: Option<f32>,
}

/// Input event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputEventType {
    /// Key down event
    KeyDown(KeyCode),
    /// Key up event
    KeyUp(KeyCode),
    /// Mouse move event
    MouseMove { x: f32, y: f32 },
    /// Mouse down event
    MouseDown { button: MouseButton, x: f32, y: f32 },
    /// Mouse up event
    MouseUp { button: MouseButton, x: f32, y: f32 },
    /// Mouse wheel event
    MouseWheel { delta_x: f32, delta_y: f32 },
    /// Touch start event
    TouchStart(Vec<TouchPoint>),
    /// Touch move event
    TouchMove(Vec<TouchPoint>),
    /// Touch end event
    TouchEnd(Vec<TouchPoint>),
    /// Touch cancel event
    TouchCancel(Vec<TouchPoint>),
}

/// Input event information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputEvent {
    /// Event type
    pub event_type: InputEventType,
    /// Timestamp in milliseconds
    pub timestamp: u64,
    /// Modifier keys state (shift, ctrl, alt, meta)
    pub modifiers: HashMap<String, bool>,
}

/// Input API for SmashLang
pub struct Input;

impl Input {
    /// Check if input device is available
    pub fn is_available(device_type: &str) -> bool {
        let input_device = create_input_device();
        input_device.is_available(device_type)
    }
    
    /// Register for input events
    pub async fn register_events(device_types: Vec<String>) -> Result<String> {
        let input_device = create_input_device();
        input_device.register_events(device_types).await
    }
    
    /// Unregister from input events
    pub fn unregister_events(registration_id: &str) -> Result<bool> {
        let input_device = create_input_device();
        input_device.unregister_events(registration_id)
    }
    
    /// Simulate input event
    pub async fn simulate_input(event: InputEvent) -> Result<bool> {
        let input_device = create_input_device();
        input_device.simulate_input(event).await
    }
    
    /// Get current keyboard state
    pub fn get_keyboard_state() -> Result<HashMap<String, bool>> {
        let input_device = create_input_device();
        input_device.get_keyboard_state()
    }
    
    /// Get current mouse position
    pub fn get_mouse_position() -> Result<(f32, f32)> {
        let input_device = create_input_device();
        input_device.get_mouse_position()
    }
    
    /// Get current touch points
    pub fn get_touch_points() -> Result<Vec<TouchPoint>> {
        let input_device = create_input_device();
        input_device.get_touch_points()
    }
}

/// Create a platform-specific input device implementation
fn create_input_device() -> Box<dyn InputDevice> {
    #[cfg(target_os = "android")]
    {
        Box::new(platform::android::AndroidInputDevice::new())
    }
    #[cfg(target_os = "ios")]
    {
        Box::new(platform::ios::IOSInputDevice::new())
    }
    #[cfg(target_os = "windows")]
    {
        Box::new(platform::windows::WindowsInputDevice::new())
    }
    #[cfg(target_os = "macos")]
    {
        Box::new(platform::macos::MacOSInputDevice::new())
    }
    #[cfg(target_os = "linux")]
    {
        Box::new(platform::linux::LinuxInputDevice::new())
    }
    #[cfg(not(any(target_os = "android", target_os = "ios", target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Box::new(platform::common::DefaultInputDevice::new())
    }
}
