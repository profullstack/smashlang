# SmashLang Hardware Interfaces

The SmashLang Hardware Interfaces package provides access to various hardware devices and system capabilities, including cameras, microphones, screen recording, and other peripheral devices. This document provides a comprehensive guide to using these interfaces in your SmashLang applications.

## Table of Contents

- [Installation](#installation)
- [Camera](#camera)
- [Microphone](#microphone)
- [Screen Recording](#screen-recording)
- [Device Management](#device-management)
  - [Bluetooth](#bluetooth)
  - [USB](#usb)
  - [MIDI](#midi)
  - [Gamepad](#gamepad)
- [Examples](#examples)
- [Best Practices](#best-practices)

## Installation

The Hardware Interfaces package is included with SmashLang. Import it in your applications as follows:

```smash
import "hardware";
```

You can also import specific modules:

```smash
import { camera, microphone } from "hardware";
```

## Camera

The camera module provides access to connected camera devices for capturing photos and recording videos.

### Checking Availability

```smash
const cameraAvailable = await hardware.camera.isAvailable();
if (cameraAvailable) {
  print("Camera is available");
}
```

### Requesting Permission

Before using the camera, you must request user permission:

```smash
const permission = await hardware.camera.requestPermission();
if (!permission) {
  print("Camera permission denied");
  return;
}
```

### Listing Available Cameras

```smash
const cameras = await hardware.camera.getDevices();
print(`Found ${cameras.length} camera(s):`);

cameras.forEach((camera, index) => {
  print(`${index + 1}. ${camera.label} (${camera.id})`);
});
```

### Creating a Camera Stream

```smash
const cameraStream = new hardware.camera.CameraStream(deviceId, {
  width: 1280,
  height: 720,
  frameRate: 30,
  facingMode: 'user',  // 'user' for front camera, 'environment' for back camera
  audio: false         // Include audio from the camera's microphone
});

// Start the camera
await cameraStream.start();
```

### Taking Photos

```smash
// Take a photo and get the data
const photo = await cameraStream.takePhoto({
  format: 'jpeg',  // 'jpeg', 'png', 'webp'
  quality: 0.9     // 0.0 to 1.0
});

// Save a photo directly to a file
const photoPath = await cameraStream.savePhoto("./photo.jpg", {
  format: 'jpeg',
  quality: 0.9
});
```

### Recording Video

```smash
// Start recording
await cameraStream.startRecording({
  format: 'mp4',   // 'mp4', 'webm'
  quality: 0.8     // 0.0 to 1.0
});

// Record for a few seconds
await new Promise(resolve => setTimeout(resolve, 5000));

// Stop recording and save to a file
const videoPath = await cameraStream.stopRecording("./video.mp4");
```

### Applying Filters

```smash
// Apply a filter to the camera stream
await cameraStream.applyFilter("grayscale", { intensity: 1.0 });

// Remove all filters
await cameraStream.removeFilters();
```

### Stopping the Camera

```smash
// Stop the camera when done
cameraStream.stop();
```

## Microphone

The microphone module provides access to audio input devices for recording audio and performing speech recognition.

### Checking Availability

```smash
const microphoneAvailable = await hardware.microphone.isAvailable();
if (microphoneAvailable) {
  print("Microphone is available");
}
```

### Requesting Permission

```smash
const permission = await hardware.microphone.requestPermission();
if (!permission) {
  print("Microphone permission denied");
  return;
}
```

### Listing Available Microphones

```smash
const microphones = await hardware.microphone.getDevices();
print(`Found ${microphones.length} microphone(s):`);

microphones.forEach((mic, index) => {
  print(`${index + 1}. ${mic.label} (${mic.id})`);
});
```

### Creating a Microphone Stream

```smash
const micStream = new hardware.microphone.MicrophoneStream(deviceId, {
  sampleRate: 44100,            // Sample rate in Hz
  channels: 1,                  // 1 for mono, 2 for stereo
  echoCancellation: true,       // Enable echo cancellation
  noiseSuppression: true,       // Enable noise suppression
  autoGainControl: true         // Enable automatic gain control
});

// Start the microphone
await micStream.start();
```

### Getting Audio Level

```smash
// Get the current audio level (0.0 to 1.0)
const level = await micStream.getAudioLevel();
print(`Current audio level: ${Math.floor(level * 100)}%`);
```

### Recording Audio

```smash
// Start recording
await micStream.startRecording({
  format: 'wav',    // 'wav', 'mp3', 'ogg'
  quality: 0.9      // 0.0 to 1.0
});

// Record for a few seconds
await new Promise(resolve => setTimeout(resolve, 5000));

// Stop recording and save to a file
const audioPath = await micStream.stopRecording("./recording.wav");

// Or stop recording and get the data
const recordingData = await micStream.stopRecording();

// Save the recording data to a file later
const savedPath = await micStream.saveRecording("./recording.mp3", "mp3");
```

### Audio Processing

```smash
// Apply audio processing
await micStream.applyProcessor("noise-gate", { threshold: 0.1 });

// Remove all processors
await micStream.removeProcessors();
```

### Speech Recognition

```smash
// Perform speech recognition
const result = await hardware.microphone.recognizeSpeech({
  language: 'en-US',         // Language code
  continuous: false,          // Continuous recognition
  interimResults: false       // Return interim results
});

print(`Recognized text: "${result.transcript}"`);
```

### Stopping the Microphone

```smash
// Stop the microphone when done
micStream.stop();
```

## Screen Recording

The screen module provides capabilities for capturing screenshots and recording screen content.

### Checking Availability

```smash
const screenAvailable = await hardware.screen.isAvailable();
if (screenAvailable) {
  print("Screen recording is available");
}
```

### Requesting Permission

```smash
const permission = await hardware.screen.requestPermission();
if (!permission) {
  print("Screen recording permission denied");
  return;
}
```

### Listing Available Screen Sources

```smash
// Get all screens
const screens = await hardware.screen.getSources('screen');

// Get all application windows
const windows = await hardware.screen.getSources('window');

// Get all sources (screens, windows, applications)
const allSources = await hardware.screen.getSources();
```

### Taking Screenshots

```smash
// Take a screenshot and get the data
const screenshot = await hardware.screen.takeScreenshot(sourceId, {
  format: 'png',              // 'png', 'jpeg', 'webp'
  quality: 0.9,               // 0.0 to 1.0
  captureMouseCursor: true    // Include the mouse cursor
});

// Save a screenshot directly to a file
const screenshotPath = await hardware.screen.saveScreenshot(
  "./screenshot.png",
  sourceId,
  { captureMouseCursor: true }
);
```

### Screen Recording

```smash
// Create a screen recorder
const screenRecorder = new hardware.screen.ScreenRecorder(sourceId, {
  width: 1920,                // Desired width
  height: 1080,               // Desired height
  frameRate: 30,              // Frame rate
  captureMouseCursor: true,   // Include the mouse cursor
  captureClicks: true,        // Highlight mouse clicks
  captureAudio: false         // Include system audio
});

// Start recording
await screenRecorder.start({
  format: 'mp4',    // 'mp4', 'webm', 'gif'
  quality: 0.8      // 0.0 to 1.0
});

// Add a marker at a specific point in the recording
await screenRecorder.addMarker("Important moment");

// Pause recording
await screenRecorder.pause();

// Resume recording
await screenRecorder.resume();

// Stop recording and save to a file
const recordingInfo = await screenRecorder.stop("./screen_recording.mp4");
```

## Device Management

The devices module provides access to various hardware peripherals and system devices.

### Listing All Devices

```smash
// Get all connected devices
const devices = await hardware.devices.getDevices();

// Get devices of a specific type
const audioDevices = await hardware.devices.getDevices('audio');
```

### Monitoring Device Changes

```smash
// Start monitoring device connections/disconnections
await hardware.devices.monitorDevices((event) => {
  if (event.type === 'connected') {
    print(`Device connected: ${event.device.name}`);
  } else if (event.type === 'disconnected') {
    print(`Device disconnected: ${event.device.name}`);
  }
});

// Stop monitoring
await hardware.devices.stopMonitoring();
```

### Bluetooth

```smash
// Check if Bluetooth is available
const bluetoothAvailable = await hardware.devices.bluetooth.isAvailable();

// Enable Bluetooth
await hardware.devices.bluetooth.enable();

// Scan for Bluetooth devices
const bluetoothDevices = await hardware.devices.bluetooth.scan({
  timeout: 10000,     // Scan timeout in milliseconds
  lowEnergy: true      // Include BLE devices
});

// Connect to a Bluetooth device
const connection = await hardware.devices.bluetooth.connect(deviceId);

// Disconnect from a Bluetooth device
await hardware.devices.bluetooth.disconnect(deviceId);

// Disable Bluetooth
await hardware.devices.bluetooth.disable();
```

### USB

```smash
// Check if USB access is available
const usbAvailable = await hardware.devices.usb.isAvailable();

// Get all connected USB devices
const usbDevices = await hardware.devices.usb.getDevices();

// Request permission to access a USB device
const permission = await hardware.devices.usb.requestPermission(deviceId);

// Open a connection to a USB device
const connection = await hardware.devices.usb.open(deviceId);

// Transfer data to/from the USB device
const result = await hardware.devices.usb.transfer(deviceId, {
  direction: 'in',      // 'in' or 'out'
  endpoint: 1,           // Endpoint number
  data: new Uint8Array([1, 2, 3, 4])  // Data to send (for 'out')
});

// Close the USB connection
await hardware.devices.usb.close(deviceId);
```

### MIDI

```smash
// Check if MIDI access is available
const midiAvailable = await hardware.devices.midi.isAvailable();

// Get all MIDI input devices
const midiInputs = await hardware.devices.midi.getInputs();

// Get all MIDI output devices
const midiOutputs = await hardware.devices.midi.getOutputs();

// Open a MIDI input device and register for events
const inputConnection = await hardware.devices.midi.openInput(deviceId, (message) => {
  print(`MIDI message received: ${message}`);
});

// Open a MIDI output device
const outputConnection = await hardware.devices.midi.openOutput(deviceId);

// Send a MIDI message
await hardware.devices.midi.send(deviceId, [144, 60, 100]);  // Note on, middle C, velocity 100

// Close MIDI connections
await hardware.devices.midi.close(inputDeviceId, 'input');
await hardware.devices.midi.close(outputDeviceId, 'output');
```

### Gamepad

```smash
// Check if gamepad access is available
const gamepadAvailable = await hardware.devices.gamepad.isAvailable();

// Get all connected gamepads
const gamepads = await hardware.devices.gamepad.getDevices();

// Register for gamepad events
await hardware.devices.gamepad.registerEvents((event) => {
  if (event.type === 'button') {
    print(`Button ${event.button}: ${event.pressed ? 'Pressed' : 'Released'} (${event.value})`);
  } else if (event.type === 'axis') {
    print(`Axis ${event.axis}: ${event.value}`);
  }
});

// Get the current state of a gamepad
const state = await hardware.devices.gamepad.getState(deviceId);

// Unregister from gamepad events
await hardware.devices.gamepad.unregisterEvents();
```

## Examples

For complete examples of using the hardware interfaces, see the [hardware_example.smash](../examples/hardware_example.smash) file included with SmashLang.

## Best Practices

1. **Always check availability first**: Before using any hardware interface, check if it's available on the current device.

2. **Request permissions early**: Request user permissions at the start of your application, before you need to use the hardware.

3. **Handle errors gracefully**: Hardware access can fail for various reasons. Always wrap hardware calls in try/catch blocks.

4. **Release resources**: Always stop streams and close connections when you're done with them.

5. **Respect battery life**: Hardware interfaces can drain battery quickly. Use them only when necessary and stop them when not in use.

6. **Consider privacy**: Be transparent about how you're using hardware like cameras and microphones. Provide visual indicators when they're active.

7. **Test across platforms**: Hardware capabilities can vary across different operating systems and devices. Test your application on multiple platforms.

8. **Provide fallbacks**: Some devices may not have certain hardware. Always provide alternative functionality when hardware is not available.
