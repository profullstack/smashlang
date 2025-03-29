# Your First SmashLang Program

This guide will walk you through creating and running your first SmashLang program. By the end, you'll have a solid understanding of how to write, compile, and execute SmashLang code.

## Prerequisites

Before you begin, make sure you have:

- SmashLang installed on your system (see the [Installation Guide](./installation.md))
- A text editor of your choice
- A terminal or command prompt

## Creating a Hello World Program

Let's start with the classic "Hello, World!" program. Create a new file named `hello.smash` and open it in your text editor.

Add the following code to the file:

```js
// hello.smash - My first SmashLang program

print("Hello, World!");
```

This simple program uses the built-in `print` function to display a message on the screen.

## Running Your Program

There are two ways to run your SmashLang program:

### Method 1: Using the SmashLang Compiler

Open your terminal and navigate to the directory containing your `hello.smash` file. Then run:

```bash
# Compile the program
smashc hello.smash -o hello

# Run the compiled executable
./hello
```

You should see the output:

```
Hello, World!
```

### Method 2: Using the SmashLang Interpreter

For quick development and testing, you can use the SmashLang interpreter to run your code directly:

```bash
smash hello.smash
```

This will execute your program without creating a separate executable file.

## Understanding the Program

Let's break down our simple program:

- The first line is a comment. In SmashLang, comments start with `//` and continue to the end of the line.
- The second line calls the built-in `print` function, which outputs text to the console.

## Adding More Features

Now let's enhance our program to make it more interactive. Update your `hello.smash` file with the following code:

```js
// hello.smash - An interactive greeting program

// Define a function that creates a personalized greeting
fn greet(name) {
  if (!name || name.trim() === "") {
    return "Hello, World!";
  }
  return `Hello, ${name}!`;
}

// Get the name from command line arguments or use a default
let args = process.argv.slice(2);
let name = args[0] || "";

// Display the greeting
print(greet(name));
```

This enhanced program:

1. Defines a `greet` function that takes a name parameter
2. Checks if a name was provided and creates an appropriate greeting
3. Retrieves command-line arguments using `process.argv`
4. Calls the `greet` function with the provided name

## Running the Enhanced Program

You can run this program the same way as before:

```bash
# Using the interpreter
smash hello.smash

# Or compile and run
smashc hello.smash -o hello
./hello
```

To provide a name as a command-line argument:

```bash
smash hello.smash Alice
```

Output:

```
Hello, Alice!
```

## Creating a More Complex Program

Let's create a slightly more complex program that demonstrates some of SmashLang's features. Create a new file named `calculator.smash` with the following code:

```js
// calculator.smash - A simple calculator program

// Define calculator functions
fn add(a, b) { return a + b; }
fn subtract(a, b) { return a - b; }
fn multiply(a, b) { return a * b; }
fn divide(a, b) {
  if (b === 0) {
    throw new Error("Division by zero");
  }
  return a / b;
}

// Process command line arguments
let args = process.argv.slice(2);

if (args.length < 3) {
  print("Usage: calculator.smash <number> <operation> <number>");
  print("Operations: add, subtract, multiply, divide");
  process.exit(1);
}

// Parse arguments
let num1 = parseFloat(args[0]);
let operation = args[1];
let num2 = parseFloat(args[2]);

// Check if inputs are valid numbers
if (isNaN(num1) || isNaN(num2)) {
  print("Error: Inputs must be valid numbers");
  process.exit(1);
}

// Perform the calculation
let result;
try {
  switch (operation) {
    case "add":
      result = add(num1, num2);
      break;
    case "subtract":
      result = subtract(num1, num2);
      break;
    case "multiply":
      result = multiply(num1, num2);
      break;
    case "divide":
      result = divide(num1, num2);
      break;
    default:
      print("Error: Unknown operation");
      print("Supported operations: add, subtract, multiply, divide");
      process.exit(1);
  }
  
  print(`${num1} ${operation} ${num2} = ${result}`);
} catch (error) {
  print(`Error: ${error.message}`);
  process.exit(1);
}
```

This calculator program demonstrates:

- Function definitions
- Error handling with try-catch
- Control flow with switch statements
- Command-line argument processing
- String interpolation

## Running the Calculator

Run the calculator program with three arguments: a number, an operation, and another number:

```bash
smash calculator.smash 5 add 3
```

Output:

```
5 add 3 = 8
```

Try other operations:

```bash
smash calculator.smash 10 multiply 4
```

Output:

```
10 multiply 4 = 40
```

## Next Steps

Now that you've created your first SmashLang programs, you can explore more advanced features:

- Learn how to use the [SmashLang REPL](./repl.md) for interactive development
- Explore [Language Basics](./language-basics.md) for more details on syntax and features
- Check out the [Standard Library](../standard-library/overview.md) to see what built-in functionality is available
- Learn about [Modules and Imports](../language/modules.md) to organize your code

Happy coding with SmashLang!
