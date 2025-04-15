use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::cell::RefCell;
use std::fs;
use crate::interpreter::{Value, Environment};
use crate::parser::{AstNode, SmashParser};

/// Represents the status of a module
#[derive(Debug, Clone, PartialEq)]
pub enum ModuleStatus {
    Unlinked,   // Initial state
    Linking,    // Resolving dependencies
    Linked,     // Dependencies resolved
    Evaluating, // Running module code
    Evaluated,  // Module fully loaded
    Failed,     // Error occurred
}

/// Represents a module in the SmashLang runtime
#[derive(Debug, Clone)]
pub struct Module {
    /// Absolute path to the module
    pub id: String,
    
    /// URL representation of the path
    pub url: String,
    
    /// Module namespace object
    pub exports: RefCell<Value>,
    
    /// Module status
    pub status: RefCell<ModuleStatus>,
    
    /// Module dependencies
    pub dependencies: RefCell<Vec<String>>,
    
    /// Module source code
    pub source: String,
    
    /// Module AST
    pub ast: Option<AstNode>,
    
    /// Module environment
    pub environment: RefCell<Environment>,
}

impl Module {
    /// Create a new module
    pub fn new(id: String, source: String) -> Rc<Self> {
        let url = format!("file://{}", id);
        
        Rc::new(Self {
            id,
            url,
            exports: RefCell::new(Value::Object(HashMap::new())),
            status: RefCell::new(ModuleStatus::Unlinked),
            dependencies: RefCell::new(Vec::new()),
            source,
            ast: None,
            environment: RefCell::new(Environment::new()),
        })
    }
    
    /// Parse the module source code
    pub fn parse(&mut self) -> Result<(), String> {
        match SmashParser::parse(&self.source) {
            Ok(ast) => {
                self.ast = Some(ast);
                Ok(())
            },
            Err(err) => {
                *self.status.borrow_mut() = ModuleStatus::Failed;
                Err(format!("Failed to parse module: {}", err))
            }
        }
    }
    
    /// Link the module (resolve dependencies)
    pub fn link(&self, module_registry: &ModuleRegistry) -> Result<(), String> {
        if *self.status.borrow() != ModuleStatus::Unlinked {
            return Ok(());
        }
        
        *self.status.borrow_mut() = ModuleStatus::Linking;
        
        // Extract import statements from AST
        if let Some(AstNode::Program(statements)) = &self.ast {
            for statement in statements {
                if let AstNode::Import { source, .. } = statement {
                    let dependency_path = self.resolve_module_specifier(source, module_registry)?;
                    self.dependencies.borrow_mut().push(dependency_path);
                }
            }
        }
        
        // Link dependencies
        for dependency_path in self.dependencies.borrow().clone() {
            if let Some(dependency) = module_registry.get_module(&dependency_path) {
                dependency.link(module_registry)?;
            } else {
                *self.status.borrow_mut() = ModuleStatus::Failed;
                return Err(format!("Failed to resolve dependency: {}", dependency_path));
            }
        }
        
        *self.status.borrow_mut() = ModuleStatus::Linked;
        Ok(())
    }
    
