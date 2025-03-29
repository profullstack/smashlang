# SmashLang Standard Library: Process API

The SmashLang Standard Library (`std`) provides a comprehensive set of functions and properties for interacting with the operating system process and environment. This document outlines the process-related functionality available in the `std` module.

## Table of Contents

- [Environment Variables](#environment-variables)
- [Command Line Arguments](#command-line-arguments)
- [Working Directory](#working-directory)
- [Platform Information](#platform-information)
- [Process Control](#process-control)
- [Process Information](#process-information)
- [Resource Usage](#resource-usage)
- [Event Handling](#event-handling)
- [Examples](#examples)

## Environment Variables

Access and modify environment variables using the `std.env` object.

```smash
// Access common environment variables directly
const home = std.env.HOME;
const user = std.env.USER;
const path = std.env.PATH;
const shell = std.env.SHELL;

// Get any environment variable
const customVar = std.env.get("MY_CUSTOM_VAR");

// Set an environment variable
std.env.set("MY_CUSTOM_VAR", "some value");
```

## Command Line Arguments

Access command line arguments using the `std.argv` array.

```smash
// std.argv[0] is the path to the SmashLang interpreter
// std.argv[1] is the path to the script being executed
// std.argv[2] and beyond are the arguments passed to the script

// Get all arguments passed to the script
const args = std.argv.slice(2);

// Check for a specific flag
const verbose = args.includes("--verbose");

// Get a value for an option
const index = args.indexOf("--output");
const outputFile = index >= 0 ? args[index + 1] : null;
```

## Working Directory

Manage the current working directory.

```smash
// Get the current working directory
const currentDir = std.cwd();

// Change the current working directory
std.chdir("/path/to/directory");
```

## Platform Information

Access information about the operating system and architecture.

```smash
// Get the operating system platform
// Returns 'linux', 'darwin' (macOS), 'win32', etc.
const platform = std.platform;

// Get the CPU architecture
// Returns 'x64', 'arm64', etc.
const architecture = std.arch;

// Platform-specific code
if (std.platform === "linux") {
  // Linux-specific code
} else if (std.platform === "darwin") {
  // macOS-specific code
} else if (std.platform === "win32") {
  // Windows-specific code
}
```

## Process Control

Control the current process.

```smash
// Exit the process with a success code (0)
std.exit();

// Exit the process with a specific code
std.exit(1); // Exit with error code 1
```

## Process Information

Access information about the current process.

```smash
// Get the process ID (PID)
const pid = std.pid;

// Get the parent process ID (PPID)
const ppid = std.ppid;
```

## Resource Usage

Monitor resource usage of the current process.

```smash
// Get memory usage information
const memory = std.memoryUsage();
print(`RSS: ${memory.rss} bytes`);
print(`Heap Total: ${memory.heapTotal} bytes`);
print(`Heap Used: ${memory.heapUsed} bytes`);

// Get CPU usage information
const cpu = std.cpuUsage();
print(`User CPU time: ${cpu.user} microseconds`);
print(`System CPU time: ${cpu.system} microseconds`);

// Get high-resolution time measurement
const time = std.hrtime();
print(`Seconds: ${time.seconds}`);
print(`Nanoseconds: ${time.nanoseconds}`);
print(`Milliseconds: ${time.milliseconds}`);
```

## Event Handling

Handle process events.

```smash
// Handle process exit
std.on("exit", (code) => {
  print(`Process is exiting with code: ${code}`);
});

// Handle uncaught exceptions
std.on("uncaughtException", (error) => {
  print(`Uncaught exception: ${error}`);
});

// Handle unhandled promise rejections
std.on("unhandledRejection", (reason) => {
  print(`Unhandled promise rejection: ${reason}`);
});

// Remove an event listener
std.removeListener("exit", myExitHandler);
```

## Examples

### Basic System Information

```smash
import "std";

fn main() {
  print("System Information:");
  print(`Platform: ${std.platform}`);
  print(`Architecture: ${std.arch}`);
  print(`Process ID: ${std.pid}`);
  print(`Current Directory: ${std.cwd()}`);
  print(`User Home: ${std.env.HOME}`);
}

main();
```

### Command Line Argument Parser

```smash
import "std";

fn parseArgs() {
  const args = std.argv.slice(2);
  const result = {
    flags: {},
    options: {},
    positional: []
  };
  
  for (let i = 0; i < args.length; i++) {
    const arg = args[i];
    
    if (arg.startsWith("--")) {
      // Handle --option=value format
      const equalPos = arg.indexOf("=");
      if (equalPos > 0) {
        const key = arg.substring(2, equalPos);
        const value = arg.substring(equalPos + 1);
        result.options[key] = value;
      } else {
        // Handle --flag format
        const flag = arg.substring(2);
        if (i + 1 < args.length && !args[i + 1].startsWith("-")) {
          result.options[flag] = args[i + 1];
          i++;
        } else {
          result.flags[flag] = true;
        }
      }
    } else if (arg.startsWith("-")) {
      // Handle -f format (short flags)
      const flags = arg.substring(1).split("");
      for (const flag of flags) {
        result.flags[flag] = true;
      }
    } else {
      // Handle positional arguments
      result.positional.push(arg);
    }
  }
  
  return result;
}

fn main() {
  const args = parseArgs();
  print("Parsed Arguments:");
  print("Flags:", JSON.stringify(args.flags));
  print("Options:", JSON.stringify(args.options));
  print("Positional:", JSON.stringify(args.positional));
}

main();
```

### Resource Monitoring

```smash
import "std";

fn formatBytes(bytes) {
  if (bytes === 0) return "0 Bytes";
  const k = 1024;
  const sizes = ["Bytes", "KB", "MB", "GB", "TB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
}

fn monitorResources() {
  setInterval(() => {
    const memory = std.memoryUsage();
    print("Memory Usage:");
    print(`RSS: ${formatBytes(memory.rss)}`);
    print(`Heap Used: ${formatBytes(memory.heapUsed)}/${formatBytes(memory.heapTotal)}`);
    
    const cpu = std.cpuUsage();
    print("CPU Usage:");
    print(`User: ${cpu.user / 1000}ms, System: ${cpu.system / 1000}ms`);
  }, 1000);
}

fn main() {
  print("Starting resource monitoring...");
  monitorResources();
}

main();
```

For more examples, see the [process_example.smash](../examples/process_example.smash) file in the examples directory.
