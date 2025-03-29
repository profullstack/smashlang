//! Platform-specific implementations for hardware interfaces
//! 
//! This module contains platform-specific code for different operating systems,
//! including mobile platforms like Android and iOS.

// Desktop platforms
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub mod linux;

// Mobile platforms
#[cfg(target_os = "android")]
pub mod android;

#[cfg(target_os = "ios")]
pub mod ios;

// Common platform-agnostic traits and utilities
pub mod common;
