# Input Devices in SmashLang

The SmashLang hardware module provides a comprehensive API for interacting with input devices such as keyboards, mice, and touch screens across all supported platforms.

## Overview

The input module allows your SmashLang applications to:

- Check the availability of input devices
- Register for input events (key presses, mouse movements, touch gestures)
- Simulate input events
- Query the current state of input devices

## Importing the Input Module

```js
// Import the entire hardware module
import { hardware } from "std";
const { input } = hardware;

// Or import the input module directly
import { input } from "hardware";
```

## Checking Device Availability

Before using an input device, you should check if it's available on the current platform:

```js
// Check if keyboard is available
if (input.isKeyboardAvailable()) {
  print("Keyboard is available");
}

// Check if mouse is available
if (input.isMouseAvailable()) {
  print("Mouse is available");
}

// Check if touch input is available
if (input.isTouchAvailable()) {
  print("Touch input is available");
}
```

## Registering for Input Events

You can register callbacks to be notified when input events occur:

```js
// Register for keyboard events
const keyboardRegistration = input.registerEvents(["keyboard"], event => {
  if (event.type === "keydown") {
    print(`Key pressed: ${event.key}`);
  } else if (event.type === "keyup") {
    print(`Key released: ${event.key}`);
  }
});

// Register for mouse events
const mouseRegistration = input.registerEvents(["mouse"], event => {
  if (event.type === "mousemove") {
    print(`Mouse moved to: ${event.x}, ${event.y}`);
  } else if (event.type === "mousedown") {
    print(`Mouse button ${event.button} pressed at: ${event.x}, ${event.y}`);
  } else if (event.type === "mouseup") {
    print(`Mouse button ${event.button} released at: ${event.x}, ${event.y}`);
  }
});

// Register for touch events
const touchRegistration = input.registerEvents(["touch"], event => {
  if (event.type === "touchstart") {
    print(`Touch started with ${event.touches.length} fingers`);
  } else if (event.type === "touchmove") {
    print(`Touch moved to: ${event.touches[0].x}, ${event.touches[0].y}`);
  } else if (event.type === "touchend") {
    print(`Touch ended`);
  }
});

// Register for multiple event types at once
const allInputRegistration = input.registerEvents(["keyboard", "mouse", "touch"], event => {
  print(`Received ${event.type} event`);
});
```

## Unregistering Events

When you're done listening for events, you should unregister to free up resources:

```js
// Unregister keyboard events
input.unregisterEvents(keyboardRegistration);

// Unregister mouse events
input.unregisterEvents(mouseRegistration);

// Unregister touch events
input.unregisterEvents(touchRegistration);

// Unregister all input events
input.unregisterEvents(allInputRegistration);
```

## Simulating Input

You can simulate input events programmatically:

```js
// Simulate key press and release
await input.simulateKeyPress("a");

// Simulate key down
await input.simulateKeyDown("Control");

// Simulate key up
await input.simulateKeyUp("Control");

// Simulate mouse movement
await input.simulateMouseMove(100, 200);

// Simulate mouse button press and release
await input.simulateMouseClick("left", 100, 200);

// Simulate mouse button down
await input.simulateMouseDown("right", 300, 400);

// Simulate mouse button up
await input.simulateMouseUp("right", 300, 400);

// Simulate touch event
await input.simulateTouch([
  { id: 1, x: 100, y: 200, pressure: 1.0 }
]);
```

## Querying Input State

You can query the current state of input devices:

```js
// Get the current state of all keyboard keys
const keyboardState = input.getKeyboardState();
if (keyboardState["Control"] && keyboardState["c"]) {
  print("Control+C is pressed");
}

// Get the current mouse position
const [mouseX, mouseY] = input.getMousePosition();
print(`Mouse is at: ${mouseX}, ${mouseY}`);

// Get the current state of mouse buttons
const mouseState = input.getMouseButtonState();
if (mouseState["left"]) {
  print("Left mouse button is pressed");
}

// Get the current touch points
const touchPoints = input.getTouchPoints();
print(`There are ${touchPoints.length} active touch points`);
touchPoints.forEach(point => {
  print(`Touch point ${point.id} at ${point.x}, ${point.y} with pressure ${point.pressure}`);
});
```

## Key Codes

The input module uses standard key codes for keyboard events. Here are some common key codes:

- Letter keys: `"a"` through `"z"` (lowercase)
- Number keys: `"0"` through `"9"`
- Function keys: `"F1"` through `"F12"`
- Special keys: `"Enter"`, `"Escape"`, `"Backspace"`, `"Tab"`, `"Space"`
- Modifier keys: `"Control"`, `"Shift"`, `"Alt"`, `"Meta"` (Command on macOS)
- Arrow keys: `"ArrowUp"`, `"ArrowDown"`, `"ArrowLeft"`, `"ArrowRight"`

## Mouse Buttons

Mouse buttons are identified by the following strings:

- `"left"`: Left mouse button
- `"right"`: Right mouse button
- `"middle"`: Middle mouse button (scroll wheel)
- `"back"`: Back button (if available)
- `"forward"`: Forward button (if available)

## Platform-Specific Considerations

### Linux

On Linux, the input module supports both X11 and Wayland display servers:

- **X11**: Full support for keyboard, mouse, and limited touch input
- **Wayland**: Support for keyboard and mouse events, but input simulation is limited due to Wayland's security model

### macOS

On macOS, some input operations may require accessibility permissions. Your application should prompt the user to grant these permissions if needed.

### Windows

Windows provides full support for keyboard, mouse, and touch input through the Windows API.

### Android and iOS

On mobile platforms, keyboard input is typically handled through on-screen keyboards, and touch is the primary input method. The input module provides a consistent API across all platforms, but the available devices may vary.

## Example: Simple Key Logger

```js
import { input } from "hardware";
import { fs } from "std";

// Create a log file
const logFile = fs.openSync("keylog.txt", "w");

// Register for keyboard events
const registration = input.registerEvents(["keyboard"], event => {
  if (event.type === "keydown") {
    // Log the key press
    fs.writeSync(logFile, `${new Date().toISOString()}: ${event.key}\n`);
  }
});

// Keep the program running
print("Key logger started. Press Ctrl+C to stop.");
while (true) {
  await sleep(1000);
}

// Clean up (this code won't be reached in this example)
fs.closeSync(logFile);
input.unregisterEvents(registration);
```

## Example: Mouse Tracker

```js
import { input } from "hardware";
import { sleep } from "std/time";

// Track mouse movement
print("Mouse position tracker. Press Ctrl+C to stop.");

while (true) {
  const [x, y] = input.getMousePosition();
  print(`Mouse position: ${x}, ${y}`);
  await sleep(500); // Update every 500ms
}
```

## See Also

- [Hardware Overview](./overview.md)
- [Camera Access](./camera.md)
- [Microphone Access](./microphone.md)
- [Screen Capture](./screen.md)
