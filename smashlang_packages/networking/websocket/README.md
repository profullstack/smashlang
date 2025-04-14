# SmashLang WebSocket Package

A comprehensive WebSocket client and server implementation for SmashLang applications. This package provides functionality for creating WebSocket servers and clients with support for various protocols and message formats.

## Features

- **WebSocket Client**: Connect to WebSocket servers, send and receive messages, and handle events
- **WebSocket Server**: Create WebSocket servers, handle multiple client connections, and broadcast messages
- **Event-Based API**: Simple event-based API for handling connections, messages, errors, and disconnections
- **Binary Data Support**: Send and receive binary data (ArrayBuffer, Blob)
- **JSON Support**: Automatic JSON serialization and deserialization
- **Reconnection Logic**: Built-in support for automatic reconnection
- **URL Utilities**: Helper functions for working with WebSocket URLs

## Installation

```bash
smashpkg install networking/websocket
```

## Usage

### WebSocket Client

```javascript
import websocket from 'networking/websocket';

// Create a WebSocket client
const client = websocket.createClient('ws://echo.websocket.org');

// Set up event handlers
client.on('open', (event) => {
  console.log('Connection opened!');
  
  // Send a message
  client.send('Hello, WebSocket!');
  
  // Send a JSON object
  client.send({
    type: 'greeting',
    content: 'Hello from SmashLang WebSocket client!'
  });
});

client.on('message', (event) => {
  console.log(`Received message: ${event.data}`);
});

client.on('close', (event) => {
  console.log(`Connection closed with code ${event.code}: ${event.reason}`);
});

client.on('error', (event) => {
  console.log(`Error: ${event.error.message}`);
});

// Connect to the server
client.connect().then(() => {
  console.log('Connected successfully!');
}).catch((error) => {
  console.log(`Connection error: ${error.message}`);
});

// Close the connection
client.close(websocket.CLOSE_CODES.NORMAL, 'Example completed');
```

### WebSocket Server

```javascript
import websocket from 'networking/websocket';

// Create a WebSocket server
const server = websocket.createServer({
  port: 8080,
  host: 'localhost',
  path: '/ws'
});

// Set up connection event handler
server.on('connection', (event) => {
  const client = event.client;
  console.log('New client connected!');
  
  // Send a welcome message
  client.send('Welcome to the server!');
  
  // Set up message handler for this client
  client.on('message', (messageEvent) => {
    console.log(`Received message: ${messageEvent.data}`);
    
    // Echo the message back
    client.send(`Echo: ${messageEvent.data}`);
  });
  
  // Set up close handler for this client
  client.on('close', (closeEvent) => {
    console.log(`Client disconnected: ${closeEvent.reason}`);
  });
});

// Start the server
server.start().then(() => {
  console.log(`Server running on ws://${server.getOptions().host}:${server.getOptions().port}${server.getOptions().path}`);
}).catch((error) => {
  console.log(`Failed to start server: ${error.message}`);
});

// Broadcast a message to all clients
server.broadcast('Broadcast message to all clients');

// Broadcast with a filter
server.broadcast('Message to some clients', (client) => {
  // Only send to clients that meet certain criteria
  return client.getState() === websocket.STATES.OPEN;
});

// Stop the server
server.stop().then(() => {
  console.log('Server stopped.');
});
```

## API Reference

### WebSocketClient

#### Constructor

```javascript
new WebSocketClient(url, protocols = [], options = {})
```

- `url` (string): WebSocket server URL
- `protocols` (string|string[]): WebSocket protocols
- `options` (Object): Additional options

#### Methods

- `connect()`: Connect to the WebSocket server
- `send(data)`: Send a message to the WebSocket server
- `close(code, reason)`: Close the WebSocket connection
- `on(event, callback)`: Add an event listener
- `off(event, callback)`: Remove an event listener
- `getState()`: Get the current state
- `isOpen()`: Check if the connection is open
- `getUrl()`: Get the WebSocket URL
- `getProtocols()`: Get the WebSocket protocols

#### Events

- `open`: Fired when the connection is established
- `message`: Fired when a message is received
- `close`: Fired when the connection is closed
- `error`: Fired when an error occurs

### WebSocketServer

#### Constructor

```javascript
new WebSocketServer(options = {})
```

- `options` (Object): Server options
  - `port` (number): Port to listen on (default: 8080)
  - `host` (string): Host to bind to (default: 'localhost')
  - `path` (string): URL path (default: '/')

#### Methods

- `start()`: Start the WebSocket server
- `stop()`: Stop the WebSocket server
- `broadcast(data, filter)`: Broadcast a message to all connected clients
- `on(event, callback)`: Add an event listener
- `off(event, callback)`: Remove an event listener
- `getClients()`: Get all connected clients
- `getClientCount()`: Get the number of connected clients
- `isStarted()`: Check if the server is running
- `getOptions()`: Get the server options

#### Events

- `connection`: Fired when a client connects
- `close`: Fired when the server is closed
- `error`: Fired when an error occurs

### Utility Functions

- `createClient(url, protocols, options)`: Create a WebSocket client
- `createServer(options)`: Create a WebSocket server
- `isValidUrl(url)`: Check if a URL is a valid WebSocket URL
- `httpToWsUrl(url)`: Convert a HTTP URL to a WebSocket URL

### Constants

- `STATES`: WebSocket connection states
  - `CONNECTING`: Socket has been created. The connection is not yet open.
  - `OPEN`: The connection is open and ready to communicate.
  - `CLOSING`: The connection is in the process of closing.
  - `CLOSED`: The connection is closed or couldn't be opened.

- `CLOSE_CODES`: WebSocket close codes
  - `NORMAL`: Normal closure
  - `GOING_AWAY`: Client is navigating away
  - `PROTOCOL_ERROR`: Protocol error
  - `UNSUPPORTED_DATA`: Unsupported data
  - ... and many more standard close codes

## Examples

The package includes several examples:

1. **Basic Example**: Simple client connecting to an echo server
2. **Chat Server**: A complete chat server implementation
3. **Chat Client**: A client for the chat server

To run the examples:

```bash
# Basic example
smash run smashlang_packages/networking/websocket/examples/basic.smash

# Chat server
smash run smashlang_packages/networking/websocket/examples/chat_server.smash

# Chat client (in a separate terminal)
smash run smashlang_packages/networking/websocket/examples/chat_client.smash
```

## License

MIT
