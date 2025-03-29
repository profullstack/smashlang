# SmashLang Standard Library: OS Hooks API

The SmashLang OS Hooks API (`std_os_hooks`) provides access to system-level hooks, events, and native operating system functionality. This module allows SmashLang applications to interact with the operating system at a lower level than traditional APIs.

## Table of Contents

- [Importing the Module](#importing-the-module)
- [Signal Handling](#signal-handling)
- [Clipboard Operations](#clipboard-operations)
- [Keyboard Hooks](#keyboard-hooks)
- [Mouse Hooks](#mouse-hooks)
- [System Notifications](#system-notifications)
- [File System Watchers](#file-system-watchers)
- [Examples](#examples)
- [Platform Support](#platform-support)

## Importing the Module

To use the OS Hooks API, import it as follows:

```smash
import "std_os_hooks" as os;
```

## Signal Handling

The `signals` module provides functionality to handle system signals (such as SIGINT, SIGTERM, etc.).

```smash
// Get all available signals
const availableSignals = os.signals.getAvailable();

// Register a handler for SIGINT (Ctrl+C)
os.signals.register('SIGINT', (signal) => {
  print(`Received ${signal} signal!`);
  return true; // Prevent default behavior
});

// Send a signal to another process
os.signals.send(1234, 'SIGTERM');
```

## Clipboard Operations

The `clipboard` module provides access to the system clipboard.

```smash
// Check if clipboard functionality is available
if (os.clipboard.isAvailable()) {
  // Write to clipboard
  os.clipboard.write("Hello from SmashLang!");
  
  // Read from clipboard
  const clipboardContent = os.clipboard.read();
  print(`Clipboard content: ${clipboardContent}`);
}
```

## Keyboard Hooks

The `keyboard` module allows you to register global keyboard event listeners.

```smash
// Check if keyboard hook functionality is available
if (os.keyboard.isAvailable()) {
  // Register keyboard event handler
  os.keyboard.register((eventType, event) => {
    if (eventType === 'keydown') {
      print(`Key pressed: ${event.key} (keyCode: ${event.keyCode})`);
      
      // Exit on Escape key
      if (event.key === 'Escape') {
        return false; // Stop listening
      }
    }
    return true; // Continue listening
  });
}
```

## Mouse Hooks

The `mouse` module allows you to register global mouse event listeners.

```smash
// Check if mouse hook functionality is available
if (os.mouse.isAvailable()) {
  // Register mouse event handler
  os.mouse.register((eventType, event) => {
    if (eventType === 'click') {
      print(`Mouse clicked at position (${event.x}, ${event.y}) with button ${event.button}`);
    } else if (eventType === 'move') {
      print(`Mouse moved to position (${event.x}, ${event.y})`);
    }
    return true; // Continue listening
  });
}
```

## System Notifications

The `notifications` module provides functionality to send system notifications.

```smash
// Check if notification functionality is available
if (os.notifications.isAvailable()) {
  // Send a notification
  os.notifications.send(
    "SmashLang Notification", 
    "This is a test notification from SmashLang!",
    { 
      icon: "info",
      sound: true
    }
  );
}
```

## File System Watchers

The `fileWatcher` module allows you to watch files and directories for changes.

```smash
// Watch a file for changes
os.fileWatcher.watch('/path/to/file.txt', (eventType, filename) => {
  print(`File event: ${eventType} on ${filename}`);
});

// Stop watching a file
os.fileWatcher.unwatch('/path/to/file.txt');
```

## Examples

### Signal Handler Example

```smash
import "std";
import "std_os_hooks" as os;

// Register a handler for SIGINT (Ctrl+C)
os.signals.register('SIGINT', (signal) => {
  print(`\nReceived ${signal} signal! Press Ctrl+C again to exit.`);
  
  // Register a one-time handler for the next SIGINT
  os.signals.register('SIGINT', (signal) => {
    print(`\nReceived second ${signal} signal. Exiting...`);
    std.exit(0);
  });
  
  // Prevent the default behavior
  return true;
});

print("Press Ctrl+C to test signal handling...");

// Keep the program running
while (true) {
  std.sleep(1000);
}
```

### File Watcher Example

```smash
import "std";
import "std_os_hooks" as os;

// Watch a directory for changes
const watchPath = "/path/to/watch";

os.fileWatcher.watch(watchPath, (eventType, filename) => {
  print(`${new Date().toISOString()}: ${eventType} detected on ${filename || 'unknown'}`);
  
  if (filename && filename.endsWith(".tmp")) {
    print("Temporary file change detected - ignoring");
    return;
  }
  
  print(`Processing change to ${filename}...`);
});

print(`Watching ${watchPath} for changes. Press Ctrl+C to exit.`);

// Keep the program running
while (true) {
  std.sleep(1000);
}
```

## Platform Support

The availability of OS hooks functionality depends on the platform and environment. Always check for availability using the `isAvailable()` methods before using these features.

| Feature | Linux | macOS | Windows |
|---------|-------|-------|--------|
| Signals | ✅ | ✅ | ⚠️ Limited |
| Clipboard | ✅ | ✅ | ✅ |
| Keyboard Hooks | ✅ | ✅ | ✅ |
| Mouse Hooks | ✅ | ✅ | ✅ |
| System Notifications | ✅ | ✅ | ✅ |
| File Watchers | ✅ | ✅ | ✅ |

For more examples, see the [os_hooks_example.smash](../examples/os_hooks_example.smash) file in the examples directory.
