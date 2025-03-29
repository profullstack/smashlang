/**
 * Native OS hooks implementation for SmashLang
 * 
 * This file provides functionality to interact with operating system hooks
 * such as signals, keyboard events, clipboard, and other OS-level events.
 */

const os = require('os');
const process = require('process');
const child_process = require('child_process');

// Try to load platform-specific modules
let clipboard;
let systemEvents;
let keyboardHook;
let mouseHook;

try {
  // These would be actual dependencies in a real implementation
  // clipboard = require('clipboard');
  // systemEvents = require('system-events');
  // keyboardHook = require('keyboard-hook');
  // mouseHook = require('mouse-hook');
} catch (error) {
  // Optional dependencies, so we can continue without them
  console.warn('Some OS hook dependencies could not be loaded. Limited functionality available.');
}

/**
 * Register OS hook native functions with the SmashLang runtime
 * @param {Object} runtime - The SmashLang runtime object
 */
function registerNativeFunctions(runtime) {
  // System signals
  registerSignalHandlers(runtime);
  
  // Clipboard operations
  registerClipboardFunctions(runtime);
  
  // Keyboard and mouse hooks
  registerInputHooks(runtime);
  
  // System notifications
  registerNotificationFunctions(runtime);
  
  // File system watchers
  registerFileWatchers(runtime);
}

/**
 * Register signal handling functions
 * @param {Object} runtime - The SmashLang runtime object
 */
function registerSignalHandlers(runtime) {
  // Get available signals
  runtime.registerNativeFunction('__native_get_signals', () => {
    return Object.keys(process.binding('constants').os.signals);
  });
  
  // Register a signal handler
  runtime.registerNativeFunction('__native_register_signal', (signal, callback) => {
    try {
      process.on(signal, () => {
        runtime.callFunction(callback, [signal]);
      });
      return true;
    } catch (error) {
      console.error(`Error registering signal handler for ${signal}:`, error);
      return false;
    }
  });
  
  // Send a signal to another process
  runtime.registerNativeFunction('__native_send_signal', (pid, signal) => {
    try {
      process.kill(pid, signal);
      return true;
    } catch (error) {
      console.error(`Error sending signal ${signal} to process ${pid}:`, error);
      return false;
    }
  });
}

/**
 * Register clipboard functions
 * @param {Object} runtime - The SmashLang runtime object
 */
function registerClipboardFunctions(runtime) {
  // Check if clipboard is available
  runtime.registerNativeFunction('__native_has_clipboard', () => {
    return !!clipboard;
  });
  
  // Read from clipboard
  runtime.registerNativeFunction('__native_clipboard_read', () => {
    if (!clipboard) {
      runtime.throwError('Clipboard functionality not available');
      return '';
    }
    
    try {
      return clipboard.readText() || '';
    } catch (error) {
      runtime.throwError(`Error reading from clipboard: ${error.message}`);
      return '';
    }
  });
  
  // Write to clipboard
  runtime.registerNativeFunction('__native_clipboard_write', (text) => {
    if (!clipboard) {
      runtime.throwError('Clipboard functionality not available');
      return false;
    }
    
    try {
      clipboard.writeText(text);
      return true;
    } catch (error) {
      runtime.throwError(`Error writing to clipboard: ${error.message}`);
      return false;
    }
  });
}

/**
 * Register keyboard and mouse hook functions
 * @param {Object} runtime - The SmashLang runtime object
 */
function registerInputHooks(runtime) {
  // Check if keyboard hooks are available
  runtime.registerNativeFunction('__native_has_keyboard_hooks', () => {
    return !!keyboardHook;
  });
  
  // Register keyboard event listener
  runtime.registerNativeFunction('__native_register_keyboard', (callback) => {
    if (!keyboardHook) {
      runtime.throwError('Keyboard hook functionality not available');
      return false;
    }
    
    try {
      keyboardHook.on('keydown', (event) => {
        runtime.callFunction(callback, ['keydown', event]);
      });
      
      keyboardHook.on('keyup', (event) => {
        runtime.callFunction(callback, ['keyup', event]);
      });
      
      return true;
    } catch (error) {
      runtime.throwError(`Error registering keyboard hook: ${error.message}`);
      return false;
    }
  });
  
  // Check if mouse hooks are available
  runtime.registerNativeFunction('__native_has_mouse_hooks', () => {
    return !!mouseHook;
  });
  
  // Register mouse event listener
  runtime.registerNativeFunction('__native_register_mouse', (callback) => {
    if (!mouseHook) {
      runtime.throwError('Mouse hook functionality not available');
      return false;
    }
    
    try {
      mouseHook.on('move', (event) => {
        runtime.callFunction(callback, ['move', event]);
      });
      
      mouseHook.on('click', (event) => {
        runtime.callFunction(callback, ['click', event]);
      });
      
      mouseHook.on('scroll', (event) => {
        runtime.callFunction(callback, ['scroll', event]);
      });
      
      return true;
    } catch (error) {
      runtime.throwError(`Error registering mouse hook: ${error.message}`);
      return false;
    }
  });
}

/**
 * Register system notification functions
 * @param {Object} runtime - The SmashLang runtime object
 */
function registerNotificationFunctions(runtime) {
  // Check if system notifications are available
  runtime.registerNativeFunction('__native_has_notifications', () => {
    return !!systemEvents;
  });
  
  // Send a system notification
  runtime.registerNativeFunction('__native_send_notification', (title, message, options = {}) => {
    if (!systemEvents) {
      // Fallback to console if system notifications aren't available
      console.log(`NOTIFICATION: ${title} - ${message}`);
      return false;
    }
    
    try {
      systemEvents.sendNotification(title, message, options);
      return true;
    } catch (error) {
      runtime.throwError(`Error sending notification: ${error.message}`);
      return false;
    }
  });
}

/**
 * Register file system watcher functions
 * @param {Object} runtime - The SmashLang runtime object
 */
function registerFileWatchers(runtime) {
  const watchers = new Map();
  
  // Watch a file or directory for changes
  runtime.registerNativeFunction('__native_watch_path', (path, callback) => {
    try {
      const fs = require('fs');
      
      if (watchers.has(path)) {
        runtime.throwError(`Already watching path: ${path}`);
        return false;
      }
      
      const watcher = fs.watch(path, (eventType, filename) => {
        runtime.callFunction(callback, [eventType, filename]);
      });
      
      watchers.set(path, watcher);
      return true;
    } catch (error) {
      runtime.throwError(`Error watching path ${path}: ${error.message}`);
      return false;
    }
  });
  
  // Stop watching a file or directory
  runtime.registerNativeFunction('__native_unwatch_path', (path) => {
    try {
      const watcher = watchers.get(path);
      if (watcher) {
        watcher.close();
        watchers.delete(path);
        return true;
      }
      return false;
    } catch (error) {
      runtime.throwError(`Error unwatching path ${path}: ${error.message}`);
      return false;
    }
  });
}

module.exports = { registerNativeFunctions };
