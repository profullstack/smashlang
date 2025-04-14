# ES Modules Implementation for SmashLang

This document outlines the implementation plan for enhancing the ES Modules system in SmashLang.

## Overview

ES Modules provide a standard way to organize and share code between files and packages. They are a core feature of modern JavaScript development and essential for building modular applications. While SmashLang has basic import statement support, a complete implementation is needed to support the packages we've developed.

## Current Status

Based on code examination:
- Basic import statement exists in the grammar (`import_statement = { "import" ~ string_literal ~ ";" }`)
- AST has an `Import` node
- No support for named imports, default imports, or exports
- No module resolution system

## Implementation Goals

1. Implement complete import syntax (named, default, namespace)
2. Implement export syntax (named, default, re-exports)
3. Implement module resolution algorithm
4. Implement module loading and caching
5. Add comprehensive tests

## Detailed Implementation Plan

### 1. Enhanced Import Syntax

```javascript
// Target syntax
import defaultExport from 'module-name';
import { export1, export2 as alias2 } from 'module-name';
import * as namespace from 'module-name';
import defaultExport, { export1, export2 as alias2 } from 'module-name';
import 'module-name'; // Side effects only
```

#### Implementation Steps:

1. **Grammar Updates**
   - Replace the simple import statement with a more comprehensive rule
   - Add support for different import styles

```pest
import_statement = {
    "import" ~ (
        (default_import ~ ("," ~ named_imports)?) |
        named_imports |
        namespace_import |
        string_literal // Side effects only
    ) ~ "from" ~ string_literal ~ ";"
}

default_import = { identifier }
named_imports = { "{" ~ import_specifier ~ ("," ~ import_specifier)* ~ "}" }
import_specifier = { identifier ~ ("as" ~ identifier)? }
namespace_import = { "*" ~ "as" ~ identifier }
```

2. **AST Updates**
   - Replace the simple `Import` node with more detailed nodes

```rust
ImportDeclaration {
    source: String,
    default_import: Option<String>,
    named_imports: Vec<ImportSpecifier>,
    namespace_import: Option<String>,
    side_effect_only: bool,
}

struct ImportSpecifier {
    name: String,
    alias: Option<String>,
}
```

3. **Runtime Implementation**
   - Implement module loading based on the import source
   - Bind imported values to the current scope
   - Handle different import styles (default, named, namespace)

### 2. Export Syntax

```javascript
// Target syntax
// Named exports
export const STATES = { /* ... */ };
export function createClient() { /* ... */ }

// Default export
export default websocket;

// Export declarations
export class WebSocketClient { /* ... */ }

// Re-exports
export { foo, bar } from 'other-module';
export * from 'another-module';
export { default } from 'module';
export { foo as bar } from 'module';
```

#### Implementation Steps:

1. **Grammar Updates**
   - Add various export statement rules to the grammar

```pest
export_statement = {
    export_declaration |
    export_default_declaration |
    export_named_declaration |
    export_from_declaration
}

export_declaration = { "export" ~ (variable_declaration | function_declaration | class_declaration) }
export_default_declaration = { "export" ~ "default" ~ expression ~ ";" }
export_named_declaration = { "export" ~ "{" ~ export_specifier ~ ("," ~ export_specifier)* ~ "}" ~ ";" }
export_from_declaration = { "export" ~ (
    "{" ~ export_specifier ~ ("," ~ export_specifier)* ~ "}" |
    "*" ~ ("as" ~ identifier)?
) ~ "from" ~ string_literal ~ ";" }

export_specifier = { identifier ~ ("as" ~ identifier)? }
```

2. **AST Updates**
   - Add various export nodes to the AST

```rust
ExportDeclaration {
    declaration: Box<AstNode>, // Variable, function, or class declaration
}

ExportDefaultDeclaration {
    expression: Box<AstNode>,
}

ExportNamedDeclaration {
    specifiers: Vec<ExportSpecifier>,
    source: Option<String>,
}

ExportAllDeclaration {
    source: String,
    exported_name: Option<String>, // For "export * as name from 'module'"
}

struct ExportSpecifier {
    name: String,
    exported_name: Option<String>,
}
```

