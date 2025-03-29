/**
 * SmashLang Runtime
 * 
 * This is the main entry point for the SmashLang runtime.
 * It initializes the runtime environment and registers all native functions.
 */

const { registerNativeFunctions: registerProcessFunctions } = require('./native_process');
const { registerNativeFunctions: registerOSHooksFunctions } = require('./native_os_hooks');

/**
 * SmashLang Runtime class
 * Handles execution of SmashLang code and provides native function bindings
 */
class SmashLangRuntime {
  constructor() {
    this.nativeFunctions = new Map();
    this.initialize();
  }

  /**
   * Initialize the runtime
   */
  initialize() {
    // Register all native functions
    this.registerCoreFunctions();
    registerProcessFunctions(this);
    registerOSHooksFunctions(this);
    
    console.log('SmashLang runtime initialized');
  }

  /**
   * Register a native function with the runtime
   * @param {string} name - Name of the native function
   * @param {Function} fn - JavaScript function implementation
   */
  registerNativeFunction(name, fn) {
    this.nativeFunctions.set(name, fn);
  }

  /**
   * Call a registered native function
   * @param {string} name - Name of the native function
   * @param {Array} args - Arguments to pass to the function
   * @returns {any} - Result of the function call
   */
  callNativeFunction(name, args = []) {
    const fn = this.nativeFunctions.get(name);
    if (!fn) {
      this.throwError(`Native function '${name}' not found`);
      return null;
    }
    
    try {
      return fn(...args);
    } catch (error) {
      this.throwError(`Error calling native function '${name}': ${error.message}`);
      return null;
    }
  }

  /**
   * Call a SmashLang function
   * @param {string} name - Name of the function to call
   * @param {Array} args - Arguments to pass to the function
   * @returns {any} - Result of the function call
   */
  callFunction(name, args = []) {
    // This would be implemented to call a SmashLang function
    // For now, we'll just log it
    console.log(`Calling SmashLang function '${name}' with args:`, args);
    return null;
  }

  /**
   * Throw a runtime error
   * @param {string} message - Error message
   */
  throwError(message) {
    console.error(`SmashLang Runtime Error: ${message}`);
    // In a real implementation, this would throw an exception or handle errors appropriately
  }

  /**
   * Register core native functions
   */
  registerCoreFunctions() {
    // Core functions like print, etc.
    this.registerNativeFunction('print', (value) => {
      console.log(value);
      return value;
    });

    // Promise support
    this.registerNativeFunction('__native_create_promise', (executor) => {
      return new Promise(executor);
    });

    this.registerNativeFunction('__is_promise', (value) => {
      return value instanceof Promise;
    });
  }

  /**
   * Execute SmashLang code
   * @param {string} code - SmashLang code to execute
   * @returns {any} - Result of execution
   */
  execute(code) {
    // This would be implemented to parse and execute SmashLang code
    // For now, we'll just log it
    console.log('Executing SmashLang code:', code);
    return null;
  }
}

// Export the runtime
module.exports = SmashLangRuntime;
