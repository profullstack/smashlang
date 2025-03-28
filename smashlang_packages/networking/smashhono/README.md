# SmashHono

<p align="center">
  <img src="https://raw.githubusercontent.com/honojs/hono/main/docs/images/hono-logo.png" alt="SmashHono logo" width="200" />
</p>

A lightweight, ultrafast web framework for SmashLang inspired by [Hono.js](https://hono.dev/). Hono (u708e) means "flame" in Japanese.

## Features

- **Ultrafast**: Optimized for performance with minimal overhead
- **Middleware**: Extensible middleware system
- **Type-safe**: Built with type safety in mind
- **Multi-protocol**: Supports REST APIs, WebSockets, and GraphQL
- **Intuitive API**: Simple and expressive API design
- **Zero dependencies**: Minimal core with modular extensions

## Installation

```bash
smashpkg install networking/smashhono
```

## Basic Usage

```javascript
import "smashhono";

const app = new SmashHono();

app.get("/", (c) => c.text("Hello, SmashLang!"));

app.get("/json", (c) => {
  return c.json({ message: "Hello, JSON!" });
});

app.post("/users", async (c) => {
  const body = await c.json();
  return c.json({ id: "123", ...body }, 201);
});

app.listen(3000, () => {
  console.log("Server is running on http://localhost:3000");
});
```

## Middleware

SmashHono includes several built-in middleware functions:

```javascript
import "smashhono";

const app = new SmashHono();

// Logger middleware
app.use(smashhono.middleware.logger());

// CORS middleware
app.use(smashhono.middleware.cors());

// Basic authentication
app.use("/admin/*", smashhono.middleware.basicAuth({
  validate: (username, password) => {
    return username === "admin" && password === "password";
  }
}));

app.get("/", (c) => c.text("Hello, SmashLang!"));
app.get("/admin/dashboard", (c) => c.text("Admin Dashboard"));

app.listen(3000);
```

## REST API

```javascript
import "smashhono";

const app = new SmashHono();

// Define a group of routes with a common prefix
const api = app.basePath("/api");

// API routes
api.get("/users", (c) => {
  return c.json([
    { id: "1", name: "John" },
    { id: "2", name: "Jane" }
  ]);
});

api.get("/users/:id", (c) => {
  const id = c.params.id;
  return c.json({ id, name: `User ${id}` });
});

api.post("/users", async (c) => {
  const body = await c.json();
  return c.json({ id: "3", ...body }, 201);
});

api.put("/users/:id", async (c) => {
  const id = c.params.id;
  const body = await c.json();
  return c.json({ id, ...body });
});

api.delete("/users/:id", (c) => {
  const id = c.params.id;
  return c.text(`User ${id} deleted`, 200);
});

app.listen(3000);
```

## WebSockets

```javascript
import "smashhono";

const app = new SmashHono();

app.get("/", (c) => {
  return c.html(`
    <!DOCTYPE html>
    <html>
      <body>
        <h1>WebSocket Chat</h1>
        <div id="messages"></div>
        <input type="text" id="message" />
        <button onclick="sendMessage()">Send</button>
        
        <script>
          const ws = new WebSocket('ws://' + location.host + '/ws');
          const messages = document.getElementById('messages');
          const messageInput = document.getElementById('message');
          
          ws.onmessage = (event) => {
            const message = document.createElement('div');
            message.textContent = event.data;
            messages.appendChild(message);
          };
          
          function sendMessage() {
            ws.send(messageInput.value);
            messageInput.value = '';
          }
        </script>
      </body>
    </html>
  `);
});

// WebSocket handler
app.ws("/ws", (ws) => {
  const clients = new Set();
  
  ws.on("connection", (socket) => {
    clients.add(socket);
    
    socket.on("message", (message) => {
      // Broadcast to all clients
      for (const client of clients) {
        client.send(message);
      }
    });
    
    socket.on("close", () => {
      clients.delete(socket);
    });
  });
});

app.listen(3000);
```

## GraphQL

```javascript
import "smashhono";

const app = new SmashHono();

// Define GraphQL schema
const schema = `
  type Query {
    hello: String
    users: [User!]!
    user(id: ID!): User
  }
  
  type User {
    id: ID!
    name: String!
    email: String
  }
`;

// Define resolvers
const resolvers = {
  Query: {
    hello: () => "Hello, GraphQL!",
    users: () => [
      { id: "1", name: "John", email: "john@example.com" },
      { id: "2", name: "Jane", email: "jane@example.com" }
    ],
    user: (_, { id }) => {
      return { id, name: `User ${id}`, email: `user${id}@example.com` };
    }
  }
};

// Create GraphQL endpoint
app.graphql("/graphql", schema, resolvers);

// Optional: GraphQL playground
app.get("/playground", (c) => {
  return c.html(`
    <!DOCTYPE html>
    <html>
      <head>
        <title>GraphQL Playground</title>
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/static/css/index.css" />
        <script src="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/static/js/middleware.js"></script>
      </head>
      <body>
        <div id="root"></div>
        <script>
          window.addEventListener('load', function (event) {
            GraphQLPlayground.init(document.getElementById('root'), {
              endpoint: '/graphql'
            })
          })
        </script>
      </body>
    </html>
  `);
});

app.listen(3000);
```

## Error Handling

```javascript
import "smashhono";

const app = new SmashHono();

// Custom error handler
app.onError((err, c) => {
  console.error(`${c.req.method} ${c.req.url}:", ${err}`);
  
  if (err instanceof smashhono.HTTPException) {
    return c.json({ error: err.message }, err.status);
  }
  
  return c.json({ error: "Internal Server Error" }, 500);
});

