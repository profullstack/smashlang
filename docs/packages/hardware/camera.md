# Camera Package

<p align="center">
  <img src="../../../smashlang_packages/hardware/camera/assets/logo.light.svg" alt="Camera Package Logo" width="200" />
</p>

The Camera package provides a comprehensive API for accessing and controlling camera devices in SmashLang applications. It supports photo capture, video recording, and camera configuration across multiple platforms.

## Installation

```bash
smashpkg install camera
```

## Features

- Device enumeration and selection
- Photo capture with various formats and quality settings
- Video recording with configurable parameters
- Camera configuration (resolution, frame rate, exposure, etc.)
- Real-time preview and streaming
- Image processing and filters
- Face and object detection
- QR code and barcode scanning
- Cross-platform support (desktop, mobile, web)
- Permission handling

## Basic Usage

```js
import { camera } from "camera";

// Check if camera is available
const isAvailable = await camera.isAvailable();
if (!isAvailable) {
  console.error("Camera is not available on this device");
  return;
}

// Request camera permission
const permission = await camera.requestPermission();
if (!permission) {
  console.error("Camera permission denied");
  return;
}

// List available cameras
const devices = await camera.getDevices();
console.log(`Found ${devices.length} camera(s):`);
for (const device of devices) {
  console.log(`- ${device.label} (${device.id})`);
}

// Select the first camera (usually the default)
const deviceId = devices[0].id;

// Create a camera stream
const cameraStream = new camera.CameraStream(deviceId, {
  width: 1280,
  height: 720,
  frameRate: 30,
  facingMode: 'user' // 'user' for front camera, 'environment' for back camera
});

// Start the camera
await cameraStream.start();

// Take a photo
const photo = await cameraStream.takePhoto({
  format: 'jpeg',
  quality: 0.9
});

// Save the photo to a file
await photo.saveToFile("./photo.jpg");

// Stop the camera when done
cameraStream.stop();
```

## Advanced Usage

### Video Recording

```js
import { camera } from "camera";

// Create and start a camera stream
const cameraStream = new camera.CameraStream(deviceId, {
  width: 1920,
  height: 1080,
  frameRate: 30,
  audio: true // Include audio from the device's microphone
});

await cameraStream.start();

// Start recording video
await cameraStream.startRecording({
  format: 'mp4',
  quality: 'high',
  maxDuration: 60000 // 60 seconds
});

console.log("Recording started...");

// Stop recording after 5 seconds
setTimeout(async () => {
  const videoFile = await cameraStream.stopRecording();
  console.log(`Recording saved to: ${videoFile.path}`);
  
  // Save to a specific location
  await videoFile.saveToFile("./my_video.mp4");
  
  // Stop the camera
  cameraStream.stop();
}, 5000);
```

### Camera Configuration

```js
import { camera } from "camera";

const cameraStream = new camera.CameraStream(deviceId);

// Get available capabilities
const capabilities = await cameraStream.getCapabilities();
console.log("Available resolutions:", capabilities.resolutions);
console.log("Zoom range:", capabilities.zoom);
console.log("Focus modes:", capabilities.focusModes);

// Configure camera settings
await cameraStream.configure({
  resolution: { width: 3840, height: 2160 }, // 4K
  zoom: 1.5,
  focusMode: 'continuous',
  whiteBalance: 'auto',
  exposureMode: 'auto',
  iso: 400,
  flashMode: 'auto'
});

await cameraStream.start();

// Adjust settings while camera is running
await cameraStream.setZoom(2.0);
await cameraStream.setFocusPoint(0.5, 0.5); // Center of frame
await cameraStream.setExposureCompensation(1.0); // Brighter
```

### Image Processing and Filters

```js
import { camera } from "camera";

const cameraStream = new camera.CameraStream(deviceId);
await cameraStream.start();

// Apply filters to the live preview
await cameraStream.applyFilter("grayscale");

// Take a photo with the filter applied
const photo = await cameraStream.takePhoto();

// Apply multiple filters with parameters
await cameraStream.applyFilter("sepia", { intensity: 0.7 });
await cameraStream.applyFilter("vignette", { amount: 0.5 });

// Remove all filters
await cameraStream.removeFilters();

// Process a photo after capture
const processedPhoto = await photo.process([
  { filter: "brightness", options: { level: 1.2 } },
  { filter: "contrast", options: { level: 1.1 } },
  { filter: "sharpen", options: { amount: 0.5 } }
]);

await processedPhoto.saveToFile("./enhanced_photo.jpg");
```

### Face and Object Detection

