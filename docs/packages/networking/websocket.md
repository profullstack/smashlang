# WebSocket Package

The WebSocket package provides a comprehensive implementation of the WebSocket protocol for SmashLang applications. It includes both client and server functionality, allowing developers to create real-time applications with bidirectional communication.

## Overview

WebSockets enable real-time, bidirectional communication between clients and servers. Unlike HTTP, which is a request-response protocol, WebSockets provide a persistent connection that allows both the client and server to send messages at any time.

The SmashLang WebSocket package provides:

- A WebSocket client for connecting to WebSocket servers
- A WebSocket server for accepting client connections
- Event-based API for handling connections, messages, and errors
- Support for binary data and JSON messages
- Utility functions for working with WebSocket URLs

## Installation

```bash
smashpkg install networking/websocket
```

## Basic Usage

### WebSocket Client

```javascript
import websocket from 'networking/websocket';

// Create a WebSocket client
const client = websocket.createClient('ws://echo.websocket.org');

// Set up event handlers
client.on('open', (event) => {
  console.log('Connection opened!');
  client.send('Hello, WebSocket!');
});

client.on('message', (event) => {
  console.log(`Received message: ${event.data}`);
});

client.on('close', (event) => {
  console.log(`Connection closed: ${event.reason}`);
});

client.on('error', (event) => {
  console.log(`Error: ${event.error.message}`);
});

// Connect to the server
client.connect();

// Later, close the connection
client.close(websocket.CLOSE_CODES.NORMAL, 'Done');
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

// Handle client connections
server.on('connection', (event) => {
  const client = event.client;
  console.log('New client connected!');
  
  client.on('message', (messageEvent) => {
    console.log(`Received message: ${messageEvent.data}`);
    client.send(`Echo: ${messageEvent.data}`);
  });
  
  client.on('close', (closeEvent) => {
    console.log(`Client disconnected: ${closeEvent.reason}`);
  });
});

// Start the server
server.start().then(() => {
  console.log('Server started!');
});

// Broadcast a message to all clients
server.broadcast('Server announcement');

// Stop the server
server.stop();
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

| Method | Description |
|--------|-------------|
| `connect()` | Connect to the WebSocket server |
| `send(data)` | Send a message to the WebSocket server |
| `close(code, reason)` | Close the WebSocket connection |
| `on(event, callback)` | Add an event listener |
| `off(event, callback)` | Remove an event listener |
| `getState()` | Get the current state |
| `isOpen()` | Check if the connection is open |
| `getUrl()` | Get the WebSocket URL |
| `getProtocols()` | Get the WebSocket protocols |

#### Events

| Event | Description |
|-------|-------------|
| `open` | Fired when the connection is established |
| `message` | Fired when a message is received |
| `close` | Fired when the connection is closed |
| `error` | Fired when an error occurs |

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

| Method | Description |
|--------|-------------|
| `start()` | Start the WebSocket server |
| `stop()` | Stop the WebSocket server |
| `broadcast(data, filter)` | Broadcast a message to all connected clients |
| `on(event, callback)` | Add an event listener |
| `off(event, callback)` | Remove an event listener |
| `getClients()` | Get all connected clients |
| `getClientCount()` | Get the number of connected clients |
| `isStarted()` | Check if the server is running |
| `getOptions()` | Get the server options |

#### Events

| Event | Description |
|-------|-------------|
| `connection` | Fired when a client connects |
| `close` | Fired when the server is closed |
| `error` | Fired when an error occurs |

### Utility Functions

| Function | Description |
|----------|-------------|
| `createClient(url, protocols, options)` | Create a WebSocket client |
| `createServer(options)` | Create a WebSocket server |
| `isValidUrl(url)` | Check if a URL is a valid WebSocket URL |
| `httpToWsUrl(url)` | Convert a HTTP URL to a WebSocket URL |

### Constants

#### STATES

WebSocket connection states:

| Constant | Value | Description |
|----------|-------|-------------|
| `CONNECTING` | 0 | Socket has been created. The connection is not yet open. |
| `OPEN` | 1 | The connection is open and ready to communicate. |
| `CLOSING` | 2 | The connection is in the process of closing. |
| `CLOSED` | 3 | The connection is closed or couldn't be opened. |

#### CLOSE_CODES

WebSocket close codes:

| Constant | Value | Description |
|----------|-------|-------------|
| `NORMAL` | 1000 | Normal closure |
| `GOING_AWAY` | 1001 | Client is navigating away |
| `PROTOCOL_ERROR` | 1002 | Protocol error |
| `UNSUPPORTED_DATA` | 1003 | Unsupported data |
| `RESERVED` | 1004 | Reserved |
| `NO_STATUS` | 1005 | No status received |
| `ABNORMAL` | 1006 | Abnormal closure |
| `INVALID_FRAME_PAYLOAD_DATA` | 1007 | Invalid frame payload data |
| `POLICY_VIOLATION` | 1008 | Policy violation |
| `MESSAGE_TOO_BIG` | 1009 | Message too big |
| `MISSING_EXTENSION` | 1010 | Missing extension |
| `INTERNAL_ERROR` | 1011 | Internal error |
| `SERVICE_RESTART` | 1012 | Service restart |
| `TRY_AGAIN_LATER` | 1013 | Try again later |
| `BAD_GATEWAY` | 1014 | Bad gateway |
| `TLS_HANDSHAKE` | 1015 | TLS handshake |

## Examples

### Basic Echo Client

```javascript
import websocket from 'networking/websocket';