// Custom 404 handler
app.notFound((c) => {
  return c.json({ error: "Not Found" }, 404);
});

app.get("/error", () => {
  throw new smashhono.HTTPException(400, "Bad Request");
});

app.get("/users/:id", (c) => {
  const id = c.params.id;
  
  if (id === "999") {
    throw new smashhono.HTTPException(404, "User not found");
  }
  
  return c.json({ id, name: `User ${id}` });
});

app.listen(3000);
```

## API Reference

### SmashHono Class

- `new SmashHono(options)`: Create a new SmashHono application
- `app.get(path, ...handlers)`: Add a GET route
- `app.post(path, ...handlers)`: Add a POST route
- `app.put(path, ...handlers)`: Add a PUT route
- `app.delete(path, ...handlers)`: Add a DELETE route
- `app.patch(path, ...handlers)`: Add a PATCH route
- `app.options(path, ...handlers)`: Add an OPTIONS route
- `app.all(path, ...handlers)`: Add a route for all methods
- `app.use(...handlers)`: Add middleware
- `app.basePath(path)`: Create a sub-app with a base path
- `app.route(path, app)`: Mount another SmashHono app at a path
- `app.ws(path, handler)`: Add a WebSocket handler
- `app.graphql(path, schema, resolvers)`: Add a GraphQL endpoint
- `app.notFound(handler)`: Set custom 404 handler
- `app.onError(handler)`: Set custom error handler
- `app.listen(port, options)`: Start the server

### Context Class

- `c.req`: Request object
- `c.res`: Response object
- `c.params`: URL parameters
- `c.query`: Query parameters
- `c.text(body, status, headers)`: Send text response
- `c.json(body, status, headers)`: Send JSON response
- `c.html(body, status, headers)`: Send HTML response
- `c.redirect(url, status)`: Redirect response
- `c.body()`: Get request body as string
- `c.json()`: Get request body as JSON
- `c.formData()`: Get request body as FormData
- `c.header(name)`: Get request header

### Middleware

- `smashhono.middleware.logger()`: Log requests
- `smashhono.middleware.cors(options)`: CORS support
- `smashhono.middleware.basicAuth(options)`: Basic authentication
- `smashhono.middleware.compress()`: Response compression
- `smashhono.middleware.etag()`: ETag support
- `smashhono.middleware.jwt(options)`: JWT authentication

### Other

- `smashhono.serve(options)`: Static file serving
- `smashhono.validator.validate(schema)`: Request validation
- `smashhono.HTTPException`: Custom HTTP error

## License

MIT
