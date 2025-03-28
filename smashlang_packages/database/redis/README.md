# Redis Client for SmashLang

A high-performance Redis client library for SmashLang with support for all Redis data structures and commands.

## Installation

```bash
smashpkg install redis
```

## Features

- Complete Redis command coverage
- Support for all Redis data structures (Strings, Lists, Sets, Hashes, Sorted Sets)
- Pub/Sub functionality
- Transaction support
- Lua scripting
- Pipelining for improved performance
- Redis Cluster support

## Basic Usage

```js
import "redis";

// Create a Redis client
const client = redis.createClient({
  host: "localhost",
  port: 6379
});

// Basic operations
async fn main() {
  // String operations
  await client.commands.string.set("greeting", "Hello, SmashLang!");
  const greeting = await client.commands.string.get("greeting");
  console.log(greeting);  // "Hello, SmashLang!"
  
  // Hash operations
  await client.commands.hash.hset("user:1000", "name", "John");
  await client.commands.hash.hset("user:1000", "email", "john@example.com");
  const userData = await client.commands.hash.hgetall("user:1000");
  console.log(userData);  // { name: "John", email: "john@example.com" }
  
  // List operations
  await client.commands.list.lpush("tasks", "Learn SmashLang");
  await client.commands.list.lpush("tasks", "Build Redis app");
  const tasks = await client.commands.list.lrange("tasks", 0, -1);
  console.log(tasks);  // ["Build Redis app", "Learn SmashLang"]
  
  // Close the connection
  await client.quit();
}

main().catch(console.error);
```

## Advanced Features

### Transactions

```js
async fn transactionExample() {
  const multi = client.commands.transaction.multi();
  
  multi.commands.string.set("counter", "10");
  multi.commands.string.incr("counter");
  multi.commands.string.incr("counter");
  
  const results = await multi.commands.transaction.exec();
  console.log(results);  // ["OK", 11, 12]
}
```

### Pub/Sub

```js
async fn pubSubExample() {
  // Subscriber
  const subscriber = redis.createClient();
  await subscriber.commands.pubsub.subscribe("news-channel");
  
  subscriber.on("message", (channel, message) => {
    console.log(`Received message from ${channel}: ${message}`);
  });
  
  // Publisher
  const publisher = redis.createClient();
  await publisher.commands.pubsub.publish("news-channel", "Breaking news!");
}
```

### Pipelining

```js
async fn pipelineExample() {
  const pipeline = client.pipeline();
  
  pipeline.commands.string.set("key1", "value1");
  pipeline.commands.string.set("key2", "value2");
  pipeline.commands.string.set("key3", "value3");
  pipeline.commands.string.get("key1");
  pipeline.commands.string.get("key2");
  pipeline.commands.string.get("key3");
  
  const results = await pipeline.exec();
  console.log(results);  // ["OK", "OK", "OK", "value1", "value2", "value3"]
}
```

## Examples

See the [examples directory](./examples) for more detailed examples:

- **Basic Operations**: Working with strings, lists, sets, hashes, and sorted sets
- **Transactions**: Using multi/exec for atomic operations
- **Pub/Sub**: Implementing publish/subscribe patterns
- **Lua Scripting**: Running Lua scripts on Redis
- **Caching**: Implementing a caching layer with Redis

## API Reference

### Client

```js
const client = redis.createClient(options);
```

Options:
- `host`: Redis server hostname (default: "localhost")
- `port`: Redis server port (default: 6379)
- `password`: Redis server password
- `db`: Database index to use
- `tls`: TLS/SSL options

### Commands

The client provides access to all Redis commands organized by data structure:

#### String Commands
- `get(key)`: Get the value of a key
- `set(key, value, options)`: Set the value of a key
- `append(key, value)`: Append a value to a key
- `incr(key)`: Increment the integer value of a key
- `decr(key)`: Decrement the integer value of a key

#### Hash Commands
- `hget(key, field)`: Get the value of a hash field
- `hset(key, field, value)`: Set the value of a hash field
- `hgetall(key)`: Get all fields and values in a hash
- `hdel(key, field)`: Delete a hash field

#### List Commands
- `lpush(key, ...values)`: Prepend values to a list
- `rpush(key, ...values)`: Append values to a list
- `lpop(key)`: Remove and get the first element in a list
- `rpop(key)`: Remove and get the last element in a list
- `lrange(key, start, stop)`: Get a range of elements from a list

#### Set Commands
- `sadd(key, ...members)`: Add members to a set
- `srem(key, ...members)`: Remove members from a set
- `smembers(key)`: Get all members in a set
- `sismember(key, member)`: Check if a value is a member of a set

#### Sorted Set Commands
- `zadd(key, score, member)`: Add a member to a sorted set
- `zrange(key, start, stop, options)`: Get a range of members from a sorted set
- `zrank(key, member)`: Get the rank of a member in a sorted set
- `zscore(key, member)`: Get the score of a member in a sorted set

#### PubSub Commands
- `subscribe(...channels)`: Subscribe to channels
- `publish(channel, message)`: Publish a message to a channel
- `unsubscribe(...channels)`: Unsubscribe from channels

#### Transaction Commands
- `multi()`: Start a transaction
- `exec()`: Execute a transaction
- `discard()`: Discard a transaction

#### Scripting Commands
- `eval(script, numKeys, ...keysAndArgs)`: Execute a Lua script
- `evalsha(sha1, numKeys, ...keysAndArgs)`: Execute a Lua script cached on the server

## License

MIT