const client = websocket.createClient('ws://echo.websocket.org');

client.on('open', () => {
  console.log('Connected to echo server');
  client.send('Hello, echo server!');
});

client.on('message', (event) => {
  console.log(`Echo: ${event.data}`);
  client.close();
});

client.connect();
```

### Chat Server

```javascript
import websocket from 'networking/websocket';

const server = websocket.createServer({ port: 8080 });
const users = new Map();

server.on('connection', (event) => {
  const client = event.client;
  const username = `User${Math.floor(Math.random() * 1000)}`;
  
  users.set(client, username);
  
  // Welcome message
  client.send(`Welcome, ${username}!`);
  
  // Broadcast join message
  server.broadcast(`${username} has joined the chat`, c => c !== client);
  
  client.on('message', (messageEvent) => {
    const message = messageEvent.data;
    
    // Broadcast the message to all clients
    server.broadcast(`${username}: ${message}`);
  });
  
  client.on('close', () => {
    // Broadcast leave message
    server.broadcast(`${username} has left the chat`);
    users.delete(client);
  });
});

server.start().then(() => {
  console.log('Chat server running on ws://localhost:8080');
});
```

### JSON Communication

```javascript
import websocket from 'networking/websocket';

const client = websocket.createClient('wss://api.example.com/ws');

client.on('open', () => {
  // Send a JSON message
  client.send({
    type: 'subscribe',
    channel: 'updates',
    params: { frequency: 'realtime' }
  });
});

client.on('message', (event) => {
  try {
    const data = JSON.parse(event.data);
    
    switch (data.type) {
      case 'update':
        console.log(`Update received: ${data.content}`);
        break;
      case 'error':
        console.error(`Error: ${data.message}`);
        break;
    }
  } catch (e) {
    console.error('Failed to parse message as JSON');
  }
});

client.connect();
```

## Best Practices

1. **Error Handling**: Always handle connection errors and unexpected closures.
2. **Reconnection Logic**: Implement reconnection logic for clients to handle network issues.
3. **Message Validation**: Validate messages before processing them.
4. **Secure Connections**: Use WSS (WebSocket Secure) for production applications.
5. **Resource Cleanup**: Always close connections when they are no longer needed.

## Common Issues

### Connection Failures

If you're having trouble connecting to a WebSocket server:

- Check that the URL is correct and uses the proper protocol (ws:// or wss://).
- Verify that the server is running and accessible.
- Check for firewall or proxy issues that might be blocking WebSocket connections.

### Message Format Issues

When sending or receiving messages:

- Ensure that JSON messages are properly formatted.
- Handle binary data appropriately.
- Be aware of message size limitations.

## Related Packages

- **http**: For HTTP requests and responses
- **smashhono**: Web framework with WebSocket support
- **redis**: For implementing pub/sub with WebSockets

## License

MIT