```js
import { camera } from "camera";

const cameraStream = new camera.CameraStream(deviceId);
await cameraStream.start();

// Enable face detection
await cameraStream.enableFaceDetection({
  trackMultiple: true,
  minSize: 0.1 // Minimum face size as a proportion of frame
});

// Listen for face detection events
cameraStream.on("faceDetected", (faces) => {
  console.log(`Detected ${faces.length} faces`);
  for (const face of faces) {
    console.log(`Face at (${face.x}, ${face.y}), size: ${face.width}x${face.height}`);
    console.log(`Confidence: ${face.confidence}`);
    
    // Face landmarks if available
    if (face.landmarks) {
      console.log("Eyes:", face.landmarks.eyes);
      console.log("Nose:", face.landmarks.nose);
      console.log("Mouth:", face.landmarks.mouth);
    }
  }
});

// Enable object detection
await cameraStream.enableObjectDetection({
  models: ["general", "person"],
  confidence: 0.7
});

// Listen for object detection events
cameraStream.on("objectDetected", (objects) => {
  console.log(`Detected ${objects.length} objects`);
  for (const obj of objects) {
    console.log(`${obj.class} (${obj.confidence.toFixed(2)}) at (${obj.x}, ${obj.y})`);
  }
});
```

### QR Code and Barcode Scanning

```js
import { camera } from "camera";

const cameraStream = new camera.CameraStream(deviceId);
await cameraStream.start();

// Enable barcode scanning
await cameraStream.enableBarcodeScanning({
  formats: ["qr", "code128", "ean13", "pdf417"],
  scanInterval: 500 // ms between scans
});

// Listen for barcode detection events
cameraStream.on("barcodeDetected", (barcodes) => {
  for (const barcode of barcodes) {
    console.log(`Detected ${barcode.format}: ${barcode.data}`);
    
    // Parse QR code content if it's a URL
    if (barcode.format === "qr" && barcode.data.startsWith("http")) {
      console.log(`QR code contains URL: ${barcode.data}`);
    }
  }
});

// Take a single scan of the current frame
const scanResult = await cameraStream.scanBarcodes();
if (scanResult.length > 0) {
  console.log(`Found ${scanResult.length} barcodes`);
}
```

## API Reference

### Camera Module

#### `camera.isAvailable()`
Checks if camera functionality is available on the device.
- **Returns**: (Promise<Boolean>) Promise resolving to true if camera is available

#### `camera.requestPermission()`
Requests permission to use the camera.
- **Returns**: (Promise<Boolean>) Promise resolving to true if permission is granted

#### `camera.getDevices()`
Gets a list of available camera devices.
- **Returns**: (Promise<Array<Device>>) Promise resolving to an array of camera devices

### Device Object

#### `device.id`
Unique identifier for the camera device.
- **Type**: (String)

#### `device.label`
Human-readable label for the camera device.
- **Type**: (String)

#### `device.facingMode`
The direction the camera faces.
- **Type**: (String) 'user' (front) or 'environment' (back)

#### `device.capabilities`
The capabilities of the camera device.
- **Type**: (Object)

### CameraStream Class

#### `new CameraStream(deviceId, options)`
Creates a new camera stream.
- **Parameters**: 
  - `deviceId` (String): Camera device ID
  - `options` (Object, optional): Configuration options
    - `width` (Number): Desired width
    - `height` (Number): Desired height
    - `frameRate` (Number): Desired frame rate
    - `facingMode` (String): 'user' or 'environment'
    - `audio` (Boolean): Whether to include audio
- **Returns**: (CameraStream) New CameraStream instance

#### `cameraStream.start()`
Starts the camera stream.
- **Returns**: (Promise<void>) Promise that resolves when the camera starts

#### `cameraStream.stop()`
Stops the camera stream.
- **Returns**: (void)

#### `cameraStream.takePhoto(options)`
Takes a photo.
- **Parameters**: 
  - `options` (Object, optional): Photo options
    - `format` (String): 'jpeg', 'png', or 'webp'
    - `quality` (Number): 0.0 to 1.0
- **Returns**: (Promise<Photo>) Promise resolving to a Photo object

#### `cameraStream.startRecording(options)`
Starts video recording.
- **Parameters**: 
  - `options` (Object, optional): Recording options
    - `format` (String): 'mp4' or 'webm'
    - `quality` (String|Number): 'low', 'medium', 'high', or 0.0 to 1.0
    - `maxDuration` (Number): Maximum duration in milliseconds
- **Returns**: (Promise<void>) Promise that resolves when recording starts

