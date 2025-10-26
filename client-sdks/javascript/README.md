# IsolateSandbox JavaScript/TypeScript SDK

A TypeScript/JavaScript client SDK for the [IsolateSandbox API](../../README.md) - a secure sandboxed code execution service using Linux Isolate.

## Features

- 🔒 Execute code securely in isolated sandboxes
- 🌐 Works in Node.js, Bun, and modern browsers
- 📦 Full TypeScript support with type definitions
- ⚡ Modern async/await API using native fetch
- 🎯 Zero dependencies
- 🛡️ Comprehensive error handling

## Installation

```bash
# Using npm
npm install @isolate-sandbox/client

# Using yarn
yarn add @isolate-sandbox/client

# Using pnpm
pnpm add @isolate-sandbox/client

# Using bun
bun add @isolate-sandbox/client
```

## Quick Start

```typescript
import { IsolateSandboxClient } from '@isolate-sandbox/client';

// Create a client instance
const client = new IsolateSandboxClient({
  baseUrl: 'http://localhost:3000',
  // Optional: Provide API key if your server requires authentication
  apiKey: process.env.ISOLATE_SANDBOX_API_KEY,
});

// Execute some Python code
const result = await client.execute({
  language: 'python',
  code: 'print("Hello from sandbox!")',
});

console.log(result.stdout); // "Hello from sandbox!\n"
console.log(result.metadata.time); // Execution time in seconds
console.log(result.box_id); // Sandbox box ID used
```

## API Reference

### Constructor

```typescript
const client = new IsolateSandboxClient(config: IsolateSandboxConfig);
```

**Configuration Options:**

- `baseUrl` (string, required): Base URL of the IsolateSandbox API
- `timeout` (number, optional): Request timeout in milliseconds (default: 30000)
- `apiKey` (string, optional): API key for authentication (sent as `X-API-Key` header)

### Methods

#### `health()`

Check the health status of the service.

```typescript
const health = await client.health();
console.log(health.status); // "ok" or "error"
```

**Returns:** `Promise<HealthResponse>`

#### `listLanguages()`

Get a list of all supported programming languages.

```typescript
const languages = await client.listLanguages();
console.log(languages.languages); // ["python", "javascript", "rust", ...]
```

**Returns:** `Promise<LanguagesResponse>`

#### `execute(request)`

Execute code in a sandboxed environment.

```typescript
const result = await client.execute({
  language: 'python',
  code: 'print("Hello, World!")',
});

console.log(result.stdout); // Standard output
console.log(result.stderr); // Standard error
console.log(result.metadata); // Execution metadata
console.log(result.box_id); // Sandbox box ID
```

**Parameters:**
- `request.language` (string): Programming language to use
- `request.code` (string): Source code to execute

**Returns:** `Promise<ExecuteResponse>`

**Response includes:**
- `stdout` (string): Standard output from execution
- `stderr` (string): Standard error from execution
- `metadata`: Execution metadata
  - `time` (number): Execution time in seconds
  - `time_wall` (number): Wall clock time in seconds
  - `memory` (number): Memory usage in bytes
  - `exit_code` (number): Process exit code
  - `status` (string): Execution status
- `box_id` (number): Box ID used for execution

#### `listBoxFiles(boxId)`

List all files in a sandbox box.

```typescript
const files = await client.listBoxFiles(1);
console.log(files.files); // ["output.txt", "data.json", ...]
```

**Parameters:**
- `boxId` (number): Box ID to list files from

**Returns:** `Promise<BoxFilesResponse>`

#### `getBoxFile(boxId, filename)`

Get a file from a sandbox box (automatically decodes from base64).

```typescript
const file = await client.getBoxFile(1, 'output.txt');
console.log(file.content); // Decoded file content
console.log(file.filename); // "output.txt"
```

**Parameters:**
- `boxId` (number): Box ID to get file from
- `filename` (string): Name of the file to retrieve

**Returns:** `Promise<{ content: string; filename: string }>`

#### `getBoxFileRaw(boxId, filename)`

Get a file from a sandbox box as raw base64.

```typescript
const file = await client.getBoxFileRaw(1, 'output.txt');
console.log(file.content); // Base64 encoded content
console.log(file.filename); // "output.txt"
```

**Parameters:**
- `boxId` (number): Box ID to get file from
- `filename` (string): Name of the file to retrieve

**Returns:** `Promise<BoxFileResponse>`

#### `cleanupBox(boxId)`

Cleanup a sandbox box and release it back to the pool.

```typescript
const result = await client.cleanupBox(1);
console.log(result.message); // "Box 1 cleaned up successfully"
```

**Parameters:**
- `boxId` (number): Box ID to cleanup

**Returns:** `Promise<CleanupResponse>`

## Authentication

If your IsolateSandbox API server requires authentication, you can provide an API key:

```typescript
const client = new IsolateSandboxClient({
  baseUrl: 'http://localhost:3000',
  apiKey: 'your-api-key-here',
});
```

The API key will be automatically sent as the `X-API-Key` header with all requests (except `/health` endpoint which is typically unauthenticated).

**Setting up API Key on the Server:**

Set the `ISOLATE_SANDBOX_API_KEY` environment variable on your server:

```bash
export ISOLATE_SANDBOX_API_KEY="your-secret-api-key"
```

**Common Authentication Errors:**

- **403 Forbidden**: The `X-API-Key` header is missing. Make sure you provide the `apiKey` option when creating the client.
- **401 Unauthorized**: The API key is invalid. Verify that the key matches the one configured on the server.

## Error Handling

The SDK throws `IsolateSandboxError` for all API errors.

```typescript
import { IsolateSandboxClient, IsolateSandboxError } from '@isolate-sandbox/client';

try {
  const result = await client.execute({
    language: 'invalid-language',
    code: 'print("test")',
  });
} catch (error) {
  if (error instanceof IsolateSandboxError) {
    console.error('API Error:', error.message);
    console.error('Status Code:', error.statusCode);
    console.error('Response:', error.response);
  } else {
    console.error('Unexpected error:', error);
  }
}
```

**IsolateSandboxError Properties:**
- `message` (string): Error message
- `statusCode` (number): HTTP status code
- `response` (ErrorResponse | undefined): Raw error response from API

## Examples

See the [examples](./examples) directory for more detailed usage examples:

- [basic-usage.ts](./examples/basic-usage.ts) - Simple code execution
- [box-management.ts](./examples/box-management.ts) - Working with sandbox boxes
- [error-handling.ts](./examples/error-handling.ts) - Handling errors

## Requirements

- Node.js >= 18.0.0 (for native fetch support)
- OR Bun >= 1.0.0
- TypeScript >= 5.0.0 (if using TypeScript)

## License

MIT
