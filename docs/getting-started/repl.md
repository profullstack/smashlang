# Using the SmashLang REPL

The SmashLang REPL (Read-Eval-Print Loop) is an interactive environment where you can write and execute SmashLang code line by line. It's a powerful tool for learning the language, testing ideas, and debugging code snippets without creating full programs.

## Starting the REPL

To start the SmashLang REPL, open your terminal and run:

```bash
smash
```

You should see a welcome message and a prompt, indicating that the REPL is ready to accept your input:

```
SmashLang v1.0.0
Type .help for available commands
> 
```

## Basic Usage

At the REPL prompt (`>`), you can type SmashLang expressions and statements. After pressing Enter, the REPL will evaluate your input and display the result:

```
> 2 + 3
5

> "Hello, " + "World!"
"Hello, World!"

> let x = 10
10

> x * 2
20
```

## Multi-line Input

For multi-line statements like function definitions or loops, the REPL will automatically detect incomplete input and provide a continuation prompt (`...`) until you complete the statement:

```
> fn greet(name) {
... return `Hello, ${name}!`;
... }
undefined

> greet("Alice")
"Hello, Alice!"
```

You can also use Shift+Enter to create a new line without executing the code.

## REPL Special Commands

The REPL supports special commands that begin with a dot (`.`). Here are some useful commands:

### .help

Displays a list of available REPL commands:

```
> .help
Available commands:
  .clear       Clear the REPL context
  .exit        Exit the REPL
  .help        Show this help message
  .load        Load a file into the REPL session
  .save        Save the REPL session to a file
  .history     Show command history
  .vars        List defined variables
  .functions   List defined functions
```

### .clear

Clears the REPL context, removing all defined variables and functions:

```
> let x = 10
10

> .clear
REPL context cleared

> x
ReferenceError: x is not defined
```

### .exit

Exits the REPL and returns to the terminal:

```
> .exit
```

You can also use Ctrl+C twice or Ctrl+D to exit the REPL.

### .load

Loads and executes a SmashLang file in the current REPL session:

```
> .load path/to/my-script.smash
Loaded path/to/my-script.smash
```

### .save

Saves the commands entered in the current REPL session to a file:

```
> .save path/to/session.smash
Session saved to path/to/session.smash
```

### .history

Displays a list of commands entered in the current session:

```
> .history
1: 2 + 3
2: let x = 10
3: x * 2
```

### .vars

Lists all variables defined in the current session:

```
> let x = 10
10
> let message = "Hello"
"Hello"
> .vars
x: 10
message: "Hello"
```

### .functions

Lists all functions defined in the current session:

```
> fn add(a, b) { return a + b; }
undefined
> fn greet(name) { return `Hello, ${name}!`; }
undefined
> .functions
add(a, b)
greet(name)
```

## Working with Objects and Arrays

You can create and manipulate complex data structures in the REPL:

```
> let person = { name: "Alice", age: 30 }
{ name: "Alice", age: 30 }

> person.name
"Alice"

> person.job = "Developer"
"Developer"

> person
{ name: "Alice", age: 30, job: "Developer" }

> let numbers = [1, 2, 3, 4, 5]
[1, 2, 3, 4, 5]

> numbers.push(6)
6

> numbers
[1, 2, 3, 4, 5, 6]
```

## Using Destructuring

You can use destructuring assignments in the REPL:

```
> let { name, age } = person
undefined

> name
"Alice"

> let [first, second, ...rest] = numbers
undefined

> first
1

> rest
[3, 4, 5, 6]
```

## Importing Modules

You can import modules in the REPL:

```
> import { readFile } from "fs/promises"
undefined

> async function readMyFile() { return await readFile("example.txt", "utf8"); }
undefined

> readMyFile().then(content => console.log(content))
Promise { <pending> }
// File content will be displayed when the promise resolves
```

## Async/Await in the REPL

The REPL supports async/await for working with promises:

```
> let response = await fetch("https://api.example.com/data")
// Response object

> let data = await response.json()
// JSON data
```

## Error Handling

When you make a mistake, the REPL will display an error message but won't terminate the session:

```
> 10 / 0
Error: Division by zero

> undefinedVariable
ReferenceError: undefinedVariable is not defined

> JSON.parse("invalid")
SyntaxError: Unexpected token 'i', "invalid" is not valid JSON
```

## REPL Configuration

You can customize the REPL behavior by creating a `.smashrc` file in your home directory. This file can contain SmashLang code that will be executed when the REPL starts.

Example `.smashrc` file:

```js
// Custom REPL configuration

// Define utility functions
fn sayHello() {
  print("Hello from SmashLang REPL!");
}

// Set default variables
let defaultTimeout = 1000;

// Custom REPL prompt
repl.setPrompt("smash> ");

// Display a welcome message
print("Welcome to your customized SmashLang REPL!");
```

## Tips and Tricks

1. **Tab Completion**: Press Tab to autocomplete variable names, function names, and properties.

2. **Arrow Keys**: Use the up and down arrow keys to navigate through command history.

3. **Multiline Editing**: For complex code blocks, consider writing them in a text editor first, then loading the file with `.load`.

4. **Inspecting Objects**: Use `console.dir(obj)` for a more detailed view of complex objects.

5. **Performance Testing**: Use `console.time()` and `console.timeEnd()` to measure execution time:

   ```
   > console.time("loop")
   undefined
   > for (let i = 0; i < 1000000; i++) {}
   undefined
   > console.timeEnd("loop")
   loop: 5.123ms
   ```

## Next Steps

Now that you're familiar with the SmashLang REPL, you can:

- Explore the [Language Basics](./language-basics.md) to learn more about SmashLang syntax
- Create your [First SmashLang Program](./first-program.md) in a file
- Check out the [Standard Library](../standard-library/overview.md) documentation
- Learn about [Modules and Imports](../language/modules.md) for organizing your code
