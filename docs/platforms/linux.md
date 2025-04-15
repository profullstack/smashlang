# SmashLang on Linux

SmashLang provides robust support for Linux development, allowing you to create native Linux applications, command-line tools, system services, and desktop applications using SmashLang code.

## Requirements

- Modern Linux distribution (Ubuntu 20.04+, Fedora 34+, Debian 11+, etc.)
- GCC 9+ or Clang 10+
- Development packages (build-essential, libgtk-3-dev for GUI applications)

## Installation

To set up SmashLang for Linux development:

```bash
# Install the Linux target
smashpkg install target linux

# Verify installation
smashpkg list targets
```

## Creating a Linux Project

```bash
# Create a new Linux application
smash new linux MyLinuxApp

# Navigate to the project directory
cd MyLinuxApp

# Build the project
smash build
```

## Project Structure

A typical SmashLang Linux project has the following structure:

```
MyLinuxApp/
├── src/
│   ├── main.smash       # Entry point
│   ├── app.smash        # Application logic
│   └── ui/              # UI components
├── assets/              # Images, icons, etc.
├── tests/               # Test files
├── smash.toml           # Project configuration
└── Makefile             # Build configuration
```

## Example: Hello World Console Application

Here's a simple "Hello World" console example for Linux:

```javascript
// src/main.smash
function main() {
  console.log("Hello, SmashLang on Linux!");
  return 0;
}
```

## Example: GTK GUI Application

Here's a simple GTK GUI example:

```javascript
// src/main.smash
import gtk;

function main() {
  // Initialize GTK
  gtk.init(null, null);
  
  // Create a window
  const window = new gtk.Window({
    type: gtk.WindowType.TOPLEVEL
  });
  
  // Set window properties
  window.setTitle("Hello SmashLang");
  window.setDefaultSize(400, 300);
  window.setPosition(gtk.WindowPosition.CENTER);
  
  // Connect the destroy signal
  window.connect("destroy", () => {
    gtk.mainQuit();
  });
  
  // Create a label
  const label = new gtk.Label({
    label: "Hello, SmashLang on Linux!"
  });
  
  // Add the label to the window
  window.add(label);
  
  // Show all widgets
  window.showAll();
  
  // Start the GTK main loop
  gtk.main();
  
  return 0;
}
```

## Example: Qt GUI Application

SmashLang also supports Qt for cross-platform GUI development:

```javascript
// src/main.smash
import qt.widgets;
import qt.core;

class MainWindow extends qt.widgets.QMainWindow {
  constructor() {
    super();
    
    // Set window properties
    this.setWindowTitle("Hello SmashLang");
    this.resize(400, 300);
    
    // Create central widget
    const centralWidget = new qt.widgets.QWidget();
    this.setCentralWidget(centralWidget);
    
    // Create layout
    const layout = new qt.widgets.QVBoxLayout();
    centralWidget.setLayout(layout);
    
    // Create label
    const label = new qt.widgets.QLabel("Hello, SmashLang on Linux!");
    label.setAlignment(qt.core.Qt.AlignCenter);
    layout.addWidget(label);
    
    // Create button
    const button = new qt.widgets.QPushButton("Quit");
    layout.addWidget(button);
    
    // Connect button signal
    button.clicked.connect(() => {
      qt.widgets.QApplication.quit();
    });
  }
}

function main() {
  // Create application
  const app = new qt.widgets.QApplication([]);
  
  // Create and show main window
  const mainWindow = new MainWindow();
  mainWindow.show();
  
  // Start the Qt event loop
  return app.exec();
}
```

## System Integration

SmashLang provides access to Linux system APIs:

```javascript
import linux.system;
import linux.process;

function getSystemInfo() {
  // Get system information
  const info = linux.system.uname();
  
  console.log(`System: ${info.sysname}`);
  console.log(`Node: ${info.nodename}`);
  console.log(`Release: ${info.release}`);
  console.log(`Version: ${info.version}`);
  console.log(`Machine: ${info.machine}`);
  
  // Get process information
  const pid = linux.process.getpid();
  const ppid = linux.process.getppid();
  
  console.log(`PID: ${pid}`);
  console.log(`Parent PID: ${ppid}`);
  
  return info;
}
```

## File System Operations

```javascript
import fs;

function fileOperations() {
  // Write to a file
  fs.writeFileSync("test.txt", "Hello, SmashLang!");
  
  // Read from a file
  const content = fs.readFileSync("test.txt", "utf8");
  console.log(`File content: ${content}`);
  
  // Get file information
  const stats = fs.statSync("test.txt");
  console.log(`File size: ${stats.size} bytes`);
  console.log(`Created: ${stats.birthtime}`);
  console.log(`Modified: ${stats.mtime}`);
  
  // Delete the file
  fs.unlinkSync("test.txt");
}
```

## Network Operations

```javascript
import net;
import http;

async function fetchData(url) {
  try {
    const response = await http.get(url);
    console.log(`Status: ${response.statusCode}`);
    console.log(`Data: ${response.data}`);
    return response.data;
  } catch (error) {
    console.error(`Error: ${error.message}`);
    throw error;
  }
}

function createServer() {
  const server = http.createServer((req, res) => {
    res.writeHead(200, { "Content-Type": "text/plain" });
    res.end("Hello from SmashLang HTTP server!");
  });
  
  server.listen(3000, "127.0.0.1", () => {
    console.log("Server running at http://127.0.0.1:3000/");
  });
  
  return server;
}
```

## Building and Packaging

```bash
# Build debug version
smash build --debug

# Build release version
smash build --release

# Create Debian package
smash package --deb

# Create RPM package
smash package --rpm

# Create AppImage
smash package --appimage

# Create Snap package
smash package --snap
```

## Systemd Service Integration

SmashLang can create systemd services:

```javascript
// src/service.smash
import linux.systemd;

class MyService extends systemd.Service {
  constructor() {
    super("myservice");
  }
  
  onStart() {
    console.log("Service started");
    // Service initialization code
    return true;
  }
  
  onStop() {
    console.log("Service stopped");
    // Cleanup code
    return true;
  }
  
  onReload() {
    console.log("Service reloaded");
    // Reload configuration
    return true;
  }
}

function main() {
  const service = new MyService();
  return service.run();
}
```

## Performance Considerations

- SmashLang on Linux compiles to native code for optimal performance
- Use the `--optimize` flag for production builds to enable additional optimizations
- For GUI applications, use hardware acceleration when available

## Debugging

SmashLang integrates with GDB and LLDB for debugging:

```bash
# Debug with GDB
smash debug --gdb

# Debug with LLDB
smash debug --lldb
```

## Resources

- [Linux API Reference](../api/linux/index.md)
- [SmashLang Linux Examples](../examples/linux/)
- [Linux Performance Guide](../performance/linux.md)
- [Linux Packaging Guide](../packaging/linux.md)