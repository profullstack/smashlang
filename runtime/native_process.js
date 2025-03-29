/**
 * Native process-related functions for SmashLang runtime
 * 
 * This file implements the native functions that the std module uses
 * to interact with the operating system and process environment.
 */

const os = require('os');
const process = require('process');

/**
 * Register all process-related native functions with the SmashLang runtime
 * @param {Object} runtime - The SmashLang runtime object
 */
function registerNativeFunctions(runtime) {
  // Environment variables
  runtime.registerNativeFunction('__native_get_env', (name) => {
    return process.env[name] || '';
  });

  runtime.registerNativeFunction('__native_set_env', (name, value) => {
    process.env[name] = value;
    return true;
  });

  // Command line arguments
  runtime.registerNativeFunction('__native_get_argv', () => {
    return process.argv;
  });

  // Current working directory
  runtime.registerNativeFunction('__native_get_cwd', () => {
    return process.cwd();
  });

  runtime.registerNativeFunction('__native_set_cwd', (directory) => {
    try {
      process.chdir(directory);
      return true;
    } catch (error) {
      runtime.throwError(`Failed to change directory: ${error.message}`);
      return false;
    }
  });

  // Platform information
  runtime.registerNativeFunction('__native_get_platform', () => {
    return process.platform;
  });

  runtime.registerNativeFunction('__native_get_arch', () => {
    return process.arch;
  });

  // Process control
  runtime.registerNativeFunction('__native_exit', (code = 0) => {
    // We don't actually exit immediately, as that would be abrupt
    // Instead, we schedule the exit to happen after current execution completes
    setTimeout(() => {
      process.exit(code);
    }, 0);
    return true;
  });

  // Process information
  runtime.registerNativeFunction('__native_get_pid', () => {
    return process.pid;
  });

  runtime.registerNativeFunction('__native_get_ppid', () => {
    return process.ppid;
  });

  // Memory usage
  runtime.registerNativeFunction('__native_memory_usage', () => {
    return process.memoryUsage();
  });

  // CPU usage
  runtime.registerNativeFunction('__native_cpu_usage', () => {
    return process.cpuUsage();
  });

  // High-resolution time measurement
  runtime.registerNativeFunction('__native_hrtime', () => {
    const [seconds, nanoseconds] = process.hrtime();
    return {
      seconds,
      nanoseconds,
      // Also include a milliseconds representation for convenience
      milliseconds: seconds * 1000 + nanoseconds / 1000000
    };
  });

  // Register event handlers
  setupEventHandlers(runtime);
}

/**
 * Set up event handlers for process events
 * @param {Object} runtime - The SmashLang runtime object
 */
function setupEventHandlers(runtime) {
  // Handle process exit
  process.on('exit', (code) => {
    runtime.callFunction('__triggerEvent', ['exit', code]);
  });

  // Handle uncaught exceptions
  process.on('uncaughtException', (error) => {
    runtime.callFunction('__triggerEvent', ['uncaughtException', error.toString()]);
  });

  // Handle unhandled promise rejections
  process.on('unhandledRejection', (reason, promise) => {
    runtime.callFunction('__triggerEvent', ['unhandledRejection', reason.toString()]);
  });
}

module.exports = { registerNativeFunctions };
