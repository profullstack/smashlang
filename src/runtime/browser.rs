use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::interpreter::{Value, Function, Environment};

/// Represents a browser-like environment in the SmashLang runtime
#[derive(Debug, Clone)]
pub struct BrowserEnvironment {
    /// Local storage
    local_storage: Arc<Mutex<HashMap<String, String>>>,
    
    /// Session storage
    session_storage: Arc<Mutex<HashMap<String, String>>>,
    
    /// Cookies
    cookies: Arc<Mutex<HashMap<String, String>>>,
    
    /// URL
    url: Arc<Mutex<String>>,
    
    /// User agent
    user_agent: String,
    
    /// Platform
    platform: String,
    
    /// Language
    language: String,
}

impl BrowserEnvironment {
    /// Create a new browser environment
    pub fn new() -> Self {
        Self {
            local_storage: Arc::new(Mutex::new(HashMap::new())),
            session_storage: Arc::new(Mutex::new(HashMap::new())),
            cookies: Arc::new(Mutex::new(HashMap::new())),
            url: Arc::new(Mutex::new("http://localhost".to_string())),
            user_agent: "SmashLang/1.0".to_string(),
            platform: "SmashLang".to_string(),
            language: "en-US".to_string(),
        }
    }
    
    /// Create a localStorage object
    pub fn create_local_storage(&self) -> Value {
        let local_storage = self.local_storage.clone();
        
        let mut storage_obj = HashMap::new();
        
        // getItem
        storage_obj.insert("getItem".to_string(), Value::Function(Function::new_native(
            Some("getItem".to_string()),
            vec!["key".to_string()],
            move |_this, args, _env| {
                if args.is_empty() {
                    return Ok(Value::Null);
                }
                
                let key = match &args[0] {
                    Value::String(s) => s.clone(),
                    _ => args[0].to_string(),
                };
                
                let storage = local_storage.lock().unwrap();
                
                if let Some(value) = storage.get(&key) {
                    Ok(Value::String(value.clone()))
                } else {
                    Ok(Value::Null)
                }
            },
        )));
        
        // setItem
        let local_storage_set = self.local_storage.clone();
        storage_obj.insert("setItem".to_string(), Value::Function(Function::new_native(
            Some("setItem".to_string()),
            vec!["key".to_string(), "value".to_string()],
            move |_this, args, _env| {
                if args.len() < 2 {
                    return Err("localStorage.setItem requires two arguments".to_string());
                }
                
                let key = match &args[0] {
                    Value::String(s) => s.clone(),
                    _ => args[0].to_string(),
                };
                
                let value = match &args[1] {
                    Value::String(s) => s.clone(),
                    _ => args[1].to_string(),
                };
                
                let mut storage = local_storage_set.lock().unwrap();
                storage.insert(key, value);
                
                Ok(Value::Undefined)
            },
        )));
        
        // removeItem
        let local_storage_remove = self.local_storage.clone();
        storage_obj.insert("removeItem".to_string(), Value::Function(Function::new_native(
            Some("removeItem".to_string()),
            vec!["key".to_string()],
            move |_this, args, _env| {
                if args.is_empty() {
                    return Ok(Value::Undefined);
                }
                
                let key = match &args[0] {
                    Value::String(s) => s.clone(),
                    _ => args[0].to_string(),
                };
                
                let mut storage = local_storage_remove.lock().unwrap();
                storage.remove(&key);
                
                Ok(Value::Undefined)
            },
        )));
        
        // clear
        let local_storage_clear = self.local_storage.clone();
        storage_obj.insert("clear".to_string(), Value::Function(Function::new_native(
            Some("clear".to_string()),
            vec![],
            move |_this, _args, _env| {
                let mut storage = local_storage_clear.lock().unwrap();
                storage.clear();
                
                Ok(Value::Undefined)
            },
        )));
        
        // key
        let local_storage_key = self.local_storage.clone();
        storage_obj.insert("key".to_string(), Value::Function(Function::new_native(
            Some("key".to_string()),
            vec!["index".to_string()],
            move |_this, args, _env| {
                if args.is_empty() {
                    return Ok(Value::Null);
                }
                
                let index = match &args[0] {
                    Value::Number(n) => {
                        if n.fract() == 0.0 && *n >= 0.0 {
                            *n as usize
                        } else {
                            return Ok(Value::Null);
                        }
                    },
                    _ => return Ok(Value::Null),
                };
                
                let storage = local_storage_key.lock().unwrap();
                let keys: Vec<&String> = storage.keys().collect();
                
                if index < keys.len() {
                    Ok(Value::String(keys[index].clone()))
                } else {
                    Ok(Value::Null)
                }
            },
        )));
        
        // length property
        let local_storage_length = self.local_storage.clone();
        let length_getter = Function::new_native(
            Some("get length".to_string()),
            vec![],
            move |_this, _args, _env| {
                let storage = local_storage_length.lock().unwrap();
                Ok(Value::Number(storage.len() as f64))
            },
        );
        
        let mut length_descriptor = HashMap::new();
        length_descriptor.insert("get".to_string(), Value::Function(length_getter));
        length_descriptor.insert("enumerable".to_string(), Value::Boolean(true));
        
        let mut storage_obj_with_descriptors = HashMap::new();
        for (key, value) in storage_obj {
            storage_obj_with_descriptors.insert(key, value);
        }
        
        // Add length property descriptor
        storage_obj_with_descriptors.insert("length".to_string(), Value::Object(length_descriptor));
        
        Value::Object(storage_obj_with_descriptors)
    }
    
