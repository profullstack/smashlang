//! USB module for SmashLang hardware interfaces
//!
//! Provides access to USB devices and functionality.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::error::HardwareError;
use crate::Result;

/// USB device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbDevice {
    /// Unique identifier for the device
    pub id: String,
    /// Human-readable name for the device
    pub name: String,
    /// Vendor ID
    pub vendor_id: u16,
    /// Product ID
    pub product_id: u16,
    /// Serial number (if available)
    pub serial_number: Option<String>,
    /// Manufacturer name (if available)
    pub manufacturer: Option<String>,
    /// Product name (if available)
    pub product: Option<String>,
    /// Device class
    pub device_class: u8,
    /// Device subclass
    pub device_subclass: u8,
    /// Device protocol
    pub device_protocol: u8,
    /// Bus number
    pub bus_number: u8,
    /// Device address on the bus
    pub device_address: u8,
}

// Global USB state
lazy_static! {
    static ref USB_CONNECTIONS: Arc<Mutex<HashMap<String, UsbConnection>>> = Arc::new(Mutex::new(HashMap::new()));
}

/// USB connection information
struct UsbConnection {
    device_id: String,
    device_name: String,
    vendor_id: u16,
    product_id: u16,
    connected_at: std::time::SystemTime,
    // In a real implementation, we would store platform-specific connection handles here
}

/// Check if USB access is available on this device
pub fn is_usb_available() -> bool {
    #[cfg(target_os = "linux")]
    {
        // On Linux, check if we can access USB devices
        // This is a simplified check; in a real implementation we would use libusb
        std::process::Command::new("lsusb")
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
    
    #[cfg(target_os = "windows")]
    {
        // On Windows, check if USB API is available
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

/// Get a list of connected USB devices
pub async fn get_usb_devices() -> Result<Vec<UsbDevice>> {
    if !is_usb_available() {
        return Err(HardwareError::UnsupportedOperation("USB access is not available on this device".to_string()));
    }
    
    // In a real implementation, we would use platform-specific APIs to get USB devices
    // For simplicity, we'll just return some dummy devices
    
    let devices = vec![
        UsbDevice {
            id: "usb_1".to_string(),
            name: "SmashLang USB Keyboard".to_string(),
            vendor_id: 0x046d,
            product_id: 0xc52b,
            serial_number: Some("1234567890".to_string()),
            manufacturer: Some("SmashLang Peripherals".to_string()),
            product: Some("USB Keyboard".to_string()),
            device_class: 0x03, // HID
            device_subclass: 0x01,
            device_protocol: 0x01,
            bus_number: 1,
            device_address: 2,
        },
        UsbDevice {
            id: "usb_2".to_string(),
            name: "SmashLang USB Mouse".to_string(),
            vendor_id: 0x046d,
            product_id: 0xc52c,
            serial_number: Some("0987654321".to_string()),
            manufacturer: Some("SmashLang Peripherals".to_string()),
            product: Some("USB Mouse".to_string()),
            device_class: 0x03, // HID
            device_subclass: 0x01,
            device_protocol: 0x02,
            bus_number: 1,
            device_address: 3,
        },
        UsbDevice {
            id: "usb_3".to_string(),
            name: "SmashLang USB Flash Drive".to_string(),
            vendor_id: 0x0781,
            product_id: 0x5580,
            serial_number: Some("ABCDEF123456".to_string()),
            manufacturer: Some("SmashLang Storage".to_string()),
            product: Some("USB Flash Drive".to_string()),
            device_class: 0x08, // Mass Storage
            device_subclass: 0x06,
            device_protocol: 0x50,
            bus_number: 2,
            device_address: 1,
        },
    ];
    
    Ok(devices)
}

/// Get information about a specific USB device
pub async fn get_usb_device(device_id: &str) -> Result<UsbDevice> {
    if !is_usb_available() {
        return Err(HardwareError::UnsupportedOperation("USB access is not available on this device".to_string()));
    }
    
    // Find the device
    let devices = get_usb_devices().await?;
    let device = devices.into_iter().find(|d| d.id == device_id);
    
    if let Some(device) = device {
        Ok(device)
    } else {
        Err(HardwareError::InvalidId(format!("USB device not found: {}", device_id)))
    }
}

/// Open a connection to a USB device
pub async fn open_usb_device(device_id: &str) -> Result<bool> {
    if !is_usb_available() {
        return Err(HardwareError::UnsupportedOperation("USB access is not available on this device".to_string()));
    }
    
    // Find the device
    let device = get_usb_device(device_id).await?;
    
    // Check if already connected
    let connections = USB_CONNECTIONS.lock().unwrap();
    if connections.contains_key(device_id) {
        return Ok(true);
    }
    
    // Create a new connection
    let connection = UsbConnection {
        device_id: device.id.clone(),
        device_name: device.name.clone(),
        vendor_id: device.vendor_id,
        product_id: device.product_id,
        connected_at: std::time::SystemTime::now(),
    };
    
    // Store the connection
    let mut connections = USB_CONNECTIONS.lock().unwrap();
    connections.insert(device.id.clone(), connection);
    
    Ok(true)
}

/// Close a connection to a USB device
pub async fn close_usb_device(device_id: &str) -> Result<bool> {
    if !is_usb_available() {
        return Err(HardwareError::UnsupportedOperation("USB access is not available on this device".to_string()));
    }
    
    // Check if connected
    let mut connections = USB_CONNECTIONS.lock().unwrap();
    if connections.remove(device_id).is_none() {
        return Err(HardwareError::InvalidOperation(format!("Not connected to USB device: {}", device_id)));
    }
    
    Ok(true)
}

/// Send data to a USB device
pub async fn send_usb_data(device_id: &str, endpoint: u8, data: &[u8]) -> Result<usize> {
    if !is_usb_available() {
        return Err(HardwareError::UnsupportedOperation("USB access is not available on this device".to_string()));
    }
    
    // Check if connected
    let connections = USB_CONNECTIONS.lock().unwrap();
    if !connections.contains_key(device_id) {
        return Err(HardwareError::InvalidOperation(format!("Not connected to USB device: {}", device_id)));
    }
    
    // In a real implementation, we would send the data to the device
    // For simplicity, we'll just return the data length
    
    Ok(data.len())
}

/// Receive data from a USB device
pub async fn receive_usb_data(device_id: &str, endpoint: u8, length: usize) -> Result<Vec<u8>> {
    if !is_usb_available() {
        return Err(HardwareError::UnsupportedOperation("USB access is not available on this device".to_string()));
    }
    
    // Check if connected
    let connections = USB_CONNECTIONS.lock().unwrap();
    if !connections.contains_key(device_id) {
        return Err(HardwareError::InvalidOperation(format!("Not connected to USB device: {}", device_id)));
    }
    
    // In a real implementation, we would receive data from the device
    // For simplicity, we'll just return a dummy buffer
    
    let mut buffer = Vec::with_capacity(length);
    for i in 0..length {
        buffer.push((i % 256) as u8);
    }
    
    Ok(buffer)
}

/// Reset a USB device
pub async fn reset_usb_device(device_id: &str) -> Result<bool> {
    if !is_usb_available() {
        return Err(HardwareError::UnsupportedOperation("USB access is not available on this device".to_string()));
    }
    
    // Check if connected
    let connections = USB_CONNECTIONS.lock().unwrap();
    if !connections.contains_key(device_id) {
        return Err(HardwareError::InvalidOperation(format!("Not connected to USB device: {}", device_id)));
    }
    
    // In a real implementation, we would reset the device
    // For simplicity, we'll just return success
    
    Ok(true)
}