3. **Runtime Implementation**
   - Implement export collection during module evaluation
   - Create module namespace object with exported values
   - Handle re-exports by loading the source module

### 3. Module Resolution Algorithm

The module resolution algorithm determines how import specifiers are resolved to actual module files.

#### Implementation Steps:

1. **Define Resolution Rules**
   - Bare specifiers (e.g., 'lodash'): Resolve to node_modules or SmashLang packages
   - Relative specifiers (e.g., './utils'): Resolve relative to the importing module
   - Absolute specifiers (e.g., '/lib/utils'): Resolve from the project root
   - Handle file extensions (.smash, .js, etc.)
   - Handle directory imports (index.smash)

2. **Implement Resolution Algorithm**
   - Parse the import specifier
   - Apply resolution rules based on specifier type
   - Handle errors for unresolved modules

```rust
fn resolve_module(specifier: &str, importing_module_path: &str) -> Result<String, Error> {
    if specifier.starts_with('./') || specifier.starts_with('../') {
        // Relative path resolution
        resolve_relative_specifier(specifier, importing_module_path)
    } else if specifier.starts_with('/') {
        // Absolute path resolution
        resolve_absolute_specifier(specifier)
    } else {
        // Bare specifier resolution
        resolve_bare_specifier(specifier)
    }
}
```

### 4. Module Loading and Caching

Efficient module loading requires parsing, compiling, and caching modules to avoid redundant work.

#### Implementation Steps:

1. **Module Registry**
   - Create a registry to store loaded modules
   - Use module paths as keys
   - Store module namespace objects as values

2. **Module Loading Process**
   - Check if the module is already loaded
   - If not, resolve the module path
   - Read the module file
   - Parse and compile the module
   - Evaluate the module in a new scope
   - Collect exports
   - Cache the module namespace

3. **Circular Dependency Handling**
   - Detect circular dependencies
   - Implement a two-phase loading process
   - Handle partially initialized modules

### 5. Testing

Create comprehensive tests for:
- Different import styles
- Different export styles
- Module resolution
- Circular dependencies
- Error cases (missing modules, invalid exports)
- Re-exports
- Side-effect only imports

## Implementation Details

### Module Object Structure

```rust
struct Module {
    id: String,           // Absolute path to the module
    url: String,          // URL representation of the path
    exports: Value,       // Module namespace object
    status: ModuleStatus, // Loading status
    dependencies: Vec<String>, // Module dependencies
}

enum ModuleStatus {
    Unlinked,   // Initial state
    Linking,    // Resolving dependencies
    Linked,     // Dependencies resolved
    Evaluating, // Running module code
    Evaluated,  // Module fully loaded
    Failed,     // Error occurred
}
```

### Module Namespace Object

The module namespace object contains all the exports from a module:

```rust
struct ModuleNamespace {
    exports: HashMap<String, Value>,
    default_export: Option<Value>,
}
```

### Module Loading Algorithm

1. **Parse**: Parse the module source into an AST
2. **Instantiate**: Create module environment and resolve imports
3. **Evaluate**: Execute the module code and collect exports
4. **Link**: Connect exports to imports in dependent modules

## Resources

- [MDN JavaScript Modules](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Modules)
- [ECMAScript Modules Specification](https://tc39.es/ecma262/#sec-modules)
- [Node.js ESM Implementation](https://nodejs.org/api/esm.html)

## Timeline

1. **Week 1**: Implement enhanced import syntax
2. **Week 2**: Implement export syntax
3. **Week 3**: Implement module resolution algorithm
4. **Week 4**: Implement module loading and caching
5. **Week 5**: Handle edge cases (circular dependencies, etc.)
6. **Week 6**: Comprehensive testing and bug fixing