    /// Create a sessionStorage object
    pub fn create_session_storage(&self) -> Value {
        let session_storage = self.session_storage.clone();
        
        let mut storage_obj = HashMap::new();
        
        // getItem
        storage_obj.insert("getItem".to_string(), Value::Function(Function::new_native(
            Some("getItem".to_string()),
            vec!["key".to_string()],
            move |_this, args, _env| {
                if args.is_empty() {
                    return Ok(Value::Null);
                }
                
                let key = match &args[0] {
                    Value::String(s) => s.clone(),
                    _ => args[0].to_string(),
                };
                
                let storage = session_storage.lock().unwrap();
                
                if let Some(value) = storage.get(&key) {
                    Ok(Value::String(value.clone()))
                } else {
                    Ok(Value::Null)
                }
            },
        )));
        
        // setItem
        let session_storage_set = self.session_storage.clone();
        storage_obj.insert("setItem".to_string(), Value::Function(Function::new_native(
            Some("setItem".to_string()),
            vec!["key".to_string(), "value".to_string()],
            move |_this, args, _env| {
                if args.len() < 2 {
                    return Err("sessionStorage.setItem requires two arguments".to_string());
                }
                
                let key = match &args[0] {
                    Value::String(s) => s.clone(),
                    _ => args[0].to_string(),
                };
                
                let value = match &args[1] {
                    Value::String(s) => s.clone(),
                    _ => args[1].to_string(),
                };
                
                let mut storage = session_storage_set.lock().unwrap();
                storage.insert(key, value);
                
                Ok(Value::Undefined)
            },
        )));
        
        // removeItem
        let session_storage_remove = self.session_storage.clone();
        storage_obj.insert("removeItem".to_string(), Value::Function(Function::new_native(
            Some("removeItem".to_string()),
            vec!["key".to_string()],
            move |_this, args, _env| {
                if args.is_empty() {
                    return Ok(Value::Undefined);
                }
                
                let key = match &args[0] {
                    Value::String(s) => s.clone(),
                    _ => args[0].to_string(),
                };
                
                let mut storage = session_storage_remove.lock().unwrap();
                storage.remove(&key);
                
                Ok(Value::Undefined)
            },
        )));
        
        // clear
        let session_storage_clear = self.session_storage.clone();
        storage_obj.insert("clear".to_string(), Value::Function(Function::new_native(
            Some("clear".to_string()),
            vec![],
            move |_this, _args, _env| {
                let mut storage = session_storage_clear.lock().unwrap();
                storage.clear();
                
                Ok(Value::Undefined)
            },
        )));
        
        // key
        let session_storage_key = self.session_storage.clone();
        storage_obj.insert("key".to_string(), Value::Function(Function::new_native(
            Some("key".to_string()),
            vec!["index".to_string()],
            move |_this, args, _env| {
                if args.is_empty() {
                    return Ok(Value::Null);
                }
                
                let index = match &args[0] {
                    Value::Number(n) => {
                        if n.fract() == 0.0 && *n >= 0.0 {
                            *n as usize
                        } else {
                            return Ok(Value::Null);
                        }
                    },
                    _ => return Ok(Value::Null),
                };
                
                let storage = session_storage_key.lock().unwrap();
                let keys: Vec<&String> = storage.keys().collect();
                
                if index < keys.len() {
                    Ok(Value::String(keys[index].clone()))
                } else {
                    Ok(Value::Null)
                }
            },
        )));
        
        // length property
        let session_storage_length = self.session_storage.clone();
        let length_getter = Function::new_native(
            Some("get length".to_string()),
            vec![],
            move |_this, _args, _env| {
                let storage = session_storage_length.lock().unwrap();
                Ok(Value::Number(storage.len() as f64))
            },
        );
        
        let mut length_descriptor = HashMap::new();
        length_descriptor.insert("get".to_string(), Value::Function(length_getter));
        length_descriptor.insert("enumerable".to_string(), Value::Boolean(true));
        
        let mut storage_obj_with_descriptors = HashMap::new();
        for (key, value) in storage_obj {
            storage_obj_with_descriptors.insert(key, value);
        }
        
        // Add length property descriptor
        storage_obj_with_descriptors.insert("length".to_string(), Value::Object(length_descriptor));
        
        Value::Object(storage_obj_with_descriptors)
    }
    
