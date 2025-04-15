use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::interpreter::{Value, Function, Environment};

/// Represents a class in the SmashLang runtime
#[derive(Debug, Clone)]
pub struct Class {
    /// Name of the class
    pub name: String,
    
    /// Parent class (if any)
    pub parent: Option<Rc<Class>>,
    
    /// Constructor function
    pub constructor: Option<Function>,
    
    /// Instance methods
    pub instance_methods: HashMap<String, Function>,
    
    /// Static methods
    pub static_methods: HashMap<String, Function>,
    
    /// Instance properties (default values)
    pub instance_properties: HashMap<String, Value>,
    
    /// Static properties
    pub static_properties: HashMap<String, Value>,
    
    /// Private methods
    pub private_methods: HashMap<String, Function>,
    
    /// Private properties
    pub private_properties: HashMap<String, Value>,
}

impl Class {
    /// Create a new class
    pub fn new(name: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            name,
            parent: None,
            constructor: None,
            instance_methods: HashMap::new(),
            static_methods: HashMap::new(),
            instance_properties: HashMap::new(),
            static_properties: HashMap::new(),
            private_methods: HashMap::new(),
            private_properties: HashMap::new(),
        }))
    }
    
    /// Set the parent class
    pub fn set_parent(&mut self, parent: Rc<Class>) {
        self.parent = Some(parent);
    }
    
    /// Set the constructor function
    pub fn set_constructor(&mut self, constructor: Function) {
        self.constructor = Some(constructor);
    }
    
    /// Add an instance method
    pub fn add_instance_method(&mut self, name: String, method: Function) {
        self.instance_methods.insert(name, method);
    }
    
    /// Add a static method
    pub fn add_static_method(&mut self, name: String, method: Function) {
        self.static_methods.insert(name, method);
    }
    
    /// Add an instance property
    pub fn add_instance_property(&mut self, name: String, value: Value) {
        self.instance_properties.insert(name, value);
    }
    
    /// Add a static property
    pub fn add_static_property(&mut self, name: String, value: Value) {
        self.static_properties.insert(name, value);
    }
    
    /// Add a private method
    pub fn add_private_method(&mut self, name: String, method: Function) {
        self.private_methods.insert(name, method);
    }
    
    /// Add a private property
    pub fn add_private_property(&mut self, name: String, value: Value) {
        self.private_properties.insert(name, value);
    }
    
    /// Create a new instance of the class
    pub fn create_instance(&self, args: &[Value], env: &Environment) -> Result<Value, String> {
        // Create a new object
        let mut instance = HashMap::new();
        
        // Initialize instance properties
        for (name, value) in &self.instance_properties {
            instance.insert(name.clone(), value.clone());
        }
        
        // Create the instance value
        let instance_value = Value::Object(instance);
        
        // Call the constructor if it exists
        if let Some(constructor) = &self.constructor {
            constructor.call(instance_value.clone(), args, env)?;
        }
        
        // Return the instance
        Ok(instance_value)
    }
    
    /// Get an instance method
    pub fn get_instance_method(&self, name: &str) -> Option<Function> {
        if let Some(method) = self.instance_methods.get(name) {
            return Some(method.clone());
        }
        
        // Check parent class
        if let Some(parent) = &self.parent {
            return parent.get_instance_method(name);
        }
        
        None
    }
    
    /// Get a static method
    pub fn get_static_method(&self, name: &str) -> Option<Function> {
        if let Some(method) = self.static_methods.get(name) {
            return Some(method.clone());
        }
        
        // Check parent class
        if let Some(parent) = &self.parent {
            return parent.get_static_method(name);
        }
        
        None
    }
    
    /// Get a private method
    pub fn get_private_method(&self, name: &str) -> Option<Function> {
        if let Some(method) = self.private_methods.get(name) {
            return Some(method.clone());
        }
        
        // Private methods are not inherited
        None
    }
    
    /// Check if a method is accessible from a given class
    pub fn can_access_private_member(&self, from_class: &Class, name: &str) -> bool {
        // Private members can only be accessed from the same class
        self.name == from_class.name && (self.private_methods.contains_key(name) || self.private_properties.contains_key(name))
    }
    
    /// Create a class object (for static methods and properties)
    pub fn create_class_object(&self) -> Value {
        let mut class_obj = HashMap::new();
        
        // Add static properties
        for (name, value) in &self.static_properties {
            class_obj.insert(name.clone(), value.clone());
        }
        
        // Add constructor
        if let Some(constructor) = &self.constructor {
            class_obj.insert("constructor".to_string(), Value::Function(constructor.clone()));
        }
        
        // Add static methods
        for (name, method) in &self.static_methods {
            class_obj.insert(name.clone(), Value::Function(method.clone()));
        }
        
        Value::Object(class_obj)
    }
}

