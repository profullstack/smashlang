# SmashLang on iOS

SmashLang provides comprehensive support for iOS development, allowing you to write iOS applications using SmashLang code that compiles to native iOS binaries.

## Requirements

- macOS 10.15+ (Catalina or higher) for development
- Xcode 12.0+ with iOS SDK
- iOS 13.0+ target devices
- Apple Developer account (for App Store distribution)

## Installation

To set up SmashLang for iOS development:

```bash
# Install the iOS target
smashpkg install target ios

# Verify installation
smashpkg list targets
```

## Creating an iOS Project

```bash
# Create a new iOS project
smash new ios MyiOSApp

# Navigate to the project directory
cd MyiOSApp

# Build the project
xcodebuild -scheme MyiOSApp -sdk iphonesimulator
```

## Project Structure

A typical SmashLang iOS project has the following structure:

```
MyiOSApp/
├── Sources/
│   ├── AppDelegate.smash
│   ├── SceneDelegate.smash
│   ├── ViewController.smash
│   └── Main.smash
├── Resources/
│   ├── Assets.xcassets/
│   ├── LaunchScreen.storyboard
│   └── Info.plist
├── Tests/
│   └── MyiOSAppTests.smash
└── MyiOSApp.xcodeproj/
```

## Example: Hello World

Here's a simple "Hello World" example for iOS:

```javascript
// Sources/ViewController.smash
import UIKit;

class ViewController extends UIViewController {
  viewDidLoad() {
    super.viewDidLoad();
    
    // Create a label
    const label = new UILabel();
    label.frame = { x: 0, y: 0, width: this.view.frame.width, height: 100 };
    label.center = { x: this.view.center.x, y: this.view.center.y };
    label.textAlignment = NSTextAlignmentCenter;
    label.text = "Hello, SmashLang on iOS!";
    label.font = UIFont.systemFontOfSize(24);
    
    // Add to view
    this.view.addSubview(label);
    this.view.backgroundColor = UIColor.whiteColor();
  }
}
```

## Native API Access

SmashLang provides direct access to iOS's native APIs:

```javascript
// Access iOS APIs
import UIKit;

function showAlert(viewController, title, message) {
  const alert = UIAlertController.alertControllerWithTitle_message_preferredStyle(
    title,
    message,
    UIAlertControllerStyleAlert
  );
  
  alert.addAction(UIAlertAction.actionWithTitle_style_handler(
    "OK",
    UIAlertActionStyleDefault,
    null
  ));
  
  viewController.presentViewController_animated_completion(alert, true, null);
}
```

## Hardware Access

SmashLang can access iOS device hardware:

```javascript
import CoreMotion;

class MotionManager {
  constructor() {
    this.motionManager = new CMMotionManager();
  }
  
  startAccelerometerUpdates(interval = 0.1) {
    if (this.motionManager.accelerometerAvailable) {
      this.motionManager.accelerometerUpdateInterval = interval;
      
      this.motionManager.startAccelerometerUpdatesToQueue_withHandler(
        NSOperationQueue.mainQueue(),
        (data, error) => {
          if (data) {
            const x = data.acceleration.x;
            const y = data.acceleration.y;
            const z = data.acceleration.z;
            
            console.log(`Accelerometer: x=${x}, y=${y}, z=${z}`);
          }
        }
      );
    }
  }
  
  stopAccelerometerUpdates() {
    this.motionManager.stopAccelerometerUpdates();
  }
}
```

## SwiftUI Support

SmashLang supports SwiftUI for modern iOS development:

```javascript
import SwiftUI;

class ContentView extends View {
  body() {
    return VStack.init([
      Text.init("Hello, SmashLang!")
        .font(.title)
        .padding(),
      
      Button.init("Tap Me", () => {
        console.log("Button tapped!");
      })
      .padding()
    ])
    .padding();
  }
}
```

## Building and Deploying

```bash
# Build for simulator
smash build ios --simulator

# Build for device
smash build ios --device

# Archive for App Store
smash archive ios
```

## Performance Considerations

- SmashLang on iOS compiles to native code, providing performance comparable to Swift/Objective-C
- The SmashLang runtime is optimized for mobile devices with limited resources
- Use the `--optimize` flag for production builds to enable additional optimizations

## Debugging

SmashLang integrates with Xcode for debugging:

```bash
# Generate Xcode project files
smash ios xcode-project

# Open in Xcode
open MyiOSApp.xcodeproj
```

## Resources

- [iOS API Reference](https://developer.apple.com/documentation)
- [SmashLang iOS Examples](../examples/ios/)
- [iOS Performance Guide](../performance/ios.md)