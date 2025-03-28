# PocketBase Client for SmashLang

A comprehensive client library for interacting with [PocketBase](https://pocketbase.io) servers from SmashLang applications.

## Installation

```bash
smashpkg install pocketbase
```

## Features

- **Authentication**: Email/password login, OAuth2, token refresh
- **CRUD Operations**: Create, read, update, and delete collection records
- **File Handling**: Upload files and generate URLs
- **Realtime Subscriptions**: Subscribe to collection changes
- **Admin API**: Manage collections, fields, users, and settings

## Basic Usage

```js
import "pocketbase";

// Create a client
const pb = pocketbase.createClient("http://127.0.0.1:8090");

// Authentication
async fn login() {
  try {
    const authData = await pb.authWithPassword("users", "user@example.com", "password123");
    console.log("Logged in as:", authData.model.email);
  } catch (error) {
    console.error("Login failed:", error);
  }
}

// Create a record
async fn createTask() {
  try {
    const data = {
      title: "Learn SmashLang",
      description: "Master the SmashLang programming language",
      status: "in-progress"
    };
    
    const record = await pb.createRecord("tasks", data);
    console.log("Created task:", record);
  } catch (error) {
    console.error("Failed to create task:", error);
  }
}

// Realtime subscriptions
fn subscribeToTasks() {
  pb.subscribe("tasks", (event) => {
    console.log(`Task ${event.action}:`, event.record);
  });
}
```

## Examples

Check out the [examples directory](./examples) for more detailed examples:

- **[Basic Authentication](./examples/auth.smash)**: User login, logout, and token management
- **[CRUD Operations](./examples/crud.smash)**: Working with collection records
- **[Realtime Subscriptions](./examples/realtime.smash)**: Live updates from the server
- **[File Handling](./examples/files.smash)**: Uploading and managing files
- **[Admin Operations](./examples/admin.smash)**: Collection and user management

## API Reference

### Client

```js
const client = pocketbase.createClient(url);
```

#### Authentication Methods

- `authWithPassword(collection, email, password)`: Authenticate with email/password
- `authWithOAuth2(provider, code, codeVerifier, redirectUrl)`: OAuth2 authentication
- `refreshAuth()`: Refresh the auth token
- `logout()`: Log out the current user

#### Record Methods

- `getCollection(collectionIdOrName)`: Get collection details
- `getCollectionList()`: List all collections
- `getRecord(collection, id)`: Get a single record
- `getRecordList(collection, page, perPage, filter)`: List records with filtering
- `createRecord(collection, data)`: Create a new record
- `updateRecord(collection, id, data)`: Update an existing record
- `deleteRecord(collection, id)`: Delete a record

#### File Methods

- `uploadFile(collection, id, fieldName, fileData, filename)`: Upload a file
- `getFileUrl(record, filename, queryParams)`: Get a file's URL

#### Realtime Methods

- `subscribe(collection, callback)`: Subscribe to collection changes
- `unsubscribe(collection)`: Unsubscribe from collection changes

### Admin

```js
const admin = new pocketbase.Admin(client);
```

#### Collection Management

- `createCollection(data)`: Create a new collection
- `updateCollection(collectionId, data)`: Update a collection
- `deleteCollection(collectionId)`: Delete a collection

#### Field Management

- `createField(collectionId, data)`: Create a new field
- `updateField(collectionId, fieldId, data)`: Update a field
- `deleteField(collectionId, fieldId)`: Delete a field

#### User Management

- `listUsers(page, perPage)`: List users
- `createUser(data)`: Create a new user
- `updateUser(userId, data)`: Update a user
- `deleteUser(userId)`: Delete a user

## License

MIT
