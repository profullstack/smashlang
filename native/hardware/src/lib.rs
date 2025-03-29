//! SmashLang Hardware Interfaces - Native Implementation
//!
//! This crate provides native hardware access for SmashLang, including:
//! - Camera access for photos and video recording
//! - Microphone access for audio recording and speech recognition
//! - Screen capture and recording
//! - Device management (Bluetooth, USB, MIDI, Gamepad)

#[macro_use]
extern crate lazy_static;

use std::sync::{Arc, Mutex};

#[cfg(feature = "node")]
use napi_derive::napi;

mod camera;
mod microphone;
mod screen;
mod input;
mod devices;
mod error;
mod utils;

pub use error::HardwareError;
pub type Result<T> = std::result::Result<T, HardwareError>;

/// Re-export all modules for direct use
pub use camera::Camera;
pub use microphone::Microphone;
pub use screen::Screen;
pub use input::Input;
pub use devices::{bluetooth, usb, midi, gamepad};

#[cfg(feature = "node")]
#[napi]
pub fn init_hardware() -> bool {
    // Initialize any global hardware resources
    true
}

#[cfg(feature = "node")]
mod node_bindings {
    use super::*;
    use napi_derive::napi;
    
    // Camera bindings
    pub mod camera {
        use super::*;
        use crate::camera::*;
        
        #[napi]
        pub fn camera_is_available() -> bool {
            Camera::is_available()
        }
        
        #[napi]
        pub async fn camera_request_permission() -> bool {
            Camera::request_permission().await.unwrap_or(false)
        }
        
