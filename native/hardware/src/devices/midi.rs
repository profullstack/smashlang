//! MIDI module for SmashLang hardware interfaces
//!
//! Provides access to MIDI devices and functionality.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::error::HardwareError;
use crate::Result;

/// MIDI device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidiDevice {
    /// Unique identifier for the device
    pub id: String,
    /// Human-readable name for the device
    pub name: String,
    /// Manufacturer name (if available)
    pub manufacturer: Option<String>,
    /// Whether the device is an input device
    pub is_input: bool,
    /// Whether the device is an output device
    pub is_output: bool,
    /// Whether the device is virtual
    pub is_virtual: bool,
}

/// MIDI message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidiMessage {
    /// MIDI message type
    pub message_type: String,
    /// MIDI channel (0-15)
    pub channel: u8,
    /// First data byte
    pub data1: u8,
    /// Second data byte
    pub data2: u8,
    /// Timestamp in milliseconds
    pub timestamp: u64,
}

// Global MIDI state
lazy_static! {
    static ref MIDI_INPUTS: Arc<Mutex<HashMap<String, MidiInput>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref MIDI_OUTPUTS: Arc<Mutex<HashMap<String, MidiOutput>>> = Arc::new(Mutex::new(HashMap::new()));
}

/// MIDI input connection
struct MidiInput {
    device_id: String,
    device_name: String,
    connected_at: std::time::SystemTime,
    // In a real implementation, we would store platform-specific connection handles here
}

/// MIDI output connection
struct MidiOutput {
    device_id: String,
    device_name: String,
    connected_at: std::time::SystemTime,
    // In a real implementation, we would store platform-specific connection handles here
}

/// Check if MIDI access is available on this device
pub fn is_midi_available() -> bool {
    #[cfg(target_os = "linux")]
    {
        // On Linux, check if ALSA MIDI is available
        // This is a simplified check; in a real implementation we would use midir
        std::process::Command::new("aconnect")
            .arg("-l")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
    
    #[cfg(target_os = "windows")]
    {
        // On Windows, check if Windows MIDI API is available
        // This is a simplified check; in a real implementation we would use the Windows API
        true
    }
    
    #[cfg(target_os = "macos")]
    {
        // On macOS, check if CoreMIDI is available
        // This is a simplified check; in a real implementation we would use CoreMIDI
        true
    }
    
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        false // Default fallback for unsupported platforms
    }
}

/// Get a list of MIDI input devices
pub async fn get_midi_inputs() -> Result<Vec<MidiDevice>> {
    if !is_midi_available() {
        return Err(HardwareError::UnsupportedOperation("MIDI access is not available on this device".to_string()));
    }
    
    // In a real implementation, we would use platform-specific APIs to get MIDI input devices
    // For simplicity, we'll just return some dummy devices
    
    let devices = vec![
        MidiDevice {
            id: "midi_in_1".to_string(),
            name: "SmashLang MIDI Keyboard".to_string(),
            manufacturer: Some("SmashLang Music".to_string()),
            is_input: true,
            is_output: false,
            is_virtual: false,
        },
        MidiDevice {
            id: "midi_in_2".to_string(),
            name: "SmashLang MIDI Controller".to_string(),
            manufacturer: Some("SmashLang Music".to_string()),
            is_input: true,
            is_output: false,
            is_virtual: false,
        },
    ];
    
    Ok(devices)
}

/// Get a list of MIDI output devices
pub async fn get_midi_outputs() -> Result<Vec<MidiDevice>> {
    if !is_midi_available() {
        return Err(HardwareError::UnsupportedOperation("MIDI access is not available on this device".to_string()));
    }
    
    // In a real implementation, we would use platform-specific APIs to get MIDI output devices
    // For simplicity, we'll just return some dummy devices
    
    let devices = vec![
        MidiDevice {
            id: "midi_out_1".to_string(),
            name: "SmashLang MIDI Synthesizer".to_string(),
            manufacturer: Some("SmashLang Music".to_string()),
            is_input: false,
            is_output: true,
            is_virtual: false,
        },
        MidiDevice {
            id: "midi_out_2".to_string(),
            name: "SmashLang MIDI Sound Module".to_string(),
            manufacturer: Some("SmashLang Music".to_string()),
            is_input: false,
            is_output: true,
            is_virtual: false,
        },
    ];
    
    Ok(devices)
}

/// Open a MIDI input device
pub async fn open_midi_input(device_id: &str) -> Result<bool> {
    if !is_midi_available() {
        return Err(HardwareError::UnsupportedOperation("MIDI access is not available on this device".to_string()));
    }
    
    // Find the device
    let devices = get_midi_inputs().await?;
    let device = devices.into_iter().find(|d| d.id == device_id);
    
    if let Some(device) = device {
        // Check if already connected
        let inputs = MIDI_INPUTS.lock().unwrap();
        if inputs.contains_key(device_id) {
            return Ok(true);
        }
        
        // Create a new connection
        let input = MidiInput {
            device_id: device.id.clone(),
            device_name: device.name.clone(),
            connected_at: std::time::SystemTime::now(),
        };
        
        // Store the connection
        let mut inputs = MIDI_INPUTS.lock().unwrap();
        inputs.insert(device.id.clone(), input);
        
        Ok(true)
    } else {
        Err(HardwareError::InvalidId(format!("MIDI input device not found: {}", device_id)))
    }
}

