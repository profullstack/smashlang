# SmashLang Control Flow

SmashLang provides familiar control flow structures with JavaScript-inspired syntax.

## Conditionals

### If-Else Statements

```js
// Basic if statement
let age = 25;

if (age >= 18) {
    print("Adult");
}

// If-else statement
let score = 75;

if (score >= 90) {
    print("Grade: A");
} else if (score >= 80) {
    print("Grade: B");
} else if (score >= 70) {
    print("Grade: C");
} else if (score >= 60) {
    print("Grade: D");
} else {
    print("Grade: F");
}
```

### Ternary Operator

```js
let status = age >= 18 ? "adult" : "minor";
print(`Status: ${status}`);
```

### Switch Statement

```js
let day = 3;
let dayName;

switch (day) {
    case 1:
        dayName = "Monday";
        break;
    case 2:
        dayName = "Tuesday";
        break;
    case 3:
        dayName = "Wednesday";
        break;
    case 4:
        dayName = "Thursday";
        break;
    case 5:
        dayName = "Friday";
        break;
    case 6:
        dayName = "Saturday";
        break;
    case 7:
        dayName = "Sunday";
        break;
    default:
        dayName = "Invalid day";
}

print(`Day: ${dayName}`);
```

## Loops

### For Loop

```js
for (let i = 0; i < 5; i++) {
    print(`Iteration ${i}`);
}
```

### While Loop

```js
let counter = 0;
while (counter < 5) {
    print(`Count: ${counter}`);
    counter++;
}
```

### Do-While Loop

```js
let num = 0;
do {
    print(`Number: ${num}`);
    num++;
} while (num < 5);
```

### For-In Loop (for objects)

```js
let person = {
    name: "John",
    age: 30,
    job: "Developer"
};

for (let key in person) {
    print(`${key}: ${person[key]}`);
}
```

### For-Of Loop (for arrays)

```js
let fruits = ["apple", "banana", "orange", "grape"];

for (let fruit of fruits) {
    print(fruit);
}
```

### Break and Continue

```js
// Break example
for (let i = 0; i < 10; i++) {
    if (i === 5) break;
    print(`Break loop: ${i}`);
}

// Continue example
for (let i = 0; i < 10; i++) {
    if (i % 2 === 0) continue;
    print(`Continue loop: ${i}`);
}
```

### Nested Loops

```js
for (let i = 0; i < 3; i++) {
    for (let j = 0; j < 3; j++) {
        print(`Position (${i}, ${j})`);
    }
}
```

### Labeled Statements

```js
outer: for (let i = 0; i < 3; i++) {
    for (let j = 0; j < 3; j++) {
        if (i === 1 && j === 1) {
            print("Breaking outer loop");
            break outer;
        }
        print(`Label position (${i}, ${j})`);
    }
}
```

## Practical Examples

### Data Processing with Loops

```js
let temperatures = [22.5, 25.3, 18.9, 20.1, 23.4, 19.8, 21.2];

// Calculate average temperature using a for loop
fn calculateAverage(data) {
    let sum = 0;
    
    for (let i = 0; i < data.length; i++) {
        sum += data[i];
    }
    
    return data.length > 0 ? sum / data.length : 0;
}

print(`Average temperature: ${calculateAverage(temperatures).toFixed(2)}°C`);
```

### Conditional Logic for Input Validation

```js
fn validateUserInput(username, password) {
    // Check if username is provided
    if (!username) {
        return { valid: false, message: "Username is required" };
    }
    
    // Check username length
    if (username.length < 3) {
        return { valid: false, message: "Username must be at least 3 characters" };
    }
    
    // Check if password is provided
    if (!password) {
        return { valid: false, message: "Password is required" };
    }
    
    // Check password strength with nested conditions
    if (password.length < 8) {
        return { valid: false, message: "Password must be at least 8 characters" };
    }
    
    // If all checks pass
    return { valid: true, message: "Input is valid" };
}
```

### Combining Async/Await with Control Flow

```js
async fn fetchUserData(userIds) {
    let results = [];
    
    for (let i = 0; i < userIds.length; i++) {
        let userId = userIds[i];
        print(`Fetching data for user ${userId}...`);
        
        try {
            const response = await fetch(`https://api.example.com/users/${userId}`);
            
            if (response.status === 200) {
                const userData = await response.json();
                results.push(userData);
                print(`Successfully fetched data for user ${userId}`);
            } else if (response.status === 404) {
                print(`User ${userId} not found`);
            } else {
                print(`Error fetching user ${userId}: ${response.status}`);
            }
        } catch (error) {
            print(`Exception while fetching user ${userId}: ${error}`);
            // Continue with next user despite error
            continue;
        }
    }
    
    return results;
}
```
