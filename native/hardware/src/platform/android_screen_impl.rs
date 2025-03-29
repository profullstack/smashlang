#[async_trait]
impl ScreenCapture for AndroidScreenCapture {
    async fn is_available(&self) -> bool {
        #[cfg(target_os = "android")]
        {
            // Check if we're running on Android and have the necessary permissions
            match self.has_screen_capture_permission() {
                Ok(has_permission) => has_permission,
                Err(_) => false,
            }
        }
        
        #[cfg(not(target_os = "android"))]
        {
            false // Not available on non-Android platforms
        }
    }
    
    async fn request_permission(&self) -> Result<bool> {
        #[cfg(target_os = "android")]
        {
            // Request MediaProjection permission on Android
            self.request_screen_capture_permission()
        }
        
        #[cfg(not(target_os = "android"))]
        {
            Err(HardwareError::UnsupportedPlatform("Android screen recording is not available on this platform".to_string()))
        }
    }
    
    async fn get_sources(&self, source_type: Option<&str>) -> Result<Vec<ScreenSource>> {
        #[cfg(target_os = "android")]
        {
            let mut sources = Vec::new();
            let type_filter = source_type.unwrap_or("all");
            
            // On Android, typically only the full screen is available
            if type_filter == "all" || type_filter == "screen" {
                // Get display metrics
                if let Ok((width, height)) = self.get_display_metrics() {
                    sources.push(ScreenSource {
                        id: "screen_0".to_string(),
                        name: format!("Android Display ({}x{})", width, height),
                        source_type: "screen".to_string(),
                        thumbnail: None,
                    });
                }
            }
            
            // On Android, we can also get the current foreground app
            if type_filter == "all" || type_filter == "application" {
                let env = self.get_env()?;
                
                // Find the SmashLangActivity class
                if let Ok(activity_class) = env.find_class("com/profullstack/smashlang/SmashLangActivity") {
                    // Get the current activity instance
                    if let Ok(activity) = env.call_static_method(
                        activity_class,
                        "getCurrentActivity",
                        "()Lcom/profullstack/smashlang/SmashLangActivity;",
                        &[]
                    ) {
                        if let Ok(activity_obj) = activity.l() {
                            // Get foreground app name
                            if let Ok(app_name) = env.call_method(
                                activity_obj,
                                "getForegroundAppName",
                                "()Ljava/lang/String;",
                                &[]
                            ) {
                                if let Ok(app_name_obj) = app_name.l() {
                                    if let Ok(app_name_str) = env.get_string(JString::from(app_name_obj)) {
                                        sources.push(ScreenSource {
                                            id: "app_foreground".to_string(),
                                            name: format!("Current App: {}", app_name_str.to_string_lossy().to_string()),
                                            source_type: "application".to_string(),
                                            thumbnail: None,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            Ok(sources)
        }
        
        #[cfg(not(target_os = "android"))]
        {
            Err(HardwareError::UnsupportedPlatform("Android screen recording is not available on this platform".to_string()))
        }
    }
    
    async fn take_screenshot(&self, source_id: Option<&str>, options: ScreenshotOptions) -> Result<Screenshot> {
        #[cfg(target_os = "android")]
        {
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
            
            // Take screenshot
            let screenshot_result = match env.call_method(
                activity,
                "takeScreenshot",
                "()[B",
                &[]
            ) {
                Ok(result) => result.l()?,
                Err(e) => return Err(HardwareError::DeviceAccessError(format!("Failed to take screenshot: {}", e))),
            };
            
            // Convert byte array to Rust Vec<u8>
            let bytes = env.convert_byte_array(screenshot_result.into_inner())?;
            
            // Get width and height
            let (width, height) = self.get_display_metrics()?;
            
            // Encode as base64
            let base64_data = BASE64.encode(&bytes);
            
            Ok(Screenshot {
                data: base64_data,
                width: width as u32,
                height: height as u32,
                format: options.format.unwrap_or_else(|| "png".to_string()),
            })
        }
        
        #[cfg(not(target_os = "android"))]
        {
            Err(HardwareError::UnsupportedPlatform("Android screen recording is not available on this platform".to_string()))
        }
    }
    
    async fn save_screenshot(&self, screenshot_data: &str, file_path: &str) -> Result<SaveResult> {
        let path = Path::new(file_path);
        
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Decode base64 data
        let bytes = match BASE64.decode(screenshot_data) {
            Ok(data) => data,
            Err(e) => return Err(HardwareError::InvalidData(format!("Failed to decode screenshot data: {}", e))),
        };
        
        // Write to file
        match fs::write(path, &bytes) {
            Ok(_) => Ok(SaveResult {
                path: file_path.to_string(),
                success: true,
            }),
            Err(e) => Err(HardwareError::FileOperationError(format!("Failed to save screenshot: {}", e))),
        }
    }
    
    async fn start_recording(&self, options: ScreenRecordingOptions) -> Result<ScreenRecorder> {
        #[cfg(target_os = "android")]
        {
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
            
            // Get width and height from options or use defaults
            let width = options.width.unwrap_or_else(|| {
                if let Ok((w, _)) = self.get_display_metrics() {
                    w as u32
                } else {
                    1280 // Default width
                }
            });
            
            let height = options.height.unwrap_or_else(|| {
                if let Ok((_, h)) = self.get_display_metrics() {
                    h as u32
                } else {
                    720 // Default height
                }
            });
            
            let frame_rate = options.frame_rate.unwrap_or(30);
            
            // Start recording
            let recorder_id_result = match env.call_method(
                activity,
                "startScreenRecording",
                "(III)Ljava/lang/String;",
                &[JValue::Int(width as i32), JValue::Int(height as i32), JValue::Int(frame_rate as i32)]
            ) {
                Ok(result) => result.l()?,
                Err(e) => return Err(HardwareError::DeviceAccessError(format!("Failed to start screen recording: {}", e))),
            };
            
            // Get recorder ID as string
            let recorder_id = match env.get_string(JString::from(recorder_id_result)) {
                Ok(id_str) => id_str.to_string_lossy().to_string(),
                Err(e) => return Err(HardwareError::DeviceAccessError(format!("Failed to get recorder ID: {}", e))),
            };
            
            // Create recording instance
            let instance = RecordingInstance {
                id: recorder_id.clone(),
                width,
                height,
                frame_rate,
                start_time: std::time::SystemTime::now(),
                paused: false,
                markers: Vec::new(),
            };
            
            // Store in our map
            self.recording_instances.lock().unwrap().insert(recorder_id.clone(), instance);
            
            Ok(ScreenRecorder {
                id: recorder_id,
                width,
                height,
                frame_rate,
            })
        }
        
        #[cfg(not(target_os = "android"))]
        {
            Err(HardwareError::UnsupportedPlatform("Android screen recording is not available on this platform".to_string()))
        }
    }
    
    async fn stop_recording(&self, recorder_id: &str, file_path: &str) -> Result<RecordingResult> {
        #[cfg(target_os = "android")]
        {
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
            
            // Create JString for recorder ID
            let j_recorder_id = env.new_string(recorder_id)?;
            
            // Create JString for file path
            let j_file_path = env.new_string(file_path)?;
            
            // Stop recording and save to file
            let success = match env.call_method(
                activity,
                "stopScreenRecording",
                "(Ljava/lang/String;Ljava/lang/String;)Z",
                &[JValue::Object(j_recorder_id.into()), JValue::Object(j_file_path.into())]
            ) {
                Ok(result) => result.z()?,
                Err(e) => return Err(HardwareError::DeviceAccessError(format!("Failed to stop screen recording: {}", e))),
            };
            
            if !success {
                return Err(HardwareError::DeviceAccessError("Failed to save recording to file".to_string()));
            }
            
            // Get recording instance
            let mut instances = self.recording_instances.lock().unwrap();
            let instance = match instances.remove(recorder_id) {
                Some(instance) => instance,
                None => return Err(HardwareError::InvalidParameter(format!("Recording with ID {} not found", recorder_id))),
            };
            
            // Calculate duration
            let duration = instance.start_time.elapsed().unwrap_or_default();
            
            Ok(RecordingResult {
                path: file_path.to_string(),
                duration: duration.as_secs_f64(),
                width: instance.width,
                height: instance.height,
                frame_rate: instance.frame_rate,
                format: "mp4".to_string(),
                markers: instance.markers,
                success: true,
            })
        }
        
        #[cfg(not(target_os = "android"))]
        {
            Err(HardwareError::UnsupportedPlatform("Android screen recording is not available on this platform".to_string()))
        }
    }
    
    async fn pause_recording(&self, recorder_id: &str) -> Result<()> {
        #[cfg(target_os = "android")]
        {
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
            
            // Create JString for recorder ID
            let j_recorder_id = env.new_string(recorder_id)?;
            
            // Pause recording
            let success = match env.call_method(
                activity,
                "pauseScreenRecording",
                "(Ljava/lang/String;)Z",
                &[JValue::Object(j_recorder_id.into())]
            ) {
                Ok(result) => result.z()?,
                Err(e) => return Err(HardwareError::DeviceAccessError(format!("Failed to pause screen recording: {}", e))),
            };
            
            if !success {
                return Err(HardwareError::DeviceAccessError("Failed to pause recording".to_string()));
            }
            
            // Update recording instance
            let mut instances = self.recording_instances.lock().unwrap();
            if let Some(instance) = instances.get_mut(recorder_id) {
                instance.paused = true;
            } else {
                return Err(HardwareError::InvalidParameter(format!("Recording with ID {} not found", recorder_id)));
            }
            
            Ok(())
        }
        
        #[cfg(not(target_os = "android"))]
        {
            Err(HardwareError::UnsupportedPlatform("Android screen recording is not available on this platform".to_string()))
        }
    }
    
    async fn resume_recording(&self, recorder_id: &str) -> Result<()> {
        #[cfg(target_os = "android")]
        {
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
            
            // Create JString for recorder ID
            let j_recorder_id = env.new_string(recorder_id)?;
            
            // Resume recording
            let success = match env.call_method(
                activity,
                "resumeScreenRecording",
                "(Ljava/lang/String;)Z",
                &[JValue::Object(j_recorder_id.into())]
            ) {
                Ok(result) => result.z()?,
                Err(e) => return Err(HardwareError::DeviceAccessError(format!("Failed to resume screen recording: {}", e))),
            };
            
            if !success {
                return Err(HardwareError::DeviceAccessError("Failed to resume recording".to_string()));
            }
            
            // Update recording instance
            let mut instances = self.recording_instances.lock().unwrap();
            if let Some(instance) = instances.get_mut(recorder_id) {
                instance.paused = false;
            } else {
                return Err(HardwareError::InvalidParameter(format!("Recording with ID {} not found", recorder_id)));
            }
            
            Ok(())
        }
        
        #[cfg(not(target_os = "android"))]
        {
            Err(HardwareError::UnsupportedPlatform("Android screen recording is not available on this platform".to_string()))
        }
    }
    
    async fn add_marker(&self, recorder_id: &str, label: &str) -> Result<()> {
        #[cfg(target_os = "android")]
        {
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
            
            // Create JString for recorder ID
            let j_recorder_id = env.new_string(recorder_id)?;
            
            // Create JString for label
            let j_label = env.new_string(label)?;
            
            // Add marker
            let success = match env.call_method(
                activity,
                "addRecordingMarker",
                "(Ljava/lang/String;Ljava/lang/String;)Z",
                &[JValue::Object(j_recorder_id.into()), JValue::Object(j_label.into())]
            ) {
                Ok(result) => result.z()?,
                Err(e) => return Err(HardwareError::DeviceAccessError(format!("Failed to add recording marker: {}", e))),
            };
            
            if !success {
                return Err(HardwareError::DeviceAccessError("Failed to add marker".to_string()));
            }
            
            // Update recording instance
            let mut instances = self.recording_instances.lock().unwrap();
            if let Some(instance) = instances.get_mut(recorder_id) {
                let elapsed = instance.start_time.elapsed().unwrap_or_default();
                instance.markers.push(crate::screen::RecordingMarker {
                    time: elapsed.as_secs_f64(),
                    label: label.to_string(),
                });
            } else {
                return Err(HardwareError::InvalidParameter(format!("Recording with ID {} not found", recorder_id)));
            }
            
            Ok(())
        }
        
        #[cfg(not(target_os = "android"))]
        {
            Err(HardwareError::UnsupportedPlatform("Android screen recording is not available on this platform".to_string()))
        }
    }
}
