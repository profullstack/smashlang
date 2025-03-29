# SmashLang Hardware Interfaces Overview

SmashLang provides a comprehensive set of hardware interfaces that allow your applications to interact with various hardware devices across all supported platforms. This includes cameras, microphones, screens, and input devices like keyboards, mice, and touch screens.

## Supported Hardware Interfaces

SmashLang's hardware module provides access to the following hardware interfaces:

- **Camera**: Capture images and video from connected cameras
- **Microphone**: Record audio from connected microphones
- **Screen**: Capture screenshots and record screen activity
- **Input Devices**: Interact with keyboards, mice, and touch screens

## Cross-Platform Support

The hardware interfaces are designed to work consistently across all platforms supported by SmashLang:

- **Windows**: Using WinAPI, Media Foundation, and DirectShow
- **macOS**: Using AVFoundation and Core Graphics
- **Linux**: Using V4L2, ALSA, X11, and Wayland
- **Android**: Using Android Camera API and AudioRecord
- **iOS**: Using AVFoundation

## Basic Usage

To use the hardware interfaces, you need to import the hardware module:

```js
import { camera, microphone, screen, input } from "hardware";
```

Each hardware interface provides a set of functions to interact with the corresponding hardware device. For example, to capture an image from a camera:

```js
// Check if a camera is available
if (camera.isAvailable()) {
  // Capture an image
  const image = camera.captureImage();
  
  // Save the image to a file
  image.save("capture.jpg");
}
```

## Hardware Permissions

Accessing hardware devices often requires user permissions, especially on mobile platforms. SmashLang handles permission requests automatically, but your application should be prepared to handle cases where permissions are denied.

```js
// Check if microphone permission is granted
if (microphone.hasPermission()) {
  // Start recording
  microphone.startRecording("recording.wav");
} else {
  // Request permission
  microphone.requestPermission().then(granted => {
    if (granted) {
      microphone.startRecording("recording.wav");
    } else {
      print("Microphone permission denied");
    }
  });
}
```

## Hardware Detection

SmashLang provides functions to detect available hardware devices and their capabilities:

```js
// List all available cameras
const cameras = camera.listDevices();
print(`Found ${cameras.length} cameras`);

// Print camera details
cameras.forEach(cam => {
  print(`Camera: ${cam.name}`);
  print(`  Resolution: ${cam.maxWidth}x${cam.maxHeight}`);
  print(`  FPS: ${cam.maxFps}`);
});

// Check if screen capture is available
if (screen.isAvailable()) {
  print("Screen capture is available");
  print(`Max resolution: ${screen.maxWidth}x${screen.maxHeight}`);
}
```

## Detailed Documentation

For more detailed information about each hardware interface, check out the following guides:

- [Camera Access](./camera.md)
- [Microphone Access](./microphone.md)
- [Screen Capture](./screen.md)
- [Input Devices](./input.md)
- [Cross-Platform Support](./cross-platform.md)

## Examples

Check out the [examples directory](./examples/) for complete examples of using the hardware interfaces in SmashLang applications.
