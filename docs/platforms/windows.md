# SmashLang on Windows

SmashLang provides robust support for Windows development, allowing you to create native Windows applications, services, and tools using SmashLang code.

## Requirements

- Windows 10/11 (64-bit recommended)
- Visual Studio 2019+ with C++ workload or MinGW-w64
- Windows SDK 10.0.18362.0 or newer

## Installation

To set up SmashLang for Windows development:

```powershell
# Install the Windows target
smashpkg install target windows

# Verify installation
smashpkg list targets
```

## Creating a Windows Project

```powershell
# Create a new Windows application
smash new windows MyWindowsApp

# Navigate to the project directory
cd MyWindowsApp

# Build the project
smash build
```

## Project Structure

A typical SmashLang Windows project has the following structure:

```
MyWindowsApp/
├── src/
│   ├── main.smash       # Entry point
│   ├── app.smash        # Application logic
│   └── resources.rc     # Windows resources
├── include/             # Header files
├── assets/              # Images, icons, etc.
├── tests/               # Test files
└── smash.toml           # Project configuration
```

## Example: Hello World GUI Application

Here's a simple "Hello World" GUI example for Windows:

```javascript
// src/main.smash
import windows.ui;
import windows.app;

class MainWindow extends Window {
  constructor() {
    super({
      title: "Hello SmashLang",
      width: 800,
      height: 600,
      icon: "assets/icon.ico"
    });
    
    // Create a button
    this.button = new Button({
      parent: this,
      text: "Click Me!",
      x: 350,
      y: 250,
      width: 100,
      height: 30
    });
    
    // Add event handler
    this.button.onClick = () => {
      MessageBox.show({
        parent: this,
        title: "Hello",
        message: "Hello, SmashLang on Windows!",
        type: MessageBoxType.Information
      });
    };
  }
}

// Application entry point
function main() {
  const app = new Application();
  const mainWindow = new MainWindow();
  
  mainWindow.show();
  return app.run();
}
```

## Example: Console Application

```javascript
// src/main.smash
import windows.console;

function main() {
  console.log("Hello, SmashLang on Windows!");
  
  console.log("Enter your name:");
  const name = console.readLine();
  
  console.log(`Hello, ${name}!`);
  
  return 0;
}
```

## Windows API Access

SmashLang provides direct access to the Windows API:

```javascript
import windows.api;

// Use Windows API functions
function getSystemInfo() {
  const info = new SYSTEM_INFO();
  GetSystemInfo(info);
  
  console.log(`Processor architecture: ${info.wProcessorArchitecture}`);
  console.log(`Number of processors: ${info.dwNumberOfProcessors}`);
  console.log(`Page size: ${info.dwPageSize}`);
  
  return info;
}
```

## COM Integration

SmashLang supports COM for interoperability with Windows components:

```javascript
import windows.com;

function createExcelWorkbook() {
  // Initialize COM
  CoInitialize(null);
  
  try {
    // Create Excel application
    const excel = new COMObject("Excel.Application");
    excel.Visible = true;
    
    // Create a new workbook
    const workbook = excel.Workbooks.Add();
    const sheet = workbook.Worksheets(1);
    
    // Add data
    sheet.Cells(1, 1).Value = "Hello";
    sheet.Cells(1, 2).Value = "SmashLang";
    
    return workbook;
  } finally {
    // Uninitialize COM
    CoUninitialize();
  }
}
```

## Windows Services

SmashLang can create Windows services:

```javascript
import windows.service;

class MyService extends WindowsService {
  constructor() {
    super("MyService", "My SmashLang Service");
  }
  
  onStart(args) {
    // Service startup code
    this.logger.info("Service started");
  }
  
  onStop() {
    // Service shutdown code
    this.logger.info("Service stopped");
  }
  
  onPause() {
    this.logger.info("Service paused");
  }
  
  onContinue() {
    this.logger.info("Service continued");
  }
}

// Register and start the service
function main() {
  const service = new MyService();
  return service.run();
}
```

## Building and Packaging

```powershell
# Build debug version
smash build --debug

# Build release version
smash build --release

# Create installer
smash package --installer

# Create portable executable
smash package --portable
```

## Performance Considerations

- SmashLang on Windows compiles to native code for optimal performance
- Use the `--optimize` flag for production builds to enable additional optimizations
- For GUI applications, use hardware acceleration when available

## Debugging

SmashLang integrates with Visual Studio for debugging:

```powershell
# Generate Visual Studio project files
smash windows vs-project

# Open in Visual Studio
start MyWindowsApp.sln
```

## Resources

- [Windows API Reference](https://docs.microsoft.com/en-us/windows/win32/api/)
- [SmashLang Windows Examples](../examples/windows/)
- [Windows Performance Guide](../performance/windows.md)