//! Gamepad module for SmashLang hardware interfaces
//!
//! Provides access to gamepad and game controller devices.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::error::HardwareError;
use crate::Result;

/// Gamepad device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamepadDevice {
    /// Unique identifier for the device
    pub id: String,
    /// Human-readable name for the device
    pub name: String,
    /// Index of the gamepad
    pub index: u32,
    /// Number of buttons
    pub buttons: u32,
    /// Number of axes
    pub axes: u32,
    /// Whether the device supports force feedback
    pub has_force_feedback: bool,
    /// Vendor ID (if available)
    pub vendor_id: Option<u16>,
    /// Product ID (if available)
    pub product_id: Option<u16>,
}

/// Gamepad state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamepadState {
    /// Gamepad ID
    pub id: String,
    /// Button states (true = pressed)
    pub buttons: Vec<bool>,
    /// Axis values (-1.0 to 1.0)
    pub axes: Vec<f32>,
    /// Timestamp in milliseconds
    pub timestamp: u64,
}

/// Gamepad event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamepadEvent {
    /// Gamepad ID
    pub id: String,
    /// Event type
    pub event_type: String,
    /// Button index (for button events)
    pub button: Option<u32>,
    /// Axis index (for axis events)
    pub axis: Option<u32>,
    /// Value (button pressed state or axis value)
    pub value: f32,
    /// Timestamp in milliseconds
    pub timestamp: u64,
}

/// Force feedback effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceFeedbackEffect {
    /// Effect type
    pub effect_type: String,
    /// Duration in milliseconds
    pub duration: u32,
    /// Strength (0.0 to 1.0)
    pub strength: f32,
    /// Direction (degrees, 0-360)
    pub direction: Option<u16>,
    /// Effect-specific parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

// Global gamepad state
lazy_static! {
    static ref GAMEPAD_CONNECTIONS: Arc<Mutex<HashMap<String, GamepadConnection>>> = Arc::new(Mutex::new(HashMap::new()));
}

/// Gamepad connection information
struct GamepadConnection {
    device_id: String,
    device_name: String,
    index: u32,
    connected_at: std::time::SystemTime,
    // In a real implementation, we would store platform-specific connection handles here
}

/// Check if gamepad access is available on this device
pub fn is_gamepad_available() -> bool {
    #[cfg(target_os = "linux")]
    {
        // On Linux, check if we can access gamepads
        // This is a simplified check; in a real implementation we would use gilrs or similar
        std::path::Path::new("/dev/input").exists()
    }
    
    #[cfg(target_os = "windows")]
    {
        // On Windows, check if XInput is available
        // This is a simplified check; in a real implementation we would use the Windows API
        true
    }
    
    #[cfg(target_os = "macos")]
    {
        // On macOS, check if IOKit is available
        // This is a simplified check; in a real implementation we would use IOKit
        true
    }
    
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        false // Default fallback for unsupported platforms
    }
}

/// Get a list of connected gamepad devices
pub async fn get_gamepad_devices() -> Result<Vec<GamepadDevice>> {
    if !is_gamepad_available() {
        return Err(HardwareError::UnsupportedOperation("Gamepad access is not available on this device".to_string()));
    }
    
    // In a real implementation, we would use platform-specific APIs to get gamepad devices
    // For simplicity, we'll just return some dummy devices
    
    let devices = vec![
        GamepadDevice {
            id: "gamepad_1".to_string(),
            name: "SmashLang Gamepad".to_string(),
            index: 0,
            buttons: 16,
            axes: 6,
            has_force_feedback: true,
            vendor_id: Some(0x046d),
            product_id: Some(0xc21d),
        },
        GamepadDevice {
            id: "gamepad_2".to_string(),
            name: "SmashLang Pro Controller".to_string(),
            index: 1,
            buttons: 20,
            axes: 8,
            has_force_feedback: true,
            vendor_id: Some(0x054c),
            product_id: Some(0x05c4),
        },
    ];
    
    Ok(devices)
}

/// Open a gamepad device
pub async fn open_gamepad(device_id: &str) -> Result<bool> {
    if !is_gamepad_available() {
        return Err(HardwareError::UnsupportedOperation("Gamepad access is not available on this device".to_string()));
    }
    
    // Find the device
    let devices = get_gamepad_devices().await?;
    let device = devices.into_iter().find(|d| d.id == device_id);
    
    if let Some(device) = device {
        // Check if already connected
        let connections = GAMEPAD_CONNECTIONS.lock().unwrap();
        if connections.contains_key(device_id) {
            return Ok(true);
        }
        
        // Create a new connection
        let connection = GamepadConnection {
            device_id: device.id.clone(),
            device_name: device.name.clone(),
            index: device.index,
            connected_at: std::time::SystemTime::now(),
        };
        
        // Store the connection
        let mut connections = GAMEPAD_CONNECTIONS.lock().unwrap();
        connections.insert(device.id.clone(), connection);
        
        Ok(true)
    } else {
        Err(HardwareError::InvalidId(format!("Gamepad device not found: {}", device_id)))
    }
}

/// Close a gamepad device
pub async fn close_gamepad(device_id: &str) -> Result<bool> {
    if !is_gamepad_available() {
        return Err(HardwareError::UnsupportedOperation("Gamepad access is not available on this device".to_string()));
    }
    
    // Check if connected
    let mut connections = GAMEPAD_CONNECTIONS.lock().unwrap();
    if connections.remove(device_id).is_none() {
        return Err(HardwareError::InvalidOperation(format!("Not connected to gamepad device: {}", device_id)));
    }
    
    Ok(true)
}

