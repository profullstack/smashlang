# Math Package

<p align="center">
  <img src="../../../smashlang_packages/core/math/assets/logo.light.svg" alt="Math Package Logo" width="200" />
</p>

The Math package provides comprehensive mathematical functions and utilities for SmashLang applications. It includes basic arithmetic operations, advanced mathematical functions, and support for vectors and matrices.

## Installation

```bash
smashpkg install math
```

## Features

- Basic arithmetic operations (add, subtract, multiply, divide)
- Advanced mathematical functions (sin, cos, tan, log, etc.)
- Statistical functions (mean, median, mode, standard deviation)
- Vector and matrix operations
- Random number generation
- Number formatting and parsing

## Basic Usage

```js
import { math } from "math";

// Basic arithmetic
const sum = math.add(5, 3);        // 8
const difference = math.subtract(5, 3); // 2
const product = math.multiply(5, 3);    // 15
const quotient = math.divide(6, 3);     // 2

// Advanced functions
const sinValue = math.sin(Math.PI / 2); // 1
const logValue = math.log(10);          // 2.302585092994046
const sqrtValue = math.sqrt(16);        // 4

// Statistical functions
const data = [1, 2, 3, 4, 5];
const average = math.mean(data);        // 3
const median = math.median(data);       // 3
const stdDev = math.stdDev(data);       // ~1.414

// Random numbers
const randomNum = math.random();        // Random number between 0 and 1
const randomInt = math.randomInt(1, 10); // Random integer between 1 and 10
```

## Advanced Usage

### Vector Operations

```js
import { Vector } from "math";

// Create vectors
const v1 = new Vector([1, 2, 3]);
const v2 = new Vector([4, 5, 6]);

// Vector operations
const v3 = v1.add(v2);         // Vector([5, 7, 9])
const dotProduct = v1.dot(v2); // 32
const magnitude = v1.magnitude(); // ~3.742
const normalized = v1.normalize(); // Vector([~0.267, ~0.535, ~0.802])
```

### Matrix Operations

```js
import { Matrix } from "math";

// Create matrices
const m1 = new Matrix([
  [1, 2],
  [3, 4]
]);
const m2 = new Matrix([
  [5, 6],
  [7, 8]
]);

// Matrix operations
const m3 = m1.add(m2);         // Matrix([[6, 8], [10, 12]])
const product = m1.multiply(m2); // Matrix([[19, 22], [43, 50]])
const determinant = m1.determinant(); // -2
const inverse = m1.inverse();  // Matrix([[-2, 1], [1.5, -0.5]])
const transpose = m1.transpose(); // Matrix([[1, 3], [2, 4]])
```

### Statistical Analysis

```js
import { stats } from "math";

const dataset = [12, 15, 18, 22, 25, 30, 35, 40, 45, 50];

// Basic statistics
const mean = stats.mean(dataset);     // 29.2
const median = stats.median(dataset); // 27.5
const mode = stats.mode(dataset);     // null (no mode)
const variance = stats.variance(dataset); // 156.96
const stdDev = stats.stdDev(dataset); // ~12.53

// Data analysis
const quartiles = stats.quartiles(dataset); // [18, 27.5, 40]
const range = stats.range(dataset);   // 38
const iqr = stats.iqr(dataset);       // 22
```

## API Reference

### Basic Math Functions

#### `math.add(a, b)`
Adds two numbers.
- **Parameters**: 
  - `a` (Number): First number
  - `b` (Number): Second number
- **Returns**: (Number) Sum of a and b

#### `math.subtract(a, b)`
Subtracts the second number from the first.
- **Parameters**: 
  - `a` (Number): First number
  - `b` (Number): Second number
- **Returns**: (Number) Difference of a and b

#### `math.multiply(a, b)`
Multiplies two numbers.
- **Parameters**: 
  - `a` (Number): First number
  - `b` (Number): Second number
- **Returns**: (Number) Product of a and b