        #[napi]
        pub async fn camera_get_devices() -> napi::Result<String> {
            match Camera::get_devices().await {
                Ok(devices) => Ok(serde_json::to_string(&devices).unwrap_or_default()),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub async fn camera_start(options: String) -> napi::Result<String> {
            let options: CameraOptions = match serde_json::from_str(&options) {
                Ok(opts) => opts,
                Err(e) => return Err(napi::Error::from_reason(format!("Invalid camera options: {}", e)))
            };
            
            match Camera::start(options).await {
                Ok(stream) => Ok(serde_json::to_string(&stream).unwrap_or_default()),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub fn camera_stop(stream_id: String) -> napi::Result<bool> {
            match Camera::stop(&stream_id) {
                Ok(_) => Ok(true),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub async fn camera_take_photo(stream_id: String, options: String) -> napi::Result<String> {
            let options: PhotoOptions = match serde_json::from_str(&options) {
                Ok(opts) => opts,
                Err(e) => return Err(napi::Error::from_reason(format!("Invalid photo options: {}", e)))
            };
            
            match Camera::take_photo(&stream_id, options).await {
                Ok(photo) => Ok(serde_json::to_string(&photo).unwrap_or_default()),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub async fn camera_save_photo(photo_data: String, file_path: String) -> napi::Result<String> {
            match Camera::save_photo(&photo_data, &file_path).await {
                Ok(result) => Ok(serde_json::to_string(&result).unwrap_or_default()),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub async fn camera_start_recording(stream_id: String, options: String) -> napi::Result<bool> {
            let options: RecordingOptions = match serde_json::from_str(&options) {
                Ok(opts) => opts,
                Err(e) => return Err(napi::Error::from_reason(format!("Invalid recording options: {}", e)))
            };
            
            match Camera::start_recording(&stream_id, options).await {
                Ok(_) => Ok(true),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub async fn camera_stop_recording(stream_id: String, file_path: String) -> napi::Result<String> {
            match Camera::stop_recording(&stream_id, &file_path).await {
                Ok(result) => Ok(serde_json::to_string(&result).unwrap_or_default()),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub async fn camera_apply_filter(stream_id: String, filter_name: String, options: String) -> napi::Result<bool> {
            let options: serde_json::Value = match serde_json::from_str(&options) {
                Ok(opts) => opts,
                Err(e) => return Err(napi::Error::from_reason(format!("Invalid filter options: {}", e)))
            };
            
            match Camera::apply_filter(&stream_id, &filter_name, options).await {
                Ok(_) => Ok(true),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub async fn camera_remove_filters(stream_id: String) -> napi::Result<bool> {
            match Camera::remove_filters(&stream_id).await {
                Ok(_) => Ok(true),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
    }
    
    // Microphone bindings
    pub mod microphone {
        use super::*;
        use crate::microphone::*;
        
        #[napi]
        pub fn microphone_is_available() -> bool {
            Microphone::is_available()
        }
        
        #[napi]
        pub async fn microphone_request_permission() -> bool {
            Microphone::request_permission().await.unwrap_or(false)
        }
        
        #[napi]
        pub async fn microphone_get_devices() -> napi::Result<String> {
            match Microphone::get_devices().await {
                Ok(devices) => Ok(serde_json::to_string(&devices).unwrap_or_default()),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub async fn microphone_start(options: String) -> napi::Result<String> {
            let options: MicrophoneOptions = match serde_json::from_str(&options) {
                Ok(opts) => opts,
                Err(e) => return Err(napi::Error::from_reason(format!("Invalid microphone options: {}", e)))
            };
            
            match Microphone::start(options).await {
                Ok(stream) => Ok(serde_json::to_string(&stream).unwrap_or_default()),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub fn microphone_stop(stream_id: String) -> napi::Result<bool> {
            match Microphone::stop(&stream_id) {
                Ok(_) => Ok(true),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub async fn microphone_start_recording(stream_id: String, options: String) -> napi::Result<bool> {
            let options: AudioRecordingOptions = match serde_json::from_str(&options) {
                Ok(opts) => opts,
                Err(e) => return Err(napi::Error::from_reason(format!("Invalid recording options: {}", e)))
            };
            
            match Microphone::start_recording(&stream_id, options).await {
                Ok(_) => Ok(true),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub async fn microphone_stop_recording(stream_id: String, file_path: Option<String>) -> napi::Result<String> {
            match Microphone::stop_recording(&stream_id, file_path.as_deref()).await {
                Ok(result) => Ok(serde_json::to_string(&result).unwrap_or_default()),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub async fn microphone_save_recording(recording_data: String, file_path: String, format: Option<String>) -> napi::Result<String> {
            match Microphone::save_recording(&recording_data, &file_path, format.as_deref()).await {
                Ok(result) => Ok(serde_json::to_string(&result).unwrap_or_default()),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub async fn microphone_get_audio_level(stream_id: String) -> napi::Result<f64> {
            match Microphone::get_audio_level(&stream_id).await {
                Ok(level) => Ok(level),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub async fn microphone_apply_processor(stream_id: String, processor_name: String, options: String) -> napi::Result<bool> {
            let options: serde_json::Value = match serde_json::from_str(&options) {
                Ok(opts) => opts,
                Err(e) => return Err(napi::Error::from_reason(format!("Invalid processor options: {}", e)))
            };
            
            match Microphone::apply_processor(&stream_id, &processor_name, options).await {
                Ok(_) => Ok(true),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub async fn microphone_remove_processors(stream_id: String) -> napi::Result<bool> {
            match Microphone::remove_processors(&stream_id).await {
                Ok(_) => Ok(true),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub async fn microphone_recognize_speech(options: String) -> napi::Result<String> {
            let options: SpeechRecognitionOptions = match serde_json::from_str(&options) {
                Ok(opts) => opts,
                Err(e) => return Err(napi::Error::from_reason(format!("Invalid speech recognition options: {}", e)))
            };
            
            match Microphone::recognize_speech(options).await {
                Ok(result) => Ok(serde_json::to_string(&result).unwrap_or_default()),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
    }
    
    // Screen recording bindings
    pub mod screen {
        use super::*;
        use crate::screen::*;
        
        #[napi]
        pub fn screen_is_available() -> bool {
            Screen::is_available()
        }
        
        #[napi]
        pub async fn screen_request_permission() -> bool {
            Screen::request_permission().await.unwrap_or(false)
        }
        
        #[napi]
        pub async fn screen_get_sources(source_type: Option<String>) -> napi::Result<String> {
            match Screen::get_sources(source_type.as_deref()).await {
                Ok(sources) => Ok(serde_json::to_string(&sources).unwrap_or_default()),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub async fn screen_take_screenshot(source_id: Option<String>, options: String) -> napi::Result<String> {
            let options: ScreenshotOptions = match serde_json::from_str(&options) {
                Ok(opts) => opts,
                Err(e) => return Err(napi::Error::from_reason(format!("Invalid screenshot options: {}", e)))
            };
            
            match Screen::take_screenshot(source_id.as_deref(), options).await {
                Ok(screenshot) => Ok(serde_json::to_string(&screenshot).unwrap_or_default()),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub async fn screen_save_screenshot(screenshot_data: String, file_path: String) -> napi::Result<String> {
            match Screen::save_screenshot(&screenshot_data, &file_path).await {
                Ok(result) => Ok(serde_json::to_string(&result).unwrap_or_default()),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub async fn screen_start_recording(options: String) -> napi::Result<String> {
            let options: ScreenRecordingOptions = match serde_json::from_str(&options) {
                Ok(opts) => opts,
                Err(e) => return Err(napi::Error::from_reason(format!("Invalid recording options: {}", e)))
            };
            
            match Screen::start_recording(options).await {
                Ok(recorder) => Ok(serde_json::to_string(&recorder).unwrap_or_default()),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub async fn screen_stop_recording(recorder_id: String, file_path: String) -> napi::Result<String> {
            match Screen::stop_recording(&recorder_id, &file_path).await {
                Ok(result) => Ok(serde_json::to_string(&result).unwrap_or_default()),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub async fn screen_pause_recording(recorder_id: String) -> napi::Result<bool> {
            match Screen::pause_recording(&recorder_id).await {
                Ok(_) => Ok(true),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub async fn screen_resume_recording(recorder_id: String) -> napi::Result<bool> {
            match Screen::resume_recording(&recorder_id).await {
                Ok(_) => Ok(true),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub async fn screen_add_marker(recorder_id: String, label: String) -> napi::Result<bool> {
            match Screen::add_marker(&recorder_id, &label).await {
                Ok(_) => Ok(true),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub fn screen_get_display_server() -> String {
            #[cfg(target_os = "linux")]
            {
                match platform::linux::get_display_server_type() {
                    platform::linux::DisplayServer::X11 => String::from("x11"),
                    platform::linux::DisplayServer::Wayland => String::from("wayland"),
                    _ => String::from("unknown")
                }
            }
            #[cfg(not(target_os = "linux"))]
            {
                String::from("unknown")
            }
        }
    }
    
    // Input device bindings
    pub mod input {
        use super::*;
        use crate::input::*;
        
        #[napi]
        pub fn input_is_available(device_type: String) -> bool {
            Input::is_available(&device_type)
        }
        
        #[napi]
        pub async fn input_register_events(device_types: String) -> napi::Result<String> {
            let device_types: Vec<String> = match serde_json::from_str(&device_types) {
                Ok(types) => types,
                Err(e) => return Err(napi::Error::from_reason(format!("Invalid device types: {}", e)))
            };
            
            match Input::register_events(device_types).await {
                Ok(id) => Ok(id),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub fn input_unregister_events(registration_id: String) -> napi::Result<bool> {
            match Input::unregister_events(&registration_id) {
                Ok(_) => Ok(true),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub async fn input_simulate_input(event: String) -> napi::Result<bool> {
            let event: InputEvent = match serde_json::from_str(&event) {
                Ok(evt) => evt,
                Err(e) => return Err(napi::Error::from_reason(format!("Invalid input event: {}", e)))
            };
            
            match Input::simulate_input(event).await {
                Ok(_) => Ok(true),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub fn input_get_keyboard_state() -> napi::Result<String> {
            match Input::get_keyboard_state() {
                Ok(state) => Ok(serde_json::to_string(&state).unwrap_or_default()),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub fn input_get_mouse_position() -> napi::Result<String> {
            match Input::get_mouse_position() {
                Ok(position) => {
                    let pos_obj = serde_json::json!({
                        "x": position.0,
                        "y": position.1
                    });
                    Ok(serde_json::to_string(&pos_obj).unwrap_or_default())
                },
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        #[napi]
        pub fn input_get_touch_points() -> napi::Result<String> {
            match Input::get_touch_points() {
                Ok(points) => Ok(serde_json::to_string(&points).unwrap_or_default()),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
    }
    
    // Device management bindings
    pub mod devices {
        use super::*;
        use crate::devices::*;
        
        #[napi]
        pub async fn devices_get_all(device_type: Option<String>) -> napi::Result<String> {
            match get_all_devices(device_type.as_deref()).await {
                Ok(devices) => Ok(serde_json::to_string(&devices).unwrap_or_default()),
                Err(e) => Err(napi::Error::from_reason(e.to_string()))
            }
        }
        
        // Bluetooth bindings
        pub mod bluetooth {
            use super::*;
            use crate::devices::bluetooth::*;
            
            #[napi]
            pub fn bluetooth_is_available() -> bool {
                is_bluetooth_available()
            }
            
            #[napi]
            pub async fn bluetooth_enable() -> napi::Result<bool> {
                match enable_bluetooth().await {
                    Ok(_) => Ok(true),
                    Err(e) => Err(napi::Error::from_reason(e.to_string()))
                }
            }
            
            #[napi]
            pub async fn bluetooth_disable() -> napi::Result<bool> {
                match disable_bluetooth().await {
                    Ok(_) => Ok(true),
                    Err(e) => Err(napi::Error::from_reason(e.to_string()))
                }
            }
            
            #[napi]
            pub async fn bluetooth_scan(options: String) -> napi::Result<String> {
                let options: BluetoothScanOptions = match serde_json::from_str(&options) {
                    Ok(opts) => opts,
                    Err(e) => return Err(napi::Error::from_reason(format!("Invalid scan options: {}", e)))
                };
                
                match scan_bluetooth_devices(options).await {
                    Ok(devices) => Ok(serde_json::to_string(&devices).unwrap_or_default()),
                    Err(e) => Err(napi::Error::from_reason(e.to_string()))
                }
            }
            
            #[napi]
            pub async fn bluetooth_connect(device_id: String) -> napi::Result<String> {
                match connect_bluetooth_device(&device_id).await {
                    Ok(connection) => Ok(serde_json::to_string(&connection).unwrap_or_default()),
                    Err(e) => Err(napi::Error::from_reason(e.to_string()))
                }
            }
            
            #[napi]
            pub async fn bluetooth_disconnect(device_id: String) -> napi::Result<bool> {
                match disconnect_bluetooth_device(&device_id).await {
                    Ok(_) => Ok(true),
                    Err(e) => Err(napi::Error::from_reason(e.to_string()))
                }
            }
        }
        
        // USB bindings
        pub mod usb {
            use super::*;
            use crate::devices::usb::*;
            
            #[napi]
            pub fn usb_is_available() -> bool {
                is_usb_available()
            }
            
            #[napi]
            pub async fn usb_get_devices() -> napi::Result<String> {
                match get_usb_devices().await {
                    Ok(devices) => Ok(serde_json::to_string(&devices).unwrap_or_default()),
                    Err(e) => Err(napi::Error::from_reason(e.to_string()))
                }
            }
            
            #[napi]
            pub async fn usb_request_permission(device_id: String) -> napi::Result<bool> {
                match request_usb_permission(&device_id).await {
                    Ok(granted) => Ok(granted),
                    Err(e) => Err(napi::Error::from_reason(e.to_string()))
                }
            }
            
            #[napi]
            pub async fn usb_open(device_id: String) -> napi::Result<String> {
                match open_usb_device(&device_id).await {
                    Ok(connection) => Ok(serde_json::to_string(&connection).unwrap_or_default()),
                    Err(e) => Err(napi::Error::from_reason(e.to_string()))
                }
            }
            
            #[napi]
            pub async fn usb_close(device_id: String) -> napi::Result<bool> {
                match close_usb_device(&device_id).await {
                    Ok(_) => Ok(true),
                    Err(e) => Err(napi::Error::from_reason(e.to_string()))
                }
            }
            
            #[napi]
            pub async fn usb_transfer(device_id: String, options: String) -> napi::Result<String> {
                let options: UsbTransferOptions = match serde_json::from_str(&options) {
                    Ok(opts) => opts,
                    Err(e) => return Err(napi::Error::from_reason(format!("Invalid transfer options: {}", e)))
                };
                
                match transfer_usb_data(&device_id, options).await {
                    Ok(result) => Ok(serde_json::to_string(&result).unwrap_or_default()),
                    Err(e) => Err(napi::Error::from_reason(e.to_string()))
                }
            }
        }
        
        // MIDI bindings
        pub mod midi {
            use super::*;
            use crate::devices::midi::*;
            
            #[napi]
            pub fn midi_is_available() -> bool {
                is_midi_available()
            }
            
            #[napi]
            pub async fn midi_get_inputs() -> napi::Result<String> {
                match get_midi_inputs().await {
                    Ok(inputs) => Ok(serde_json::to_string(&inputs).unwrap_or_default()),
                    Err(e) => Err(napi::Error::from_reason(e.to_string()))
                }
            }
            
            #[napi]
            pub async fn midi_get_outputs() -> napi::Result<String> {
                match get_midi_outputs().await {
                    Ok(outputs) => Ok(serde_json::to_string(&outputs).unwrap_or_default()),
                    Err(e) => Err(napi::Error::from_reason(e.to_string()))
                }
            }
            
            // Note: MIDI callbacks require special handling with Node.js
            // This is a simplified version
            #[napi]
            pub async fn midi_open_input(device_id: String) -> napi::Result<String> {
                match open_midi_input(&device_id).await {
                    Ok(connection) => Ok(serde_json::to_string(&connection).unwrap_or_default()),
                    Err(e) => Err(napi::Error::from_reason(e.to_string()))
                }
            }
            
            #[napi]
            pub async fn midi_open_output(device_id: String) -> napi::Result<String> {
                match open_midi_output(&device_id).await {
                    Ok(connection) => Ok(serde_json::to_string(&connection).unwrap_or_default()),
                    Err(e) => Err(napi::Error::from_reason(e.to_string()))
                }
            }
            
            #[napi]
            pub async fn midi_close(device_id: String, device_type: String) -> napi::Result<bool> {
                match close_midi_device(&device_id, &device_type).await {
                    Ok(_) => Ok(true),
                    Err(e) => Err(napi::Error::from_reason(e.to_string()))
                }
            }
            
            #[napi]
            pub async fn midi_send(device_id: String, message: String) -> napi::Result<bool> {
                let message: Vec<u8> = match serde_json::from_str(&message) {
                    Ok(msg) => msg,
                    Err(e) => return Err(napi::Error::from_reason(format!("Invalid MIDI message: {}", e)))
                };
                
                match send_midi_message(&device_id, &message).await {
                    Ok(_) => Ok(true),
                    Err(e) => Err(napi::Error::from_reason(e.to_string()))
                }
            }
        }
        
        // Gamepad bindings
        pub mod gamepad {
            use super::*;
            use crate::devices::gamepad::*;
            
            #[napi]
            pub fn gamepad_is_available() -> bool {
                is_gamepad_available()
            }
            
            #[napi]
            pub async fn gamepad_get_devices() -> napi::Result<String> {
                match get_gamepad_devices().await {
                    Ok(devices) => Ok(serde_json::to_string(&devices).unwrap_or_default()),
                    Err(e) => Err(napi::Error::from_reason(e.to_string()))
                }
            }
            
            // Note: Gamepad events require special handling with Node.js
            // This is a simplified version
            #[napi]
            pub async fn gamepad_register_events() -> napi::Result<bool> {
                match register_gamepad_events().await {
                    Ok(_) => Ok(true),
                    Err(e) => Err(napi::Error::from_reason(e.to_string()))
                }
            }
            
            #[napi]
            pub async fn gamepad_unregister_events() -> napi::Result<bool> {
                match unregister_gamepad_events().await {
                    Ok(_) => Ok(true),
                    Err(e) => Err(napi::Error::from_reason(e.to_string()))
                }
            }
            
            #[napi]
            pub async fn gamepad_get_state(device_id: String) -> napi::Result<String> {
                match get_gamepad_state(&device_id).await {
                    Ok(state) => Ok(serde_json::to_string(&state).unwrap_or_default()),
                    Err(e) => Err(napi::Error::from_reason(e.to_string()))
                }
            }
        }
    }
}
