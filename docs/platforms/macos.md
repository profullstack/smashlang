# SmashLang on macOS

SmashLang provides comprehensive support for macOS development, allowing you to create native macOS applications, command-line tools, and system services using SmashLang code.

## Requirements

- macOS 10.15+ (Catalina or higher)
- Xcode 12.0+ (for GUI applications and App Store distribution)
- Command Line Tools for Xcode

## Installation

To set up SmashLang for macOS development:

```bash
# Install the macOS target
smashpkg install target macos

# Verify installation
smashpkg list targets
```

## Creating a macOS Project

```bash
# Create a new macOS application
smash new macos MyMacApp

# Navigate to the project directory
cd MyMacApp

# Build the project
smash build
```

## Project Structure

A typical SmashLang macOS project has the following structure:

```
MyMacApp/
├── Sources/
│   ├── AppDelegate.smash
│   ├── ViewController.smash
│   └── Main.smash
├── Resources/
│   ├── Assets.xcassets/
│   ├── Main.storyboard
│   └── Info.plist
├── Tests/
│   └── MyMacAppTests.smash
└── MyMacApp.xcodeproj/
```

## Example: Hello World GUI Application

Here's a simple "Hello World" GUI example for macOS:

```javascript
// Sources/AppDelegate.smash
import Cocoa;

class AppDelegate extends NSObject {
  constructor() {
    super();
    this.window = null;
  }
  
  applicationDidFinishLaunching(notification) {
    // Create window
    this.window = new NSWindow({
      contentRect: NSMakeRect(0, 0, 800, 600),
      styleMask: NSWindowStyleMaskTitled | 
                NSWindowStyleMaskClosable | 
                NSWindowStyleMaskMiniaturizable | 
                NSWindowStyleMaskResizable,
      backing: NSBackingStoreBuffered,
      defer: false
    });
    
    // Configure window
    this.window.title = "Hello SmashLang";
    this.window.center();
    
    // Create label
    const label = new NSTextField({
      frame: NSMakeRect(0, 0, 200, 30)
    });
    label.stringValue = "Hello, SmashLang on macOS!";
    label.bezeled = false;
    label.drawsBackground = false;
    label.editable = false;
    label.selectable = false;
    
    // Center label in window
    label.frame = {
      x: (this.window.frame.size.width - label.frame.size.width) / 2,
      y: (this.window.frame.size.height - label.frame.size.height) / 2,
      width: label.frame.size.width,
      height: label.frame.size.height
    };
    
    // Add label to window
    this.window.contentView.addSubview(label);
    
    // Show window
    this.window.makeKeyAndOrderFront(null);
  }
  
  applicationShouldTerminateAfterLastWindowClosed(sender) {
    return true;
  }
}

// Application entry point
function main() {
  const app = NSApplication.sharedApplication();
  const delegate = new AppDelegate();
  app.delegate = delegate;
  app.run();
}
```

## Example: Command-Line Tool

```javascript
// Sources/Main.smash
import Foundation;

function main(args) {
  console.log("Hello, SmashLang on macOS!");
  
  if (args.length > 1) {
    console.log(`Arguments: ${args.slice(1).join(", ")}`);
  } else {
    console.log("No arguments provided.");
  }
  
  return 0;
}
```

## macOS API Access

SmashLang provides direct access to macOS APIs:

```javascript
import Cocoa;
import Foundation;

function getSystemInfo() {
  const processInfo = NSProcessInfo.processInfo();
  
  console.log(`macOS Version: ${processInfo.operatingSystemVersionString}`);
  console.log(`Host Name: ${processInfo.hostName}`);
  console.log(`Processors: ${processInfo.processorCount}`);
  console.log(`Physical Memory: ${processInfo.physicalMemory / (1024 * 1024 * 1024)} GB`);
  
  return processInfo;
}
```

## SwiftUI Support

SmashLang supports SwiftUI for modern macOS development:

```javascript
import SwiftUI;

class ContentView extends View {
  body() {
    return VStack.init([
      Text.init("Hello, SmashLang!")
        .font(.title)
        .padding(),
      
      Button.init("Quit", () => {
        NSApplication.sharedApplication().terminate(null);
      })
      .padding()
    ])
    .frame(width: 300, height: 200)
    .padding();
  }
}

function main() {
  const app = NSApplication.sharedApplication();
  app.setActivationPolicy(NSApplicationActivationPolicyRegular);
  
  const contentView = new ContentView();
  const window = NSWindow.init({
    contentRect: NSMakeRect(0, 0, 300, 200),
    styleMask: NSWindowStyleMaskTitled | 
              NSWindowStyleMaskClosable | 
              NSWindowStyleMaskMiniaturizable,
    backing: NSBackingStoreBuffered,
    defer: false
  });
  
  window.title = "SwiftUI Example";
  window.contentView = NSHostingView.init(contentView);
  window.center();
  window.makeKeyAndOrderFront(null);
  
  app.activate();
  app.run();
}
```

## Building and Packaging

```bash
# Build debug version
smash build --debug

# Build release version
smash build --release

# Create application bundle
smash package --app

# Create disk image (DMG)
smash package --dmg

# Create installer package
smash package --pkg
```

## App Store Distribution

```bash
# Prepare for App Store submission
smash package --app-store

# Validate the app
xcrun altool --validate-app -f MyMacApp.pkg -t osx -u "your@apple.id"

# Upload to App Store Connect
xcrun altool --upload-app -f MyMacApp.pkg -t osx -u "your@apple.id"
```

## Performance Considerations

- SmashLang on macOS compiles to native code for optimal performance
- Use the `--optimize` flag for production builds to enable additional optimizations
- For GUI applications, use hardware acceleration when available

## Debugging

SmashLang integrates with Xcode for debugging:

```bash
# Generate Xcode project files
smash macos xcode-project

# Open in Xcode
open MyMacApp.xcodeproj
```

## Resources

- [macOS API Reference](https://developer.apple.com/documentation/macos)
- [SmashLang macOS Examples](../examples/macos/)
- [macOS Performance Guide](../performance/macos.md)