    /// Evaluate the module
    pub fn evaluate(&self, module_registry: &ModuleRegistry) -> Result<Value, String> {
        if *self.status.borrow() == ModuleStatus::Evaluated {
            return Ok(self.exports.borrow().clone());
        }
        
        if *self.status.borrow() == ModuleStatus::Evaluating {
            return Ok(self.exports.borrow().clone());
        }
        
        *self.status.borrow_mut() = ModuleStatus::Evaluating;
        
        // Evaluate dependencies first
        for dependency_path in self.dependencies.borrow().clone() {
            if let Some(dependency) = module_registry.get_module(&dependency_path) {
                dependency.evaluate(module_registry)?;
            } else {
                *self.status.borrow_mut() = ModuleStatus::Failed;
                return Err(format!("Failed to evaluate dependency: {}", dependency_path));
            }
        }
        
        // Create module environment
        let mut env = Environment::new();
        
        // Add import bindings to environment
        if let Some(AstNode::Program(statements)) = &self.ast {
            for statement in statements {
                if let AstNode::Import { source, default_import, named_imports, namespace_import, .. } = statement {
                    let dependency_path = self.resolve_module_specifier(source, module_registry)?;
                    
                    if let Some(dependency) = module_registry.get_module(&dependency_path) {
                        let dependency_exports = dependency.exports.borrow().clone();
                        
                        // Handle default import
                        if let Some(default_name) = default_import {
                            if let Value::Object(exports) = &dependency_exports {
                                if let Some(default_export) = exports.get("default") {
                                    env.define(default_name, default_export.clone());
                                } else {
                                    return Err(format!("Module {} has no default export", dependency_path));
                                }
                            }
                        }
                        
                        // Handle named imports
                        for import_specifier in named_imports {
                            if let Value::Object(exports) = &dependency_exports {
                                let import_name = &import_specifier.name;
                                let local_name = import_specifier.alias.as_ref().unwrap_or(import_name);
                                
                                if let Some(export_value) = exports.get(import_name) {
                                    env.define(local_name, export_value.clone());
                                } else {
                                    return Err(format!("Module {} has no export named {}", dependency_path, import_name));
                                }
                            }
                        }
                        
                        // Handle namespace import
                        if let Some(namespace_name) = namespace_import {
                            env.define(namespace_name, dependency_exports);
                        }
                    }
                }
            }
        }
        
        *self.environment.borrow_mut() = env;
        
        // Evaluate module code
        if let Some(AstNode::Program(statements)) = &self.ast {
            for statement in statements {
                match statement {
                    AstNode::Export { declaration } => {
                        // Handle export declaration
                        match &**declaration {
                            AstNode::LetDecl { name, value } => {
                                let value_result = self.evaluate_node(value, &self.environment.borrow())?;
                                self.environment.borrow_mut().define(name, value_result.clone());
                                
                                if let Value::Object(ref mut exports) = *self.exports.borrow_mut() {
                                    exports.insert(name.clone(), value_result);
                                }
                            },
                            AstNode::ConstDecl { name, value } => {
                                let value_result = self.evaluate_node(value, &self.environment.borrow())?;
                                self.environment.borrow_mut().define(name, value_result.clone());
                                
                                if let Value::Object(ref mut exports) = *self.exports.borrow_mut() {
                                    exports.insert(name.clone(), value_result);
                                }
                            },
                            AstNode::Function { name, .. } => {
                                let func_result = self.evaluate_node(declaration, &self.environment.borrow())?;
                                self.environment.borrow_mut().define(name, func_result.clone());
                                
                                if let Value::Object(ref mut exports) = *self.exports.borrow_mut() {
                                    exports.insert(name.clone(), func_result);
                                }
                            },
                            AstNode::ClassDeclaration { name, .. } => {
                                let class_result = self.evaluate_node(declaration, &self.environment.borrow())?;
                                self.environment.borrow_mut().define(name, class_result.clone());
                                
                                if let Value::Object(ref mut exports) = *self.exports.borrow_mut() {
                                    exports.insert(name.clone(), class_result);
                                }
                            },
                            _ => {
                                return Err(format!("Unsupported export declaration: {:?}", declaration));
                            }
                        }
                    },
                    AstNode::ExportDefault { expression } => {
                        // Handle default export
                        let value_result = self.evaluate_node(expression, &self.environment.borrow())?;
                        
                        if let Value::Object(ref mut exports) = *self.exports.borrow_mut() {
                            exports.insert("default".to_string(), value_result);
                        }
                    },
                    AstNode::ExportNamed { specifiers, source } => {
                        // Handle named exports
                        if let Some(source_path) = source {
                            // Re-export from another module
                            let dependency_path = self.resolve_module_specifier(source_path, module_registry)?;
                            
                            if let Some(dependency) = module_registry.get_module(&dependency_path) {
                                let dependency_exports = dependency.exports.borrow().clone();
                                
                                if let Value::Object(dep_exports) = dependency_exports {
                                    if let Value::Object(ref mut exports) = *self.exports.borrow_mut() {
                                        for specifier in specifiers {
                                            let export_name = &specifier.name;
                                            let local_name = specifier.exported_name.as_ref().unwrap_or(export_name);
                                            
                                            if let Some(export_value) = dep_exports.get(export_name) {
                                                exports.insert(local_name.clone(), export_value.clone());
                                            } else {
                                                return Err(format!("Module {} has no export named {}", dependency_path, export_name));
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            // Export local bindings
                            if let Value::Object(ref mut exports) = *self.exports.borrow_mut() {
                                for specifier in specifiers {
                                    let local_name = &specifier.name;
                                    let export_name = specifier.exported_name.as_ref().unwrap_or(local_name);
                                    
                                    if let Some(value) = self.environment.borrow().get(local_name) {
                                        exports.insert(export_name.clone(), value);
                                    } else {
                                        return Err(format!("Cannot export undefined variable: {}", local_name));
                                    }
                                }
                            }
                        }
                    },
                    AstNode::ExportAll { source, exported_name } => {
                        // Handle export * from "module"
                        let dependency_path = self.resolve_module_specifier(source, module_registry)?;
                        
                        if let Some(dependency) = module_registry.get_module(&dependency_path) {
                            let dependency_exports = dependency.exports.borrow().clone();
                            
                            if let Value::Object(dep_exports) = dependency_exports {
                                if let Some(namespace) = exported_name {
                                    // export * as namespace from "module"
                                    if let Value::Object(ref mut exports) = *self.exports.borrow_mut() {
                                        exports.insert(namespace.clone(), Value::Object(dep_exports));
                                    }
                                } else {
                                    // export * from "module"
                                    if let Value::Object(ref mut exports) = *self.exports.borrow_mut() {
                                        for (key, value) in dep_exports {
                                            if key != "default" {
                                                exports.insert(key, value);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    AstNode::Import { .. } => {
                        // Imports are handled during linking
                    },
                    _ => {
                        // Evaluate other statements
                        self.evaluate_node(statement, &self.environment.borrow())?;
                    }
                }
            }
        }
        
        *self.status.borrow_mut() = ModuleStatus::Evaluated;
        Ok(self.exports.borrow().clone())
    }
    
    /// Evaluate a single AST node
    fn evaluate_node(&self, node: &AstNode, env: &Environment) -> Result<Value, String> {
        // This is a simplified version for demonstration
        // In a real implementation, this would be a complete interpreter
        match node {
            AstNode::Number(n) => Ok(Value::Number(*n as f64)),
            AstNode::Float(f) => Ok(Value::Number(*f)),
            AstNode::String(s) => Ok(Value::String(s.clone())),
            AstNode::Boolean(b) => Ok(Value::Boolean(*b)),
            AstNode::Null => Ok(Value::Null),
            AstNode::Undefined => Ok(Value::Undefined),
            AstNode::Identifier(name) => {
                if let Some(value) = env.get(name) {
                    Ok(value)
                } else {
                    Err(format!("Undefined variable: {}", name))
                }
            },
            // Add more cases for other AST nodes
            _ => Ok(Value::Undefined),
        }
    }
    
    /// Resolve a module specifier to an absolute path
    fn resolve_module_specifier(&self, specifier: &str, registry: &ModuleRegistry) -> Result<String, String> {
        if specifier.starts_with("./") || specifier.starts_with("../") {
            // Relative path
            let base_dir = Path::new(&self.id).parent().unwrap_or_else(|| Path::new(""));
            let resolved_path = base_dir.join(specifier);
            
            // Normalize path
            let canonical_path = match fs::canonicalize(&resolved_path) {
                Ok(path) => path,
                Err(_) => {
                    // Try adding .smash extension
                    let with_ext = resolved_path.with_extension("smash");
                    match fs::canonicalize(&with_ext) {
                        Ok(path) => path,
                        Err(_) => {
                            // Try as directory with index.smash
                            let index_path = resolved_path.join("index.smash");
                            match fs::canonicalize(&index_path) {
                                Ok(path) => path,
                                Err(_) => {
                                    return Err(format!("Cannot resolve module: {}", specifier));
                                }
                            }
                        }
                    }
                }
            };
            
            Ok(canonical_path.to_string_lossy().to_string())
        } else if specifier.starts_with("/") {
            // Absolute path
            let resolved_path = Path::new(specifier);
            
            // Normalize path
            let canonical_path = match fs::canonicalize(&resolved_path) {
                Ok(path) => path,
                Err(_) => {
                    // Try adding .smash extension
                    let with_ext = resolved_path.with_extension("smash");
                    match fs::canonicalize(&with_ext) {
                        Ok(path) => path,
                        Err(_) => {
                            // Try as directory with index.smash
                            let index_path = resolved_path.join("index.smash");
                            match fs::canonicalize(&index_path) {
                                Ok(path) => path,
                                Err(_) => {
                                    return Err(format!("Cannot resolve module: {}", specifier));
                                }
                            }
                        }
                    }
                }
            };
            
            Ok(canonical_path.to_string_lossy().to_string())
        } else {
            // Bare specifier (e.g., "lodash")
            registry.resolve_bare_specifier(specifier)
        }
    }
}

/// Module registry for managing modules
#[derive(Debug)]
pub struct ModuleRegistry {
    /// Loaded modules
    modules: RefCell<HashMap<String, Rc<Module>>>,
    
    /// Module resolution paths
    paths: Vec<PathBuf>,
}

impl ModuleRegistry {
    /// Create a new module registry
    pub fn new() -> Self {
        Self {
            modules: RefCell::new(HashMap::new()),
            paths: vec![PathBuf::from("node_modules"), PathBuf::from("smashlang_packages")],
        }
    }
    
    /// Add a module resolution path
    pub fn add_path(&mut self, path: PathBuf) {
        self.paths.push(path);
    }
    
    /// Get a module by path
    pub fn get_module(&self, path: &str) -> Option<Rc<Module>> {
        self.modules.borrow().get(path).cloned()
    }
    
    /// Load a module from a file
    pub fn load_module(&self, path: &str) -> Result<Rc<Module>, String> {
        if let Some(module) = self.get_module(path) {
            return Ok(module);
        }
        
        // Read the file
        let source = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(err) => return Err(format!("Failed to read module file: {}", err)),
        };
        
        // Create the module
        let mut module = Module::new(path.to_string(), source);
        
        // Parse the module
        if let Err(err) = Rc::get_mut(&mut module).unwrap().parse() {
            return Err(err);
        }
        
        // Add the module to the registry
        self.modules.borrow_mut().insert(path.to_string(), module.clone());
        
        Ok(module)
    }
    
    /// Resolve a bare specifier to an absolute path
    pub fn resolve_bare_specifier(&self, specifier: &str) -> Result<String, String> {
        for path in &self.paths {
            let module_path = path.join(specifier);
            
            // Try as direct file
            if module_path.exists() && module_path.is_file() {
                return Ok(module_path.to_string_lossy().to_string());
            }
            
            // Try with .smash extension
            let with_ext = module_path.with_extension("smash");
            if with_ext.exists() && with_ext.is_file() {
                return Ok(with_ext.to_string_lossy().to_string());
            }
            
            // Try as directory with index.smash
            let index_path = module_path.join("index.smash");
            if index_path.exists() && index_path.is_file() {
                return Ok(index_path.to_string_lossy().to_string());
            }
            
            // Try as package with package.json
            let package_json = module_path.join("package.json");
            if package_json.exists() && package_json.is_file() {
                // Read package.json
                if let Ok(content) = fs::read_to_string(&package_json) {
                    // Parse package.json
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                        // Get main field
                        if let Some(main) = json.get("main").and_then(|v| v.as_str()) {
                            let main_path = module_path.join(main);
                            if main_path.exists() && main_path.is_file() {
                                return Ok(main_path.to_string_lossy().to_string());
                            }
                        }
                    }
                }
                
                // Fallback to index.smash
                let index_path = module_path.join("index.smash");
                if index_path.exists() && index_path.is_file() {
                    return Ok(index_path.to_string_lossy().to_string());
                }
            }
        }
        
        Err(format!("Cannot resolve module: {}", specifier))
    }
}