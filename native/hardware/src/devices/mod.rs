//! Devices module for SmashLang hardware interfaces
//!
//! Provides access to various hardware peripherals and system devices.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::error::HardwareError;
use crate::Result;

pub mod bluetooth;
pub mod usb;
pub mod midi;
pub mod gamepad;

/// Device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    /// Unique identifier for the device
    pub id: String,
    /// Human-readable name for the device
    pub name: String,
    /// Type of device ('audio', 'video', 'hid', 'bluetooth', etc.)
    pub device_type: String,
    /// Whether the device is currently connected
    pub connected: bool,
    /// Device-specific capabilities
    pub capabilities: HashMap<String, serde_json::Value>,
}

/// Get a list of all connected hardware devices
pub async fn get_all_devices(device_type: Option<&str>) -> Result<Vec<Device>> {
    let mut devices = Vec::new();
    
    // Get Bluetooth devices
    if device_type.is_none() || device_type == Some("bluetooth") {
        if bluetooth::is_bluetooth_available() {
            if let Ok(bluetooth_devices) = bluetooth::get_bluetooth_devices().await {
                devices.extend(bluetooth_devices.into_iter().map(|device| Device {
                    id: device.id,
                    name: device.name,
                    device_type: "bluetooth".to_string(),
                    connected: device.connected,
                    capabilities: {
                        let mut caps = HashMap::new();
                        caps.insert("address".to_string(), serde_json::Value::String(device.address));
                        if let Some(rssi) = device.rssi {
                            caps.insert("rssi".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(rssi).unwrap_or_default()));
                        }
                        caps.insert("paired".to_string(), serde_json::Value::Bool(device.paired));
                        caps
                    },
                }));
            }
        }
    }
    
    // Get USB devices
    if device_type.is_none() || device_type == Some("usb") {
        if usb::is_usb_available() {
            if let Ok(usb_devices) = usb::get_usb_devices().await {
                devices.extend(usb_devices.into_iter().map(|device| Device {
                    id: device.id,
                    name: device.name,
                    device_type: "usb".to_string(),
                    connected: true,
                    capabilities: {
                        let mut caps = HashMap::new();
                        caps.insert("vendorId".to_string(), serde_json::Value::Number(serde_json::Number::from(device.vendor_id)));
                        caps.insert("productId".to_string(), serde_json::Value::Number(serde_json::Number::from(device.product_id)));
                        if let Some(serial) = device.serial_number {
                            caps.insert("serialNumber".to_string(), serde_json::Value::String(serial));
                        }
                        caps
                    },
                }));
            }
        }
    }
    
    // Get MIDI devices
    if device_type.is_none() || device_type == Some("midi") {
        if midi::is_midi_available() {
            // Get MIDI inputs
            if let Ok(midi_inputs) = midi::get_midi_inputs().await {
                devices.extend(midi_inputs.into_iter().map(|device| Device {
                    id: device.id,
                    name: device.name,
                    device_type: "midi-input".to_string(),
                    connected: true,
                    capabilities: {
                        let mut caps = HashMap::new();
                        if let Some(manufacturer) = device.manufacturer {
                            caps.insert("manufacturer".to_string(), serde_json::Value::String(manufacturer));
                        }
                        caps
                    },
                }));
            }
            
            // Get MIDI outputs
            if let Ok(midi_outputs) = midi::get_midi_outputs().await {
                devices.extend(midi_outputs.into_iter().map(|device| Device {
                    id: device.id,
                    name: device.name,
                    device_type: "midi-output".to_string(),
                    connected: true,
                    capabilities: {
                        let mut caps = HashMap::new();
                        if let Some(manufacturer) = device.manufacturer {
                            caps.insert("manufacturer".to_string(), serde_json::Value::String(manufacturer));
                        }
                        caps
                    },
                }));
            }
        }
    }
    
    // Get gamepad devices
    if device_type.is_none() || device_type == Some("gamepad") {
        if gamepad::is_gamepad_available() {
            if let Ok(gamepad_devices) = gamepad::get_gamepad_devices().await {
                devices.extend(gamepad_devices.into_iter().map(|device| Device {
                    id: device.id,
                    name: device.name,
                    device_type: "gamepad".to_string(),
                    connected: true,
                    capabilities: {
                        let mut caps = HashMap::new();
                        caps.insert("index".to_string(), serde_json::Value::Number(serde_json::Number::from(device.index)));
                        caps.insert("buttons".to_string(), serde_json::Value::Number(serde_json::Number::from(device.buttons)));
                        caps.insert("axes".to_string(), serde_json::Value::Number(serde_json::Number::from(device.axes)));
                        caps
                    },
                }));
            }
        }
    }
    
    Ok(devices)
}

// Global device monitoring state
lazy_static! {
    static ref DEVICE_MONITORS: Arc<Mutex<HashMap<String, DeviceMonitor>>> = Arc::new(Mutex::new(HashMap::new()));
}

/// Device monitor information
struct DeviceMonitor {
    id: String,
    device_type: Option<String>,
    // In a real implementation, we would store a callback function here
}

/// Monitor device connections and disconnections
pub async fn monitor_devices(device_type: Option<&str>) -> Result<String> {
    // Generate a unique ID for this monitor
    let monitor_id = format!("device_monitor_{}", uuid::Uuid::new_v4().to_string());
    
    // Create a new monitor
    let monitor = DeviceMonitor {
        id: monitor_id.clone(),
        device_type: device_type.map(|s| s.to_string()),
    };
    
    // Store the monitor
    let mut monitors = DEVICE_MONITORS.lock().unwrap();
    monitors.insert(monitor_id.clone(), monitor);
    
    // In a real implementation, we would start a background thread to monitor devices
    // For simplicity, we'll just store the monitor configuration
    
    Ok(monitor_id)
}

/// Stop monitoring device changes
pub async fn stop_monitoring(monitor_id: &str) -> Result<()> {
    let mut monitors = DEVICE_MONITORS.lock().unwrap();
    
    if monitors.remove(monitor_id).is_none() {
        return Err(HardwareError::InvalidId(format!("Device monitor not found: {}", monitor_id)));
    }
    
    Ok(())
}
