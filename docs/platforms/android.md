# SmashLang on Android

SmashLang provides first-class support for Android development, allowing you to write Android applications using SmashLang code that compiles to native Android binaries.

## Requirements

- Android SDK 21+ (Android 5.0 Lollipop or higher)
- Android NDK r21+ for native code compilation
- Gradle 6.5+ for build system integration

## Installation

To set up SmashLang for Android development:

```bash
# Install the Android target
smashpkg install target android

# Verify installation
smashpkg list targets
```

## Creating an Android Project

```bash
# Create a new Android project
smash new android MyAndroidApp

# Navigate to the project directory
cd MyAndroidApp

# Build the project
./gradlew build
```

## Project Structure

A typical SmashLang Android project has the following structure:

```
MyAndroidApp/
├── app/
│   ├── src/
│   │   ├── main/
│   │   │   ├── smash/           # SmashLang source code
│   │   │   │   └── main.smash   # Entry point
│   │   │   ├── res/             # Android resources
│   │   │   └── AndroidManifest.xml
│   │   └── test/
│   │       └── smash/           # SmashLang tests
│   └── build.gradle
├── gradle/
├── build.gradle
└── settings.gradle
```

## Example: Hello World

Here's a simple "Hello World" example for Android:

```javascript
// app/src/main/smash/main.smash
import android.app.Activity;
import android.os.Bundle;
import android.widget.TextView;

class MainActivity extends Activity {
  onCreate(savedInstanceState) {
    super.onCreate(savedInstanceState);
    
    const textView = new TextView(this);
    textView.setText("Hello, SmashLang on Android!");
    
    this.setContentView(textView);
  }
}
```

## Native API Access

SmashLang provides direct access to Android's native APIs:

```javascript
// Access Android APIs
import android.content.Context;
import android.widget.Toast;

function showToast(context, message) {
  Toast.makeText(context, message, Toast.LENGTH_SHORT).show();
}
```

## Hardware Access

SmashLang can access Android device hardware:

```javascript
import android.hardware.Sensor;
import android.hardware.SensorEvent;
import android.hardware.SensorEventListener;
import android.hardware.SensorManager;

class AccelerometerListener implements SensorEventListener {
  onSensorChanged(event) {
    if (event.sensor.getType() === Sensor.TYPE_ACCELEROMETER) {
      const x = event.values[0];
      const y = event.values[1];
      const z = event.values[2];
      
      console.log(`Accelerometer: x=${x}, y=${y}, z=${z}`);
    }
  }
  
  onAccuracyChanged(sensor, accuracy) {
    // Handle accuracy changes
  }
}
```

## Building and Deploying

```bash
# Build debug APK
./gradlew assembleDebug

# Install on connected device
./gradlew installDebug

# Build release APK
./gradlew assembleRelease
```

## Performance Considerations

- SmashLang on Android compiles to native code, providing performance comparable to Java/Kotlin
- The SmashLang runtime is optimized for mobile devices with limited resources
- Use the `--optimize` flag for production builds to enable additional optimizations

## Debugging

SmashLang integrates with Android Studio for debugging:

```bash
# Generate Android Studio project files
smash android studio-project

# Open in Android Studio
open -a "Android Studio" .
```

## Resources

- [Android API Reference](https://developer.android.com/reference)
- [SmashLang Android Examples](../examples/android/)
- [Android Performance Guide](../performance/android.md)