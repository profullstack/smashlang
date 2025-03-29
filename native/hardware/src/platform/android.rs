//! Android-specific implementation for screen recording
//!
//! This module provides screen recording capabilities for Android devices
//! using the MediaProjection API through JNI bindings.

use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use uuid::Uuid;
use log::{info, warn, error};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

#[cfg(target_os = "android")]
use {
    jni::{JNIEnv, JavaVM, objects::{JObject, JString, JValue, JClass}, sys::jobject},
    ndk::media::{MediaCodec, MediaFormat},
    ndk_context::AndroidContext,
};

use crate::error::HardwareError;
use crate::screen::{ScreenSource, Screenshot, ScreenshotOptions, ScreenRecorder, ScreenRecordingOptions, RecordingResult, SaveResult, RecordingInstance};
use crate::Result;
use crate::platform::common::ScreenCapture;

/// Android implementation of screen capture
pub struct AndroidScreenCapture {
    #[cfg(target_os = "android")]
    vm: Option<JavaVM>,
    recording_instances: Arc<Mutex<HashMap<String, RecordingInstance>>>,
}

impl AndroidScreenCapture {
    /// Create a new instance of AndroidScreenCapture
    pub fn new() -> Self {
        #[cfg(target_os = "android")]
        let vm = unsafe {
            AndroidContext::get().vm()
        };
        
        Self {
            #[cfg(target_os = "android")]
            vm: Some(vm),
            #[cfg(not(target_os = "android"))]
            vm: None,
            recording_instances: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    #[cfg(target_os = "android")]
    fn get_env(&self) -> Result<JNIEnv> {
        match &self.vm {
            Some(vm) => match vm.get_env() {
                Ok(env) => Ok(env),
                Err(e) => Err(HardwareError::DeviceAccessError(format!("Failed to get JNI environment: {}", e))),
            },
            None => Err(HardwareError::DeviceAccessError("Java VM not initialized".to_string())),
        }
    }
    
    #[cfg(target_os = "android")]
    fn has_screen_capture_permission(&self) -> Result<bool> {
        let env = self.get_env()?;
        
        // Find the SmashLangActivity class
        let activity_class = match env.find_class("com/profullstack/smashlang/SmashLangActivity") {
            Ok(class) => class,
            Err(e) => return Err(HardwareError::DeviceAccessError(format!("Failed to find SmashLangActivity class: {}", e))),
        };
        
        // Get the current activity instance
        let activity = match env.call_static_method(
            activity_class,
            "getCurrentActivity",
            "()Lcom/profullstack/smashlang/SmashLangActivity;",
            &[]
        ) {
            Ok(obj) => obj.l()?,
            Err(e) => return Err(HardwareError::DeviceAccessError(format!("Failed to get current activity: {}", e))),
        };
        
        // Check if we have screen capture permission
        match env.call_method(
            activity,
            "hasScreenCapturePermission",
            "()Z",
            &[]
        ) {
            Ok(result) => Ok(result.z()?),
            Err(e) => Err(HardwareError::DeviceAccessError(format!("Failed to check screen capture permission: {}", e))),
        }
    }
    
    #[cfg(target_os = "android")]
    fn request_screen_capture_permission(&self) -> Result<bool> {
        let env = self.get_env()?;
        
        // Find the SmashLangActivity class
        let activity_class = match env.find_class("com/profullstack/smashlang/SmashLangActivity") {
            Ok(class) => class,
            Err(e) => return Err(HardwareError::DeviceAccessError(format!("Failed to find SmashLangActivity class: {}", e))),
        };
        
        // Get the current activity instance
        let activity = match env.call_static_method(
            activity_class,
            "getCurrentActivity",
            "()Lcom/profullstack/smashlang/SmashLangActivity;",
            &[]
        ) {
            Ok(obj) => obj.l()?,
            Err(e) => return Err(HardwareError::DeviceAccessError(format!("Failed to get current activity: {}", e))),
        };
        
        // Request screen capture permission
        match env.call_method(
            activity,
            "requestScreenCapturePermission",
            "()Z",
            &[]
        ) {
            Ok(result) => Ok(result.z()?),
            Err(e) => Err(HardwareError::DeviceAccessError(format!("Failed to request screen capture permission: {}", e))),
        }
    }
    
    #[cfg(target_os = "android")]
    fn get_display_metrics(&self) -> Result<(i32, i32)> {
        let env = self.get_env()?;
        
        // Find the SmashLangActivity class
        let activity_class = match env.find_class("com/profullstack/smashlang/SmashLangActivity") {
            Ok(class) => class,
            Err(e) => return Err(HardwareError::DeviceAccessError(format!("Failed to find SmashLangActivity class: {}", e))),
        };
        
        // Get the current activity instance
        let activity = match env.call_static_method(
            activity_class,
            "getCurrentActivity",
            "()Lcom/profullstack/smashlang/SmashLangActivity;",
            &[]
        ) {
            Ok(obj) => obj.l()?,
            Err(e) => return Err(HardwareError::DeviceAccessError(format!("Failed to get current activity: {}", e))),
        };
        
        // Get display width
        let width = match env.call_method(
            activity,
            "getDisplayWidth",
            "()I",
            &[]
        ) {
            Ok(result) => result.i()?,
            Err(e) => return Err(HardwareError::DeviceAccessError(format!("Failed to get display width: {}", e))),
        };
        
        // Get display height
        let height = match env.call_method(
            activity,
            "getDisplayHeight",
            "()I",
            &[]
        ) {
            Ok(result) => result.i()?,
            Err(e) => return Err(HardwareError::DeviceAccessError(format!("Failed to get display height: {}", e))),
        };
        
        Ok((width, height))
    }
}