/// Get the current state of a gamepad
pub async fn get_gamepad_state(device_id: &str) -> Result<GamepadState> {
    if !is_gamepad_available() {
        return Err(HardwareError::UnsupportedOperation("Gamepad access is not available on this device".to_string()));
    }
    
    // Check if connected
    let connections = GAMEPAD_CONNECTIONS.lock().unwrap();
    if !connections.contains_key(device_id) {
        return Err(HardwareError::InvalidOperation(format!("Not connected to gamepad device: {}", device_id)));
    }
    
    // In a real implementation, we would get the current state from the device
    // For simplicity, we'll just return a dummy state
    
    // Find the device to get button and axis counts
    let devices = get_gamepad_devices().await?;
    let device = devices.into_iter().find(|d| d.id == device_id);
    
    if let Some(device) = device {
        let buttons = vec![false; device.buttons as usize];
        let axes = vec![0.0; device.axes as usize];
        
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        
        Ok(GamepadState {
            id: device_id.to_string(),
            buttons,
            axes,
            timestamp,
        })
    } else {
        Err(HardwareError::InvalidId(format!("Gamepad device not found: {}", device_id)))
    }
}

/// Start monitoring gamepad events
pub async fn start_monitoring_events(device_id: &str) -> Result<bool> {
    if !is_gamepad_available() {
        return Err(HardwareError::UnsupportedOperation("Gamepad access is not available on this device".to_string()));
    }
    
    // Check if connected
    let connections = GAMEPAD_CONNECTIONS.lock().unwrap();
    if !connections.contains_key(device_id) {
        return Err(HardwareError::InvalidOperation(format!("Not connected to gamepad device: {}", device_id)));
    }
    
    // In a real implementation, we would start a thread to monitor events
    // For simplicity, we'll just return success
    
    Ok(true)
}

/// Stop monitoring gamepad events
pub async fn stop_monitoring_events(device_id: &str) -> Result<bool> {
    if !is_gamepad_available() {
        return Err(HardwareError::UnsupportedOperation("Gamepad access is not available on this device".to_string()));
    }
    
    // Check if connected
    let connections = GAMEPAD_CONNECTIONS.lock().unwrap();
    if !connections.contains_key(device_id) {
        return Err(HardwareError::InvalidOperation(format!("Not connected to gamepad device: {}", device_id)));
    }
    
    // In a real implementation, we would stop the monitoring thread
    // For simplicity, we'll just return success
    
    Ok(true)
}

/// Apply a force feedback effect to a gamepad
pub async fn apply_force_feedback(device_id: &str, effect: ForceFeedbackEffect) -> Result<bool> {
    if !is_gamepad_available() {
        return Err(HardwareError::UnsupportedOperation("Gamepad access is not available on this device".to_string()));
    }
    
    // Check if connected
    let connections = GAMEPAD_CONNECTIONS.lock().unwrap();
    if !connections.contains_key(device_id) {
        return Err(HardwareError::InvalidOperation(format!("Not connected to gamepad device: {}", device_id)));
    }
    
    // Check if the device supports force feedback
    let devices = get_gamepad_devices().await?;
    let device = devices.into_iter().find(|d| d.id == device_id);
    
    if let Some(device) = device {
        if !device.has_force_feedback {
            return Err(HardwareError::UnsupportedOperation(format!("Gamepad device does not support force feedback: {}", device_id)));
        }
        
        // In a real implementation, we would apply the effect to the device
        // For simplicity, we'll just return success
        
        Ok(true)
    } else {
        Err(HardwareError::InvalidId(format!("Gamepad device not found: {}", device_id)))
    }
}

/// Stop all force feedback effects on a gamepad
pub async fn stop_force_feedback(device_id: &str) -> Result<bool> {
    if !is_gamepad_available() {
        return Err(HardwareError::UnsupportedOperation("Gamepad access is not available on this device".to_string()));
    }
    
    // Check if connected
    let connections = GAMEPAD_CONNECTIONS.lock().unwrap();
    if !connections.contains_key(device_id) {
        return Err(HardwareError::InvalidOperation(format!("Not connected to gamepad device: {}", device_id)));
    }
    
    // In a real implementation, we would stop all effects on the device
    // For simplicity, we'll just return success
    
    Ok(true)
}

/// Create a rumble effect
pub fn create_rumble_effect(strong_magnitude: f32, weak_magnitude: f32, duration_ms: u32) -> ForceFeedbackEffect {
    let mut parameters = HashMap::new();
    parameters.insert("strong_magnitude".to_string(), serde_json::Value::Number(serde_json::Number::from_f64((strong_magnitude as f64).clamp(0.0, 1.0)).unwrap()));
    parameters.insert("weak_magnitude".to_string(), serde_json::Value::Number(serde_json::Number::from_f64((weak_magnitude as f64).clamp(0.0, 1.0)).unwrap()));
    
    ForceFeedbackEffect {
        effect_type: "rumble".to_string(),
        duration: duration_ms,
        strength: (strong_magnitude + weak_magnitude) / 2.0,
        direction: None,
        parameters,
    }
}
