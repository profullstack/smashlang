# Destructuring and Spread Operator in SmashLang

SmashLang supports modern JavaScript-like features such as destructuring assignments and the spread operator, making it easier to work with arrays and objects.

## Destructuring Assignment

Destructuring allows you to extract values from arrays or properties from objects into distinct variables using a syntax that mirrors the structure of the array or object.

### Array Destructuring

```js
// Basic array destructuring
let [a, b] = [1, 2];
print(a); // 1
print(b); // 2

// Skipping elements
let [first, , third] = [1, 2, 3];
print(first); // 1
print(third); // 3

// Rest pattern
let [head, ...tail] = [1, 2, 3, 4];
print(head); // 1
print(tail); // [2, 3, 4]

// Default values
let [x = 5, y = 7] = [1];
print(x); // 1 (value from array)
print(y); // 7 (default value)

// Swapping variables
let a = 1;
let b = 2;
[a, b] = [b, a];
print(a); // 2
print(b); // 1
```

### Object Destructuring

```js
// Basic object destructuring
let person = { name: "John", age: 30 };
let { name, age } = person;
print(name); // "John"
print(age); // 30

// Assigning to different variable names
let { name: personName, age: personAge } = person;
print(personName); // "John"
print(personAge); // 30

// Default values
let { name, age, job = "Developer" } = person;
print(job); // "Developer" (default value)

// Nested destructuring
let user = {
  id: 42,
  profile: {
    name: "Alice",
    address: {
      city: "San Francisco",
      country: "USA"
    }
  }
};

let { profile: { name, address: { city } } } = user;
print(name); // "Alice"
print(city); // "San Francisco"

// Rest pattern with objects
let { name, ...rest } = person;
print(name); // "John"
print(rest); // { age: 30 }
```

### Function Parameter Destructuring

Destructuring can also be used in function parameters:

```js
// Array parameter destructuring
fn sum([a, b]) {
  return a + b;
}
print(sum([1, 2])); // 3

// Object parameter destructuring
fn greet({ name, greeting = "Hello" }) {
  return `${greeting}, ${name}!`;
}
print(greet({ name: "John" })); // "Hello, John!"
print(greet({ name: "Alice", greeting: "Hi" })); // "Hi, Alice!"

// Destructuring with rest parameters
fn printPersonInfo({ name, age, ...rest }) {
  print(`Name: ${name}, Age: ${age}`);
  print("Additional info:", rest);
}

printPersonInfo({
  name: "John",
  age: 30,
  job: "Developer",
  city: "New York"
});
// Output:
// Name: John, Age: 30
// Additional info: { job: "Developer", city: "New York" }
```

## Spread Operator

The spread operator (`...`) allows an iterable (like an array or string) to be expanded in places where zero or more arguments or elements are expected, or an object to be expanded in places where zero or more key-value pairs are expected.

### Array Spread

```js
// Combining arrays
let arr1 = [1, 2, 3];
let arr2 = [4, 5, 6];
let combined = [...arr1, ...arr2];
print(combined); // [1, 2, 3, 4, 5, 6]

// Copying arrays
let original = [1, 2, 3];
let copy = [...original];
original[0] = 99;
print(original); // [99, 2, 3]
print(copy); // [1, 2, 3] (not affected by the change to original)

// Using spread with push
let numbers = [1, 2];
numbers.push(...[3, 4, 5]);
print(numbers); // [1, 2, 3, 4, 5]

// Using spread with function calls
fn add(a, b, c) {
  return a + b + c;
}
let args = [1, 2, 3];
print(add(...args)); // 6
```

### Object Spread

```js
// Combining objects
let obj1 = { a: 1, b: 2 };
let obj2 = { c: 3, d: 4 };
let combined = { ...obj1, ...obj2 };
print(combined); // { a: 1, b: 2, c: 3, d: 4 }

// Copying objects
let original = { a: 1, b: 2 };
let copy = { ...original };
original.a = 99;
print(original); // { a: 99, b: 2 }
print(copy); // { a: 1, b: 2 } (not affected by the change to original)

// Overriding properties
let defaults = { theme: "dark", fontSize: 12, showSidebar: true };
let userPrefs = { theme: "light" };
let settings = { ...defaults, ...userPrefs };
print(settings); // { theme: "light", fontSize: 12, showSidebar: true }

// Adding new properties while spreading
let person = { name: "John", age: 30 };
let employee = { ...person, jobTitle: "Developer", salary: 100000 };
print(employee); // { name: "John", age: 30, jobTitle: "Developer", salary: 100000 }
```

## Practical Examples

### Working with API Responses

```js
// Transforming API data
async fn fetchUserData(userId) {
  const response = await fetch(`https://api.example.com/users/${userId}`);
  const userData = await response.json();
  
  // Extract specific fields and add a formatted name
  const { firstName, lastName, email, ...otherInfo } = userData;
  
  return {
    fullName: `${firstName} ${lastName}`,
    email,
    ...otherInfo,
    lastFetched: new Date().toISOString()
  };
}

// Using the function
const user = await fetchUserData(123);
const { fullName, email } = user;
print(`User: ${fullName} (${email})`);
```

### Function Composition

```js
// Creating a function that accepts variable arguments
fn sum(...numbers) {
  return numbers.reduce(fn(total, num) => total + num, 0);
}

print(sum(1, 2, 3, 4, 5)); // 15

// Function that forwards arguments
fn logAndSum(message, ...numbers) {
  print(message);
  return sum(...numbers);
}

print(logAndSum("Adding numbers:", 1, 2, 3)); // Logs "Adding numbers:" and returns 6
```

### Immutable Data Patterns

```js
// Immutable state updates (similar to Redux patterns)
const initialState = {
  user: {
    name: "John",
    preferences: {
      theme: "dark",
      notifications: true
    }
  },
  posts: []
};

// Update nested property immutably
const updatedState = {
  ...initialState,
  user: {
    ...initialState.user,
    preferences: {
      ...initialState.user.preferences,
      theme: "light"
    }
  }
};

// Add item to array immutably
const stateWithNewPost = {
  ...updatedState,
  posts: [...updatedState.posts, { id: 1, title: "New Post" }]
};

print(stateWithNewPost.user.preferences.theme); // "light"
print(stateWithNewPost.posts.length); // 1
```

## Implementation Notes

When implementing destructuring and spread operators in SmashLang, consider the following:

1. **Parsing**: The parser needs to recognize the destructuring patterns in variable declarations, assignments, and function parameters.

2. **Type Checking**: The type system should validate that destructuring patterns match the structure of the values being destructured.

3. **Code Generation**: The compiler needs to generate appropriate code to extract values from arrays and objects.

4. **Runtime Support**: The runtime library should provide functions to handle spreading of arrays and objects.

5. **Error Handling**: Appropriate error messages should be generated when destructuring fails at runtime (e.g., trying to destructure a null value).

## See Also

- [Functions and Closures](./functions.md)
- [Arrays and Objects](./collections.md)
- [Pattern Matching](./pattern-matching.md)
