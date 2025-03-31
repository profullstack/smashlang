use regex::Regex;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;

// Type to represent a regex in C code
pub struct SmashRegex {
    pattern: Regex,
    original_pattern: String,
    flags: String,
}

#[no_mangle]
pub extern "C" fn smash_regex_create(pattern: *const c_char, flags: *const c_char) -> *mut SmashRegex {
    // Safety checks for null pointers
    if pattern.is_null() {
        return ptr::null_mut();
    }
    
    // Convert C strings to Rust strings
    let pattern_str = unsafe {
        match CStr::from_ptr(pattern).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut(),
        }
    };
    
    let flags_str = if flags.is_null() {
        ""
    } else {
        unsafe {
            match CStr::from_ptr(flags).to_str() {
                Ok(s) => s,
                Err(_) => return ptr::null_mut(),
            }
        }
    };
    
    // Build the full pattern with flags
    let mut full_pattern = String::new();
    
    // Add case-insensitive flag
    if flags_str.contains('i') {
        full_pattern.push_str("(?i)");
    }
    
    // Add multiline flag
    if flags_str.contains('m') {
        full_pattern.push_str("(?m)");
    }
    
    // Add dotall flag (s)
    if flags_str.contains('s') {
        full_pattern.push_str("(?s)");
    }
    
    // Add the actual pattern
    full_pattern.push_str(pattern_str);
    
    // Create the regex
    let regex = match Regex::new(&full_pattern) {
        Ok(r) => r,
        Err(_) => return ptr::null_mut(),
    };
    
    // Create and return the SmashRegex
    let smash_regex = Box::new(SmashRegex {
        pattern: regex,
        original_pattern: pattern_str.to_string(),
        flags: flags_str.to_string(),
    });
    
    Box::into_raw(smash_regex)
}

#[no_mangle]
pub extern "C" fn smash_regex_free(regex_ptr: *mut SmashRegex) {
    if !regex_ptr.is_null() {
        unsafe {
            // Convert the raw pointer back to a Box and drop it
            let _ = Box::from_raw(regex_ptr);
        }
    }
}

#[no_mangle]
pub extern "C" fn smash_regex_test(regex_ptr: *const SmashRegex, text: *const c_char) -> c_int {
    // Safety checks
    if regex_ptr.is_null() || text.is_null() {
        return 0;
    }
    
    let regex = unsafe { &*regex_ptr };
    let text_str = unsafe {
        match CStr::from_ptr(text).to_str() {
            Ok(s) => s,
            Err(_) => return 0,
        }
    };
    
    // Return 1 for match, 0 for no match
    if regex.pattern.is_match(text_str) { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn smash_regex_match(regex_ptr: *const SmashRegex, text: *const c_char) -> *mut c_char {
    // Safety checks
    if regex_ptr.is_null() || text.is_null() {
        return ptr::null_mut();
    }
    
    let regex = unsafe { &*regex_ptr };
    let text_str = unsafe {
        match CStr::from_ptr(text).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut(),
        }
    };
    
    // Find all matches
    let mut matches = Vec::new();
    for cap in regex.pattern.captures_iter(text_str) {
        if let Some(m) = cap.get(0) {
            matches.push(m.as_str());
        }
    }
    
    // If no matches, return null
    if matches.is_empty() {
        return ptr::null_mut();
    }
    
    // Convert matches to a JSON string
    let json = serde_json::to_string(&matches).unwrap_or_else(|_| "[]".to_string());
    
    // Convert to C string
    match CString::new(json) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn smash_regex_replace(regex_ptr: *const SmashRegex, text: *const c_char, replacement: *const c_char) -> *mut c_char {
    // Safety checks
    if regex_ptr.is_null() || text.is_null() || replacement.is_null() {
        return ptr::null_mut();
    }
    
    let regex = unsafe { &*regex_ptr };
    let text_str = unsafe {
        match CStr::from_ptr(text).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut(),
        }
    };
    
    let replacement_str = unsafe {
        match CStr::from_ptr(replacement).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut(),
        }
    };
    
    // Perform the replacement
    let result = regex.pattern.replace_all(text_str, replacement_str);
    
    // Convert to C string
    match CString::new(result.as_ref()) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

// Helper function to free C strings returned by our functions
#[no_mangle]
pub extern "C" fn smash_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}
