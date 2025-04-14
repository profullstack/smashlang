# HTTP Package

<p align="center">
  <img src="../../../smashlang_packages/networking/http/assets/logo.light.svg" alt="HTTP Package Logo" width="200" />
</p>

The HTTP package provides a comprehensive HTTP client for SmashLang applications. It allows you to make HTTP requests, handle responses, and work with various HTTP features like headers, cookies, and authentication.

## Installation

```bash
smashpkg install http
```

## Features

- Support for all HTTP methods (GET, POST, PUT, DELETE, etc.)
- Request and response headers management
- Query parameters and URL handling
- Request body in various formats (JSON, form data, etc.)
- File uploads and downloads
- Cookie management
- Authentication (Basic, Bearer, OAuth)
- HTTPS support
- Timeout and retry mechanisms
- Response parsing (JSON, text, binary)
- Streaming responses
- Proxy support

## Basic Usage

```js
import { http } from "http";

// Simple GET request
const response = await http.get("https://api.example.com/users");
const users = response.json();
console.log(users);

// POST request with JSON body
const newUser = {
  name: "John Doe",
  email: "john@example.com"
};

const createResponse = await http.post("https://api.example.com/users", {
  headers: {
    "Content-Type": "application/json"
  },
  body: JSON.stringify(newUser)
});

console.log(createResponse.status); // 201
console.log(await createResponse.json()); // { id: 123, name: "John Doe", ... }
```

## Advanced Usage

### Request Configuration

```js
import { http } from "http";

// Configure a request with various options
const response = await http.request("https://api.example.com/data", {
  method: "POST",
  headers: {
    "Content-Type": "application/json",
    "Authorization": "Bearer token123",
    "Accept": "application/json"
  },
  body: JSON.stringify({ key: "value" }),
  timeout: 5000, // 5 seconds
  retries: 3,
  retryDelay: 1000, // 1 second between retries
  followRedirects: true,
  maxRedirects: 5,
  validateStatus: (status) => status < 500, // Don't reject on 4xx errors
  proxy: "http://proxy.example.com:8080"
});

// Check response status
if (response.ok) {
  const data = await response.json();
  console.log(data);
} else {
  console.error(`Error: ${response.status} ${response.statusText}`);
}
```

### Working with Forms

```js
import { http } from "http";

// Send form data
const formData = new FormData();
formData.append("username", "johndoe");
formData.append("password", "secret");
formData.append("avatar", fileBlob, "avatar.png");

const response = await http.post("https://api.example.com/login", {
  body: formData
});

// The Content-Type header is automatically set to multipart/form-data
console.log(await response.json());
```

### Handling Cookies

```js
import { http, CookieJar } from "http";

// Create a cookie jar to store cookies between requests
const cookieJar = new CookieJar();

// First request - server sets cookies
const loginResponse = await http.post("https://api.example.com/login", {
  body: JSON.stringify({ username: "user", password: "pass" }),
  headers: { "Content-Type": "application/json" },
  cookieJar: cookieJar
});

// Second request - cookies are automatically sent
const profileResponse = await http.get("https://api.example.com/profile", {
  cookieJar: cookieJar
});

console.log(await profileResponse.json());
```

### Streaming Responses

```js
import { http } from "http";
import { fs } from "fs";

// Download a large file with streaming
const response = await http.get("https://example.com/large-file.zip", {
  stream: true
});

const fileStream = fs.createWriteStream("downloaded-file.zip");
const reader = response.body.getReader();

while (true) {
  const { done, value } = await reader.read();
  if (done) break;
  fileStream.write(value);
}

fileStream.end();
console.log("Download complete");
```

## API Reference

### HTTP Methods