#### `math.divide(a, b)`
Divides the first number by the second.
- **Parameters**: 
  - `a` (Number): Numerator
  - `b` (Number): Denominator
- **Returns**: (Number) Quotient of a and b
- **Throws**: Error if b is zero

### Advanced Math Functions

#### `math.sin(x)`
Calculates the sine of an angle.
- **Parameters**: 
  - `x` (Number): Angle in radians
- **Returns**: (Number) Sine of x

#### `math.cos(x)`
Calculates the cosine of an angle.
- **Parameters**: 
  - `x` (Number): Angle in radians
- **Returns**: (Number) Cosine of x

#### `math.tan(x)`
Calculates the tangent of an angle.
- **Parameters**: 
  - `x` (Number): Angle in radians
- **Returns**: (Number) Tangent of x

#### `math.log(x, [base=Math.E])`
Calculates the logarithm of a number.
- **Parameters**: 
  - `x` (Number): The number
  - `base` (Number, optional): The logarithm base (defaults to e)
- **Returns**: (Number) Logarithm of x with the specified base
- **Throws**: Error if x <= 0

### Statistical Functions

#### `stats.mean(data)`
Calculates the arithmetic mean of an array of numbers.
- **Parameters**: 
  - `data` (Array<Number>): Array of numbers
- **Returns**: (Number) Arithmetic mean

#### `stats.median(data)`
Calculates the median of an array of numbers.
- **Parameters**: 
  - `data` (Array<Number>): Array of numbers
- **Returns**: (Number) Median value

#### `stats.mode(data)`
Finds the mode (most frequent value) of an array of numbers.
- **Parameters**: 
  - `data` (Array<Number>): Array of numbers
- **Returns**: (Number|null) Mode value, or null if there is no mode

### Vector Class

#### `new Vector(components)`
Creates a new vector.
- **Parameters**: 
  - `components` (Array<Number>): Vector components
- **Returns**: (Vector) New Vector instance

#### `vector.add(v)`
Adds another vector to this vector.
- **Parameters**: 
  - `v` (Vector): Vector to add
- **Returns**: (Vector) New vector representing the sum

#### `vector.subtract(v)`
Subtracts another vector from this vector.
- **Parameters**: 
  - `v` (Vector): Vector to subtract
- **Returns**: (Vector) New vector representing the difference

#### `vector.dot(v)`
Calculates the dot product with another vector.
- **Parameters**: 
  - `v` (Vector): Second vector
- **Returns**: (Number) Dot product

### Matrix Class

#### `new Matrix(data)`
Creates a new matrix.
- **Parameters**: 
  - `data` (Array<Array<Number>>): 2D array of numbers
- **Returns**: (Matrix) New Matrix instance

#### `matrix.add(m)`
Adds another matrix to this matrix.
- **Parameters**: 
  - `m` (Matrix): Matrix to add
- **Returns**: (Matrix) New matrix representing the sum

#### `matrix.multiply(m)`
Multiplies this matrix by another matrix.
- **Parameters**: 
  - `m` (Matrix): Matrix to multiply by
- **Returns**: (Matrix) New matrix representing the product

## Examples

See the [examples directory](../../../smashlang_packages/core/math/examples) for more detailed examples:

- [Basic Example](../../../smashlang_packages/core/math/examples/basic.smash): Demonstrates basic arithmetic and functions
- [Vector Example](../../../smashlang_packages/core/math/examples/vectors.smash): Shows vector operations
- [Matrix Example](../../../smashlang_packages/core/math/examples/matrices.smash): Illustrates matrix operations
- [Statistics Example](../../../smashlang_packages/core/math/examples/statistics.smash): Demonstrates statistical functions

## Testing

The Math package includes comprehensive tests:

```bash
# Run all tests for the math package
smashtest smashlang_packages/core/math/tests
```

## Contributing

Contributions to the Math package are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for your changes
5. Submit a pull request

## License

MIT