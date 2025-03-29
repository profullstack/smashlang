# Object Enhancements in SmashLang

SmashLang provides modern JavaScript-like enhancements for working with objects, making your code more concise and expressive.

## Shorthand Property Names

When creating an object, if a property has the same name as a variable in scope, you can use the shorthand syntax to avoid repetition.

```js
// Instead of writing this:
let name = "Alice";
let age = 30;
let user = { name: name, age: age };

// You can write this:
let name = "Alice";
let age = 30;
let user = { name, age };

// Both create an object: { name: "Alice", age: 30 }
```

This shorthand syntax is especially useful when returning objects from functions or when working with multiple properties:

```js
fn getUserInfo(id) {
  let name = fetchName(id);
  let email = fetchEmail(id);
  let role = fetchRole(id);
  let active = isActive(id);
  
  // Return object with all properties using shorthand syntax
  return { name, email, role, active };
}
```

## Spread Operator for Objects

The spread operator (`...`) allows you to copy properties from one object into another. This is useful for creating new objects based on existing ones.

```js
// Basic object spread
let defaults = { theme: "dark", fontSize: 12, showSidebar: true };
let userPrefs = { theme: "light" };

// Combine objects - properties from userPrefs override those from defaults
let settings = { ...defaults, ...userPrefs };
// Result: { theme: "light", fontSize: 12, showSidebar: true }
```

### Use Cases for Object Spread

#### 1. Creating copies of objects

```js
let original = { a: 1, b: 2 };
let copy = { ...original };

// Modifying the copy doesn't affect the original
copy.a = 99;
console.log(original.a); // 1
console.log(copy.a);     // 99
```

#### 2. Adding new properties while preserving existing ones

```js
let user = { name: "Alice", role: "Admin" };
let userWithDetails = { ...user, id: 42, active: true };
// Result: { name: "Alice", role: "Admin", id: 42, active: true }
```

#### 3. Merging multiple objects

```js
let person = { name: "Bob" };
let job = { title: "Developer" };
let location = { city: "San Francisco" };

let employee = { ...person, ...job, ...location };
// Result: { name: "Bob", title: "Developer", city: "San Francisco" }
```

#### 4. Conditionally including properties

```js
let isAdmin = checkAdminStatus(userId);

let user = {
  name: "Charlie",
  ...(isAdmin ? { adminSince: new Date() } : {})
};
```

## Combining Shorthand Properties and Spread

You can combine both features for even more concise code:

```js
let name = "Dave";
let role = "Editor";
let basePermissions = { canRead: true, canWrite: true };

let user = {
  id: generateId(),
  name,  // shorthand property
  role,  // shorthand property
  ...basePermissions  // spread operator
};

// Result: { id: "...", name: "Dave", role: "Editor", canRead: true, canWrite: true }
```

## Implementation Notes

Behind the scenes, SmashLang's compiler transforms these syntactic enhancements into equivalent code that works across all supported platforms. The shorthand property syntax is converted to standard key-value pairs, and the spread operator is implemented as a series of property assignments.

## Best Practices

1. **Use shorthand properties** when the property name matches the variable name to make your code more concise.

2. **Use the spread operator** for creating new objects based on existing ones rather than mutating objects directly.

3. **Be aware of property order** when using spread operators, as later properties will override earlier ones with the same name.

4. **Consider performance** when spreading large objects, as each property is copied individually.

## See Also

- [Destructuring Assignment](./destructuring.md)
- [Arrays and Collections](./collections.md)
- [Functions and Closures](./functions.md)
