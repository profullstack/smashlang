# PostgreSQL Package

<p align="center">
  <img src="../../../smashlang_packages/database/postgres/assets/logo.light.svg" alt="PostgreSQL Package Logo" width="200" />
</p>

The PostgreSQL package provides a robust client for interacting with PostgreSQL databases from SmashLang applications. It supports all PostgreSQL features, including queries, transactions, prepared statements, and more.

## Installation

```bash
smashpkg install postgres
```

## Features

- Connection management with connection pooling
- SQL query execution
- Prepared statements with parameter binding
- Transaction support
- Result set handling
- Data type conversion
- Error handling
- Connection pooling
- Streaming results for large datasets
- Support for PostgreSQL-specific features (LISTEN/NOTIFY, COPY, etc.)
- Support for JSON, JSONB, and other PostgreSQL data types
- Async/await support

## Basic Usage

```js
import { postgres } from "postgres";

// Create a connection
const db = postgres.connect("postgresql://username:password@localhost:5432/mydatabase");

// Simple query
const result = await db.query("SELECT * FROM users WHERE active = true");
console.log(`Found ${result.rows.length} active users`);

// Iterate through results
for (const row of result.rows) {
  console.log(`User: ${row.name}, Email: ${row.email}`);
}

// Close the connection when done
await db.close();
```

## Advanced Usage

### Parameterized Queries

```js
import { postgres } from "postgres";

const db = postgres.connect("postgresql://localhost/mydatabase");

// Using parameterized queries to prevent SQL injection
const userId = 123;
const result = await db.query(
  "SELECT * FROM users WHERE id = $1",
  [userId]
);

if (result.rows.length > 0) {
  const user = result.rows[0];
  console.log(`Found user: ${user.name}`);
} else {
  console.log("User not found");
}

await db.close();
```

### Transactions

```js
import { postgres } from "postgres";

const db = postgres.connect("postgresql://localhost/mydatabase");

// Start a transaction
const transaction = await db.beginTransaction();

try {
  // Multiple operations in a single transaction
  await transaction.query(
    "INSERT INTO accounts (user_id, balance) VALUES ($1, $2)",
    [1, 1000]
  );
  
  await transaction.query(
    "UPDATE users SET account_created = true WHERE id = $1",
    [1]
  );
  
  // Commit the transaction
  await transaction.commit();
  console.log("Transaction committed successfully");
} catch (error) {
  // Rollback on error
  await transaction.rollback();
  console.error("Transaction failed:", error);
}

await db.close();
```

### Connection Pooling

```js
import { postgres } from "postgres";

// Create a connection pool
const pool = postgres.createPool({
  connectionString: "postgresql://localhost/mydatabase",
  min: 2,       // Minimum connections in pool
  max: 10,      // Maximum connections in pool
  idleTimeout: 30000  // Close idle connections after 30 seconds
});

// Get a connection from the pool
const client = await pool.connect();

try {
  // Use the connection
  const result = await client.query("SELECT NOW()");
  console.log("Current time:", result.rows[0].now);
} finally {
  // Return the connection to the pool
  client.release();
}

// For simple queries, you can use the pool directly
const userCount = await pool.query("SELECT COUNT(*) FROM users");
console.log("User count:", userCount.rows[0].count);

// Close the pool when your application shuts down
await pool.end();
```

### Streaming Large Result Sets

```js
import { postgres } from "postgres";

const db = postgres.connect("postgresql://localhost/mydatabase");

// Stream results for large datasets
const stream = await db.queryStream("SELECT * FROM large_table");

// Process rows as they arrive
for await (const row of stream) {
  // Process each row without loading the entire result set into memory
  console.log(`Processing row: ${row.id}`);
}

await db.close();
```

### Working with JSON Data

```js
import { postgres } from "postgres";

const db = postgres.connect("postgresql://localhost/mydatabase");

// Insert JSON data
const userData = {
  name: "John Doe",
  preferences: {
    theme: "dark",
    notifications: true
  },
  tags: ["developer", "javascript"]
};

await db.query(
  "INSERT INTO users (name, data) VALUES ($1, $2)",
  ["John Doe", userData]
);

// Query JSON data
const result = await db.query(
  "SELECT * FROM users WHERE data->>'name' = $1",
  ["John Doe"]
);

// Access JSON fields
for (const row of result.rows) {
  console.log(`User: ${row.name}`);
  console.log(`Theme: ${row.data.preferences.theme}`);
  console.log(`Tags: ${row.data.tags.join(', ')}`);
}

await db.close();
```