/// Close a MIDI input device
pub async fn close_midi_input(device_id: &str) -> Result<bool> {
    if !is_midi_available() {
        return Err(HardwareError::UnsupportedOperation("MIDI access is not available on this device".to_string()));
    }
    
    // Check if connected
    let mut inputs = MIDI_INPUTS.lock().unwrap();
    if inputs.remove(device_id).is_none() {
        return Err(HardwareError::InvalidOperation(format!("Not connected to MIDI input device: {}", device_id)));
    }
    
    Ok(true)
}

/// Open a MIDI output device
pub async fn open_midi_output(device_id: &str) -> Result<bool> {
    if !is_midi_available() {
        return Err(HardwareError::UnsupportedOperation("MIDI access is not available on this device".to_string()));
    }
    
    // Find the device
    let devices = get_midi_outputs().await?;
    let device = devices.into_iter().find(|d| d.id == device_id);
    
    if let Some(device) = device {
        // Check if already connected
        let outputs = MIDI_OUTPUTS.lock().unwrap();
        if outputs.contains_key(device_id) {
            return Ok(true);
        }
        
        // Create a new connection
        let output = MidiOutput {
            device_id: device.id.clone(),
            device_name: device.name.clone(),
            connected_at: std::time::SystemTime::now(),
        };
        
        // Store the connection
        let mut outputs = MIDI_OUTPUTS.lock().unwrap();
        outputs.insert(device.id.clone(), output);
        
        Ok(true)
    } else {
        Err(HardwareError::InvalidId(format!("MIDI output device not found: {}", device_id)))
    }
}

/// Close a MIDI output device
pub async fn close_midi_output(device_id: &str) -> Result<bool> {
    if !is_midi_available() {
        return Err(HardwareError::UnsupportedOperation("MIDI access is not available on this device".to_string()));
    }
    
    // Check if connected
    let mut outputs = MIDI_OUTPUTS.lock().unwrap();
    if outputs.remove(device_id).is_none() {
        return Err(HardwareError::InvalidOperation(format!("Not connected to MIDI output device: {}", device_id)));
    }
    
    Ok(true)
}

/// Send a MIDI message to an output device
pub async fn send_midi_message(device_id: &str, message: MidiMessage) -> Result<bool> {
    if !is_midi_available() {
        return Err(HardwareError::UnsupportedOperation("MIDI access is not available on this device".to_string()));
    }
    
    // Check if connected
    let outputs = MIDI_OUTPUTS.lock().unwrap();
    if !outputs.contains_key(device_id) {
        return Err(HardwareError::InvalidOperation(format!("Not connected to MIDI output device: {}", device_id)));
    }
    
    // In a real implementation, we would send the MIDI message to the device
    // For simplicity, we'll just return success
    
    Ok(true)
}

/// Create a virtual MIDI input device
pub async fn create_virtual_midi_input(name: &str) -> Result<MidiDevice> {
    if !is_midi_available() {
        return Err(HardwareError::UnsupportedOperation("MIDI access is not available on this device".to_string()));
    }
    
    // In a real implementation, we would create a virtual MIDI input device
    // For simplicity, we'll just return a dummy device
    
    let device = MidiDevice {
        id: format!("virtual_midi_in_{}", uuid::Uuid::new_v4().to_string()),
        name: name.to_string(),
        manufacturer: Some("SmashLang".to_string()),
        is_input: true,
        is_output: false,
        is_virtual: true,
    };
    
    Ok(device)
}

/// Create a virtual MIDI output device
pub async fn create_virtual_midi_output(name: &str) -> Result<MidiDevice> {
    if !is_midi_available() {
        return Err(HardwareError::UnsupportedOperation("MIDI access is not available on this device".to_string()));
    }
    
    // In a real implementation, we would create a virtual MIDI output device
    // For simplicity, we'll just return a dummy device
    
    let device = MidiDevice {
        id: format!("virtual_midi_out_{}", uuid::Uuid::new_v4().to_string()),
        name: name.to_string(),
        manufacturer: Some("SmashLang".to_string()),
        is_input: false,
        is_output: true,
        is_virtual: true,
    };
    
    Ok(device)
}

/// Helper function to create a note-on MIDI message
pub fn create_note_on(channel: u8, note: u8, velocity: u8) -> MidiMessage {
    MidiMessage {
        message_type: "note_on".to_string(),
        channel,
        data1: note,
        data2: velocity,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64,
    }
}

/// Helper function to create a note-off MIDI message
pub fn create_note_off(channel: u8, note: u8, velocity: u8) -> MidiMessage {
    MidiMessage {
        message_type: "note_off".to_string(),
        channel,
        data1: note,
        data2: velocity,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64,
    }
}

/// Helper function to create a control change MIDI message
pub fn create_control_change(channel: u8, controller: u8, value: u8) -> MidiMessage {
    MidiMessage {
        message_type: "control_change".to_string(),
        channel,
        data1: controller,
        data2: value,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64,
    }
}

/// Helper function to create a program change MIDI message
pub fn create_program_change(channel: u8, program: u8) -> MidiMessage {
    MidiMessage {
        message_type: "program_change".to_string(),
        channel,
        data1: program,
        data2: 0,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64,
    }
}