#### `http.request(url, options)`
Makes an HTTP request with the specified options.
- **Parameters**: 
  - `url` (String): The URL to request
  - `options` (Object): Request options
    - `method` (String): HTTP method (GET, POST, etc.)
    - `headers` (Object): Request headers
    - `body` (String|Object|FormData): Request body
    - `timeout` (Number): Request timeout in milliseconds
    - `retries` (Number): Number of retry attempts
    - `retryDelay` (Number): Delay between retries in milliseconds
    - `followRedirects` (Boolean): Whether to follow redirects
    - `maxRedirects` (Number): Maximum number of redirects to follow
    - `validateStatus` (Function): Function to determine if status is valid
    - `proxy` (String): Proxy server URL
    - `cookieJar` (CookieJar): Cookie jar for storing cookies
    - `stream` (Boolean): Whether to return a streaming response
- **Returns**: (Promise<Response>) Promise resolving to a Response object

#### `http.get(url, options)`
Makes a GET request.
- **Parameters**: 
  - `url` (String): The URL to request
  - `options` (Object): Request options (same as `request`)
- **Returns**: (Promise<Response>) Promise resolving to a Response object

#### `http.post(url, options)`
Makes a POST request.
- **Parameters**: 
  - `url` (String): The URL to request
  - `options` (Object): Request options (same as `request`)
- **Returns**: (Promise<Response>) Promise resolving to a Response object

#### `http.put(url, options)`
Makes a PUT request.
- **Parameters**: 
  - `url` (String): The URL to request
  - `options` (Object): Request options (same as `request`)
- **Returns**: (Promise<Response>) Promise resolving to a Response object

#### `http.delete(url, options)`
Makes a DELETE request.
- **Parameters**: 
  - `url` (String): The URL to request
  - `options` (Object): Request options (same as `request`)
- **Returns**: (Promise<Response>) Promise resolving to a Response object

### Response Object

#### `response.status`
The HTTP status code of the response.
- **Type**: (Number)

#### `response.statusText`
The HTTP status text of the response.
- **Type**: (String)

#### `response.ok`
Whether the response status is in the successful range (200-299).
- **Type**: (Boolean)

#### `response.headers`
The headers of the response.
- **Type**: (Object)

#### `response.body`
The body of the response as a ReadableStream (if streaming).
- **Type**: (ReadableStream)

#### `response.text()`
Gets the response body as text.
- **Returns**: (Promise<String>) Promise resolving to the response text

#### `response.json()`
Gets the response body as a JSON object.
- **Returns**: (Promise<Object>) Promise resolving to the parsed JSON

#### `response.blob()`
Gets the response body as a Blob.
- **Returns**: (Promise<Blob>) Promise resolving to a Blob

#### `response.arrayBuffer()`
Gets the response body as an ArrayBuffer.
- **Returns**: (Promise<ArrayBuffer>) Promise resolving to an ArrayBuffer

### CookieJar Class

#### `new CookieJar()`
Creates a new cookie jar for storing cookies.
- **Returns**: (CookieJar) New CookieJar instance

#### `cookieJar.setCookie(cookie, url)`
Sets a cookie in the jar.
- **Parameters**: 
  - `cookie` (String): Cookie string
  - `url` (String): URL the cookie is associated with
- **Returns**: (void)

#### `cookieJar.getCookies(url)`
Gets cookies for a URL.
- **Parameters**: 
  - `url` (String): URL to get cookies for
- **Returns**: (Array<String>) Array of cookie strings

## Examples

See the [examples directory](../../../smashlang_packages/networking/http/examples) for more detailed examples:

- [Basic Example](../../../smashlang_packages/networking/http/examples/basic.smash): Demonstrates simple HTTP requests
- [Authentication Example](../../../smashlang_packages/networking/http/examples/auth.smash): Shows authentication methods
- [File Upload Example](../../../smashlang_packages/networking/http/examples/upload.smash): Demonstrates file uploads
- [Streaming Example](../../../smashlang_packages/networking/http/examples/streaming.smash): Shows streaming responses

## Testing

The HTTP package includes comprehensive tests:

```bash
# Run all tests for the http package
smashtest smashlang_packages/networking/http/tests
```

## Contributing

Contributions to the HTTP package are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for your changes
5. Submit a pull request

## License

MIT