    /// Create a location object
    pub fn create_location(&self) -> Value {
        let url = self.url.clone();
        
        let mut location_obj = HashMap::new();
        
        // href property
        let url_href = url.clone();
        let href_getter = Function::new_native(
            Some("get href".to_string()),
            vec![],
            move |_this, _args, _env| {
                let url_str = url_href.lock().unwrap();
                Ok(Value::String(url_str.clone()))
            },
        );
        
        let url_href_set = url.clone();
        let href_setter = Function::new_native(
            Some("set href".to_string()),
            vec!["value".to_string()],
            move |_this, args, _env| {
                if args.is_empty() {
                    return Ok(Value::Undefined);
                }
                
                let new_url = match &args[0] {
                    Value::String(s) => s.clone(),
                    _ => args[0].to_string(),
                };
                
                let mut url_str = url_href_set.lock().unwrap();
                *url_str = new_url;
                
                Ok(Value::Undefined)
            },
        );
        
        let mut href_descriptor = HashMap::new();
        href_descriptor.insert("get".to_string(), Value::Function(href_getter));
        href_descriptor.insert("set".to_string(), Value::Function(href_setter));
        href_descriptor.insert("enumerable".to_string(), Value::Boolean(true));
        
        location_obj.insert("href".to_string(), Value::Object(href_descriptor));
        
        // pathname property
        let url_pathname = url.clone();
        let pathname_getter = Function::new_native(
            Some("get pathname".to_string()),
            vec![],
            move |_this, _args, _env| {
                let url_str = url_pathname.lock().unwrap();
                
                if let Ok(parsed_url) = url::Url::parse(&url_str) {
                    Ok(Value::String(parsed_url.path().to_string()))
                } else {
                    Ok(Value::String("/".to_string()))
                }
            },
        );
        
        let mut pathname_descriptor = HashMap::new();
        pathname_descriptor.insert("get".to_string(), Value::Function(pathname_getter));
        pathname_descriptor.insert("enumerable".to_string(), Value::Boolean(true));
        
        location_obj.insert("pathname".to_string(), Value::Object(pathname_descriptor));
        
        // host property
        let url_host = url.clone();
        let host_getter = Function::new_native(
            Some("get host".to_string()),
            vec![],
            move |_this, _args, _env| {
                let url_str = url_host.lock().unwrap();
                
                if let Ok(parsed_url) = url::Url::parse(&url_str) {
                    Ok(Value::String(parsed_url.host_str().unwrap_or("").to_string()))
                } else {
                    Ok(Value::String("".to_string()))
                }
            },
        );
        
        let mut host_descriptor = HashMap::new();
        host_descriptor.insert("get".to_string(), Value::Function(host_getter));
        host_descriptor.insert("enumerable".to_string(), Value::Boolean(true));
        
        location_obj.insert("host".to_string(), Value::Object(host_descriptor));
        
        // protocol property
        let url_protocol = url.clone();
        let protocol_getter = Function::new_native(
            Some("get protocol".to_string()),
            vec![],
            move |_this, _args, _env| {
                let url_str = url_protocol.lock().unwrap();
                
                if let Ok(parsed_url) = url::Url::parse(&url_str) {
                    Ok(Value::String(parsed_url.scheme().to_string() + ":"))
                } else {
                    Ok(Value::String("http:".to_string()))
                }
            },
        );
        
        let mut protocol_descriptor = HashMap::new();
        protocol_descriptor.insert("get".to_string(), Value::Function(protocol_getter));
        protocol_descriptor.insert("enumerable".to_string(), Value::Boolean(true));
        
        location_obj.insert("protocol".to_string(), Value::Object(protocol_descriptor));
        
        // search property
        let url_search = url.clone();
        let search_getter = Function::new_native(
            Some("get search".to_string()),
            vec![],
            move |_this, _args, _env| {
                let url_str = url_search.lock().unwrap();
                
                if let Ok(parsed_url) = url::Url::parse(&url_str) {
                    if parsed_url.query().is_some() {
                        Ok(Value::String("?".to_string() + parsed_url.query().unwrap()))
                    } else {
                        Ok(Value::String("".to_string()))
                    }
                } else {
                    Ok(Value::String("".to_string()))
                }
            },
        );
        
        let mut search_descriptor = HashMap::new();
        search_descriptor.insert("get".to_string(), Value::Function(search_getter));
        search_descriptor.insert("enumerable".to_string(), Value::Boolean(true));
        
        location_obj.insert("search".to_string(), Value::Object(search_descriptor));
        
        // hash property
        let url_hash = url.clone();
        let hash_getter = Function::new_native(
            Some("get hash".to_string()),
            vec![],
            move |_this, _args, _env| {
                let url_str = url_hash.lock().unwrap();
                
                if let Ok(parsed_url) = url::Url::parse(&url_str) {
                    if parsed_url.fragment().is_some() {
                        Ok(Value::String("#".to_string() + parsed_url.fragment().unwrap()))
                    } else {
                        Ok(Value::String("".to_string()))
                    }
                } else {
                    Ok(Value::String("".to_string()))
                }
            },
        );
        
        let mut hash_descriptor = HashMap::new();
        hash_descriptor.insert("get".to_string(), Value::Function(hash_getter));
        hash_descriptor.insert("enumerable".to_string(), Value::Boolean(true));
        
        location_obj.insert("hash".to_string(), Value::Object(hash_descriptor));
        
        Value::Object(location_obj)
    }
    