#### `cameraStream.stopRecording()`
Stops video recording.
- **Returns**: (Promise<VideoFile>) Promise resolving to a VideoFile object

#### `cameraStream.getCapabilities()`
Gets the capabilities of the camera.
- **Returns**: (Promise<Object>) Promise resolving to capabilities object

#### `cameraStream.configure(settings)`
Configures camera settings.
- **Parameters**: 
  - `settings` (Object): Camera settings
- **Returns**: (Promise<void>) Promise that resolves when settings are applied

#### `cameraStream.applyFilter(filterName, options)`
Applies a filter to the camera stream.
- **Parameters**: 
  - `filterName` (String): Name of the filter
  - `options` (Object, optional): Filter options
- **Returns**: (Promise<void>) Promise that resolves when filter is applied

#### `cameraStream.removeFilters()`
Removes all filters from the camera stream.
- **Returns**: (Promise<void>) Promise that resolves when filters are removed

#### `cameraStream.enableFaceDetection(options)`
Enables face detection.
- **Parameters**: 
  - `options` (Object, optional): Face detection options
- **Returns**: (Promise<void>) Promise that resolves when face detection is enabled

#### `cameraStream.enableObjectDetection(options)`
Enables object detection.
- **Parameters**: 
  - `options` (Object, optional): Object detection options
- **Returns**: (Promise<void>) Promise that resolves when object detection is enabled

#### `cameraStream.enableBarcodeScanning(options)`
Enables barcode scanning.
- **Parameters**: 
  - `options` (Object, optional): Barcode scanning options
- **Returns**: (Promise<void>) Promise that resolves when barcode scanning is enabled

#### `cameraStream.on(eventName, callback)`
Registers an event listener.
- **Parameters**: 
  - `eventName` (String): Name of the event
  - `callback` (Function): Event handler function
- **Returns**: (void)

### Photo Class

#### `photo.format`
The format of the photo.
- **Type**: (String)

#### `photo.width`
The width of the photo in pixels.
- **Type**: (Number)

#### `photo.height`
The height of the photo in pixels.
- **Type**: (Number)

#### `photo.size`
The size of the photo in bytes.
- **Type**: (Number)

#### `photo.saveToFile(path)`
Saves the photo to a file.
- **Parameters**: 
  - `path` (String): File path
- **Returns**: (Promise<void>) Promise that resolves when the file is saved

#### `photo.toBase64()`
Converts the photo to a base64 string.
- **Returns**: (Promise<String>) Promise resolving to base64 string

#### `photo.process(filters)`
Applies processing filters to the photo.
- **Parameters**: 
  - `filters` (Array<Object>): Array of filter objects
- **Returns**: (Promise<Photo>) Promise resolving to a new processed Photo

### VideoFile Class

#### `videoFile.format`
The format of the video.
- **Type**: (String)

#### `videoFile.duration`
The duration of the video in milliseconds.
- **Type**: (Number)

#### `videoFile.size`
The size of the video in bytes.
- **Type**: (Number)

#### `videoFile.path`
The temporary path of the video file.
- **Type**: (String)

#### `videoFile.saveToFile(path)`
Saves the video to a file.
- **Parameters**: 
  - `path` (String): File path
- **Returns**: (Promise<void>) Promise that resolves when the file is saved

## Examples

See the [examples directory](../../../smashlang_packages/hardware/camera/examples) for more detailed examples:

- [Basic Example](../../../smashlang_packages/hardware/camera/examples/basic.smash): Demonstrates basic camera usage
- [Video Example](../../../smashlang_packages/hardware/camera/examples/video.smash): Shows video recording
- [Filters Example](../../../smashlang_packages/hardware/camera/examples/filters.smash): Demonstrates image filters
- [Detection Example](../../../smashlang_packages/hardware/camera/examples/detection.smash): Shows face and object detection

## Platform Support

| Feature | Windows | macOS | Linux | Android | iOS | Web |
|---------|---------|-------|-------|---------|-----|-----|
| Device Enumeration | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Photo Capture | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Video Recording | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Face Detection | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Object Detection | ✅ | ✅ | ✅ | ✅ | ✅ | ⚠️ |
| Barcode Scanning | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Advanced Settings | ✅ | ✅ | ⚠️ | ✅ | ✅ | ⚠️ |

✅ = Fully supported, ⚠️ = Limited support

## Testing

The Camera package includes comprehensive tests:

```bash
# Run all tests for the camera package
smashtest smashlang_packages/hardware/camera/tests
```

## Contributing

Contributions to the Camera package are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for your changes
5. Submit a pull request

## License

MIT