## API Reference

### Connection Functions

#### `postgres.connect(connectionString, options)`
Creates a new database connection.
- **Parameters**: 
  - `connectionString` (String): PostgreSQL connection string
  - `options` (Object, optional): Connection options
    - `ssl` (Boolean|Object): SSL configuration
    - `timeout` (Number): Connection timeout in milliseconds
    - `application_name` (String): Application name for logging
- **Returns**: (Connection) Database connection object

#### `postgres.createPool(options)`
Creates a connection pool.
- **Parameters**: 
  - `options` (Object): Pool configuration
    - `connectionString` (String): PostgreSQL connection string
    - `min` (Number): Minimum number of connections
    - `max` (Number): Maximum number of connections
    - `idleTimeout` (Number): Time in ms before idle connections are closed
- **Returns**: (Pool) Connection pool object

### Connection Object

#### `connection.query(text, params)`
Executes a query.
- **Parameters**: 
  - `text` (String): SQL query text
  - `params` (Array, optional): Query parameters
- **Returns**: (Promise<Result>) Promise resolving to a result object

#### `connection.queryStream(text, params)`
Executes a query and returns a stream of results.
- **Parameters**: 
  - `text` (String): SQL query text
  - `params` (Array, optional): Query parameters
- **Returns**: (AsyncIterable<Object>) Async iterable of result rows

#### `connection.beginTransaction()`
Begins a new transaction.
- **Returns**: (Promise<Transaction>) Promise resolving to a transaction object

#### `connection.close()`
Closes the connection.
- **Returns**: (Promise<void>) Promise that resolves when the connection is closed

### Transaction Object

#### `transaction.query(text, params)`
Executes a query within the transaction.
- **Parameters**: 
  - `text` (String): SQL query text
  - `params` (Array, optional): Query parameters
- **Returns**: (Promise<Result>) Promise resolving to a result object

#### `transaction.commit()`
Commits the transaction.
- **Returns**: (Promise<void>) Promise that resolves when the transaction is committed

#### `transaction.rollback()`
Rolls back the transaction.
- **Returns**: (Promise<void>) Promise that resolves when the transaction is rolled back

### Pool Object

#### `pool.connect()`
Gets a connection from the pool.
- **Returns**: (Promise<PoolClient>) Promise resolving to a pool client

#### `pool.query(text, params)`
Executes a query using a connection from the pool.
- **Parameters**: 
  - `text` (String): SQL query text
  - `params` (Array, optional): Query parameters
- **Returns**: (Promise<Result>) Promise resolving to a result object

#### `pool.end()`
Closes all connections in the pool.
- **Returns**: (Promise<void>) Promise that resolves when all connections are closed

### PoolClient Object

#### `poolClient.query(text, params)`
Executes a query using the pool client.
- **Parameters**: 
  - `text` (String): SQL query text
  - `params` (Array, optional): Query parameters
- **Returns**: (Promise<Result>) Promise resolving to a result object

#### `poolClient.release()`
Returns the client to the pool.
- **Returns**: (void)

### Result Object

#### `result.rows`
The rows returned by the query.
- **Type**: (Array<Object>)

#### `result.rowCount`
The number of rows affected by the query.
- **Type**: (Number)

#### `result.fields`
Information about the result fields.
- **Type**: (Array<Object>)

## Examples

See the [examples directory](../../../smashlang_packages/database/postgres/examples) for more detailed examples:

- [Basic Example](../../../smashlang_packages/database/postgres/examples/basic.smash): Demonstrates simple queries
- [Transaction Example](../../../smashlang_packages/database/postgres/examples/transaction.smash): Shows transaction handling
- [Pool Example](../../../smashlang_packages/database/postgres/examples/pool.smash): Demonstrates connection pooling
- [JSON Example](../../../smashlang_packages/database/postgres/examples/json.smash): Shows working with JSON data

## Testing

The PostgreSQL package includes comprehensive tests:

```bash
# Run all tests for the postgres package
smashtest smashlang_packages/database/postgres/tests
```

## Contributing

Contributions to the PostgreSQL package are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for your changes
5. Submit a pull request

## License

MIT