# Exec

<p align="center">
  <img src="./assets/logo.svg" alt="Exec Logo" width="200" />
</p>

A process execution utility package for SmashLang, providing a Promise-based API for running system commands.

## Installation

```bash
smashpkg install exec
```

## Features

- **Asynchronous Execution**: Run commands asynchronously with Promise support
- **Synchronous Execution**: Run commands synchronously when needed
- **Flexible Options**: Configure working directory, environment variables, timeouts, and more
- **Error Handling**: Comprehensive error handling with detailed error messages
- **Timeout Support**: Set timeouts for long-running commands

## Basic Usage

```js
// Using the default export (simplest approach)
import exec from "stdio/exec";

async function simpleExample() {
  try {
    // Simple, clean syntax
    const output = await exec("ls -la");
    console.log(output);
  } catch (error) {
    console.error("Command failed:", error);
  }
}

// Using named exports
import { exec, execSync } from "stdio/exec";

async function example() {
  try {
    const output = await exec("ls -la");
    console.log(output);
  } catch (error) {
    console.error("Command failed:", error);
  }
}

// Synchronous execution
try {
  const output = execSync("ls -la");
  console.log(output);
} catch (error) {
  console.error("Command failed:", error);
}
```

## API Reference

### default export: exec(command, options)

Default export that executes a command asynchronously and returns a Promise. This provides the simplest and cleanest syntax for command execution.

- **command** (string): The command to execute.
- **options** (object, optional): Configuration options (same as the named exec function).

**Returns**: A Promise that resolves with stdout (string) or rejects with stderr (string).

```js
import exec from "stdio/exec";

// Simple and clean syntax
const output = await exec("ls -la");
```

### exec(command, options)

Named export that executes a command asynchronously and returns a Promise.

- **command** (string): The command to execute.
- **options** (object, optional): Configuration options.
  - **cwd** (string): Current working directory. Default: `process.cwd()`.
  - **env** (object): Environment variables. Default: `process.env`.
  - **timeout** (number): Timeout in milliseconds.
  - **encoding** (string): Output encoding. Default: 'utf8'.
  - **shell** (boolean): Whether to execute the command in a shell. Default: `true`.

**Returns**: A Promise that resolves with stdout (string) or rejects with stderr (string).

### execSync(command, options)

Executes a command synchronously and returns the result.

- **command** (string): The command to execute.
- **options** (object, optional): Configuration options.
  - **cwd** (string): Current working directory. Default: `process.cwd()`.
  - **env** (object): Environment variables. Default: `process.env`.
  - **encoding** (string): Output encoding. Default: 'utf8'.
  - **shell** (boolean): Whether to execute the command in a shell. Default: `true`.

**Returns**: The stdout output (string) from the command.

**Throws**: An Error if the command fails.

## Examples

### Basic Command Execution

```js
import "stdio/exec";

// Asynchronous
async function listFiles() {
  const output = await exec.exec("ls -la");
  console.log(output);
}

// Synchronous
const output = exec.execSync("ls -la");
console.log(output);
```

### Working with Options

```js
import "stdio/exec";

// Set working directory
const output = await exec.exec("ls -la", { cwd: "/home/user/projects" });

// Set environment variables
const buildOutput = await exec.exec("npm run build", {
  env: { ...process.env, NODE_ENV: "production" }
});

// Set timeout
try {
  const result = await exec.exec("long-running-command", { timeout: 5000 });
  console.log(result);
} catch (error) {
  console.error("Command timed out or failed:", error);
}
```

### Error Handling

```js
import "stdio/exec";

try {
  const output = await exec.exec("non-existent-command");
  console.log(output);
} catch (error) {
  console.error("Command failed:", error);
}
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
