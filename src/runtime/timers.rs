use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use crate::interpreter::{Value, Function, Environment};

/// Represents a timer in the SmashLang runtime
#[derive(Debug)]
pub struct Timer {
    /// Timer ID
    pub id: usize,
    
    /// Callback function
    pub callback: Function,
    
    /// Arguments to pass to the callback
    pub args: Vec<Value>,
    
    /// Delay in milliseconds
    pub delay: u64,
    
    /// Interval flag (true for setInterval, false for setTimeout)
    pub is_interval: bool,
    
    /// Creation time
    pub created_at: Instant,
    
    /// Next execution time
    pub next_execution: Instant,
    
    /// Cancelled flag
    pub cancelled: bool,
}

impl Timer {
    /// Create a new timer
    pub fn new(id: usize, callback: Function, args: Vec<Value>, delay: u64, is_interval: bool) -> Self {
        let now = Instant::now();
        Self {
            id,
            callback,
            args,
            delay,
            is_interval,
            created_at: now,
            next_execution: now + Duration::from_millis(delay),
            cancelled: false,
        }
    }
    
    /// Check if the timer is ready to execute
    pub fn is_ready(&self, now: Instant) -> bool {
        !self.cancelled && now >= self.next_execution
    }
    
    /// Update the next execution time
    pub fn update_next_execution(&mut self) {
        self.next_execution = Instant::now() + Duration::from_millis(self.delay);
    }
}

/// Represents a timer manager in the SmashLang runtime
#[derive(Debug, Clone)]
pub struct TimerManager {
    /// Timers
    timers: Arc<Mutex<HashMap<usize, Timer>>>,
    
    /// Next timer ID
    next_id: Arc<Mutex<usize>>,
    
    /// Environment for executing callbacks
    environment: Environment,
}

impl TimerManager {
    /// Create a new timer manager
    pub fn new(environment: Environment) -> Self {
        let manager = Self {
            timers: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
            environment,
        };
        
        // Start the timer thread
        manager.start_timer_thread();
        
        manager
    }
    
    /// Create a timeout
    pub fn set_timeout(&self, callback: Function, args: Vec<Value>, delay: u64) -> usize {
        let id = {
            let mut next_id = self.next_id.lock().unwrap();
            let id = *next_id;
            *next_id += 1;
            id
        };
        
        let timer = Timer::new(id, callback, args, delay, false);
        
        {
            let mut timers = self.timers.lock().unwrap();
            timers.insert(id, timer);
        }
        
        id
    }
    
    /// Create an interval
    pub fn set_interval(&self, callback: Function, args: Vec<Value>, delay: u64) -> usize {
        let id = {
            let mut next_id = self.next_id.lock().unwrap();
            let id = *next_id;
            *next_id += 1;
            id
        };
        
        let timer = Timer::new(id, callback, args, delay, true);
        
        {
            let mut timers = self.timers.lock().unwrap();
            timers.insert(id, timer);
        }
        
        id
    }
    
    /// Clear a timeout or interval
    pub fn clear_timer(&self, id: usize) -> bool {
        let mut timers = self.timers.lock().unwrap();
        
        if let Some(timer) = timers.get_mut(&id) {
            timer.cancelled = true;
            true
        } else {
            false
        }
    }
    
    /// Start the timer thread
    fn start_timer_thread(&self) {
        let timers = self.timers.clone();
        let environment = self.environment.clone();
        
        thread::spawn(move || {
            loop {
                // Sleep for a short time to avoid busy waiting
                thread::sleep(Duration::from_millis(10));
                
                let now = Instant::now();
                let mut ready_timers = Vec::new();
                
                // Find ready timers
                {
                    let mut timers_lock = timers.lock().unwrap();
                    let mut to_remove = Vec::new();
                    
                    for (id, timer) in timers_lock.iter_mut() {
                        if timer.is_ready(now) {
                            // Clone the timer for execution
                            let callback = timer.callback.clone();
                            let args = timer.args.clone();
                            
                            // Add to ready timers
                            ready_timers.push((callback, args));
                            
                            // Update or remove the timer
                            if timer.is_interval {
                                timer.update_next_execution();
                            } else {
                                to_remove.push(*id);
                            }
                        }
                    }
                    
                    // Remove completed timeouts
                    for id in to_remove {
                        timers_lock.remove(&id);
                    }
                }
                
                // Execute ready timers
                for (callback, args) in ready_timers {
                    // Execute the callback
                    let _ = callback.call(Value::Undefined, &args, &environment);
                }
            }
        });
    }
}

/// Create a setTimeout function
pub fn create_set_timeout_function(timer_manager: TimerManager) -> Function {
    Function::new_native(
        Some("setTimeout".to_string()),
        vec!["callback".to_string(), "delay".to_string(), "...args".to_string()],
        move |_this, args, _env| {
            if args.is_empty() {
                return Err("setTimeout requires at least one argument".to_string());
            }
            
            let callback = match &args[0] {
                Value::Function(func) => func.clone(),
                _ => return Err("setTimeout callback must be a function".to_string()),
            };
            
            let delay = if args.len() > 1 {
                match &args[1] {
                    Value::Number(n) => *n as u64,
                    _ => 0,
                }
            } else {
                0
            };
            
            let callback_args = if args.len() > 2 {
                args[2..].to_vec()
            } else {
                Vec::new()
            };
            
            let id = timer_manager.set_timeout(callback, callback_args, delay);
            Ok(Value::Number(id as f64))
        },
    )
}

/// Create a setInterval function
pub fn create_set_interval_function(timer_manager: TimerManager) -> Function {
    Function::new_native(
        Some("setInterval".to_string()),
        vec!["callback".to_string(), "delay".to_string(), "...args".to_string()],
        move |_this, args, _env| {
            if args.is_empty() {
                return Err("setInterval requires at least one argument".to_string());
            }
            
            let callback = match &args[0] {
                Value::Function(func) => func.clone(),
                _ => return Err("setInterval callback must be a function".to_string()),
            };
            
            let delay = if args.len() > 1 {
                match &args[1] {
                    Value::Number(n) => *n as u64,
                    _ => 0,
                }
            } else {
                0
            };
            
            let callback_args = if args.len() > 2 {
                args[2..].to_vec()
            } else {
                Vec::new()
            };
            
            let id = timer_manager.set_interval(callback, callback_args, delay);
            Ok(Value::Number(id as f64))
        },
    )
}

/// Create a clearTimeout function
pub fn create_clear_timeout_function(timer_manager: TimerManager) -> Function {
    Function::new_native(
        Some("clearTimeout".to_string()),
        vec!["id".to_string()],
        move |_this, args, _env| {
            if args.is_empty() {
                return Ok(Value::Undefined);
            }
            
            let id = match &args[0] {
                Value::Number(n) => *n as usize,
                _ => return Ok(Value::Undefined),
            };
            
            let result = timer_manager.clear_timer(id);
            Ok(Value::Boolean(result))
        },
    )
}

/// Create a clearInterval function
pub fn create_clear_interval_function(timer_manager: TimerManager) -> Function {
    Function::new_native(
        Some("clearInterval".to_string()),
        vec!["id".to_string()],
        move |_this, args, _env| {
            if args.is_empty() {
                return Ok(Value::Undefined);
            }
            
            let id = match &args[0] {
                Value::Number(n) => *n as usize,
                _ => return Ok(Value::Undefined),
            };
            
            let result = timer_manager.clear_timer(id);
            Ok(Value::Boolean(result))
        },
    )
}