    /// Create a navigator object
    pub fn create_navigator(&self) -> Value {
        let mut navigator_obj = HashMap::new();
        
        // userAgent property
        navigator_obj.insert("userAgent".to_string(), Value::String(self.user_agent.clone()));
        
        // platform property
        navigator_obj.insert("platform".to_string(), Value::String(self.platform.clone()));
        
        // language property
        navigator_obj.insert("language".to_string(), Value::String(self.language.clone()));
        
        // onLine property
        navigator_obj.insert("onLine".to_string(), Value::Boolean(true));
        
        Value::Object(navigator_obj)
    }
    
    /// Create a document object
    pub fn create_document(&self) -> Value {
        let mut document_obj = HashMap::new();
        
        // title property
        document_obj.insert("title".to_string(), Value::String("SmashLang Document".to_string()));
        
        // cookie property
        let cookies = self.cookies.clone();
        let cookie_getter = Function::new_native(
            Some("get cookie".to_string()),
            vec![],
            move |_this, _args, _env| {
                let cookies_map = cookies.lock().unwrap();
                
                let cookie_str = cookies_map.iter()
                    .map(|(key, value)| format!("{}={}", key, value))
                    .collect::<Vec<String>>()
                    .join("; ");
                
                Ok(Value::String(cookie_str))
            },
        );
        
        let cookies_set = self.cookies.clone();
        let cookie_setter = Function::new_native(
            Some("set cookie".to_string()),
            vec!["value".to_string()],
            move |_this, args, _env| {
                if args.is_empty() {
                    return Ok(Value::Undefined);
                }
                
                let cookie_str = match &args[0] {
                    Value::String(s) => s.clone(),
                    _ => args[0].to_string(),
                };
                
                // Parse cookie string
                let parts: Vec<&str> = cookie_str.split(';').collect();
                if let Some(name_value) = parts.first() {
                    let name_value_parts: Vec<&str> = name_value.split('=').collect();
                    if name_value_parts.len() >= 2 {
                        let name = name_value_parts[0].trim();
                        let value = name_value_parts[1].trim();
                        
                        let mut cookies_map = cookies_set.lock().unwrap();
                        cookies_map.insert(name.to_string(), value.to_string());
                    }
                }
                
                Ok(Value::Undefined)
            },
        );
        
        let mut cookie_descriptor = HashMap::new();
        cookie_descriptor.insert("get".to_string(), Value::Function(cookie_getter));
        cookie_descriptor.insert("set".to_string(), Value::Function(cookie_setter));
        cookie_descriptor.insert("enumerable".to_string(), Value::Boolean(true));
        
        document_obj.insert("cookie".to_string(), Value::Object(cookie_descriptor));
        
        Value::Object(document_obj)
    }
    
    /// Create a window object
    pub fn create_window(&self) -> Value {
        let mut window_obj = HashMap::new();
        
        // localStorage
        window_obj.insert("localStorage".to_string(), self.create_local_storage());
        
        // sessionStorage
        window_obj.insert("sessionStorage".to_string(), self.create_session_storage());
        
        // location
        window_obj.insert("location".to_string(), self.create_location());
        
        // navigator
        window_obj.insert("navigator".to_string(), self.create_navigator());
        
        // document
        window_obj.insert("document".to_string(), self.create_document());
        
        Value::Object(window_obj)
    }
}