/// ClassInstance represents an instance of a class
#[derive(Debug, Clone)]
pub struct ClassInstance {
    /// The class this is an instance of
    pub class: Rc<Class>,
    
    /// Instance properties
    pub properties: HashMap<String, Value>,
    
    /// Private properties
    pub private_properties: HashMap<String, Value>,
}

impl ClassInstance {
    /// Create a new class instance
    pub fn new(class: Rc<Class>) -> Self {
        let mut properties = HashMap::new();
        
        // Initialize instance properties
        for (name, value) in &class.instance_properties {
            properties.insert(name.clone(), value.clone());
        }
        
        Self {
            class,
            properties,
            private_properties: HashMap::new(),
        }
    }
    
    /// Get a property
    pub fn get_property(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.properties.get(name) {
            return Some(value.clone());
        }
        
        None
    }
    
    /// Set a property
    pub fn set_property(&mut self, name: &str, value: Value) {
        self.properties.insert(name.to_string(), value);
    }
    
    /// Get a private property
    pub fn get_private_property(&self, name: &str, from_class: &Class) -> Option<Value> {
        // Check if the class can access the private property
        if self.class.can_access_private_member(from_class, name) {
            if let Some(value) = self.private_properties.get(name) {
                return Some(value.clone());
            }
        }
        
        None
    }
    
    /// Set a private property
    pub fn set_private_property(&mut self, name: &str, value: Value, from_class: &Class) -> Result<(), String> {
        // Check if the class can access the private property
        if self.class.can_access_private_member(from_class, name) {
            self.private_properties.insert(name.to_string(), value);
            Ok(())
        } else {
            Err(format!("Cannot access private property '{}' from class '{}'", name, from_class.name))
        }
    }
    
    /// Call a method
    pub fn call_method(&self, name: &str, args: &[Value], env: &Environment) -> Result<Value, String> {
        if let Some(method) = self.class.get_instance_method(name) {
            method.call(Value::Object(self.properties.clone()), args, env)
        } else {
            Err(format!("Method '{}' not found", name))
        }
    }
    
    /// Call a private method
    pub fn call_private_method(&self, name: &str, args: &[Value], env: &Environment, from_class: &Class) -> Result<Value, String> {
        // Check if the class can access the private method
        if self.class.can_access_private_member(from_class, name) {
            if let Some(method) = self.class.get_private_method(name) {
                method.call(Value::Object(self.properties.clone()), args, env)
            } else {
                Err(format!("Private method '{}' not found", name))
            }
        } else {
            Err(format!("Cannot access private method '{}' from class '{}'", name, from_class.name))
        }
    }
}

/// Create a class constructor function
pub fn create_class_constructor(class: Rc<RefCell<Class>>) -> Function {
    Function::new_native(
        Some(class.borrow().name.clone()),
        vec!["...args".to_string()],
        move |_this, args, env| {
            // Create a new instance of the class
            class.borrow().create_instance(args, env)
        },
    )
}