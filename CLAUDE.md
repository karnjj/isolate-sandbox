# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is an **isolate-sandbox** service - a Rust-based web service that provides secure code execution using the `isolate` sandbox tool. It's designed to safely execute untrusted code in isolated environments with proper resource limits and security controls.

The project includes both a server API and a JavaScript/TypeScript client SDK.

## Development Commands

### Important: Use orbctl
This project uses `orbctl` for command execution. Always prefix commands with `orbctl run`:

- **Build**: `orbctl run cargo build`
- **Run with logging**: `orbctl run RUST_LOG=info cargo run`
- **Build and run with sudo**: `orbctl run cargo build && orbctl run sudo RUST_LOG=info ./target/debug/isolate-sandbox`
- **Run tests**: `orbctl run cargo test`
- **Format code**: `orbctl run cargo fmt`
- **Run linter**: `orbctl run cargo clippy`

### Docker Commands
- **Build image**: `make docker-build` (builds for linux/amd64)
- **Run container**: `make docker-run` (requires privileged mode for isolate)

### Client SDK Commands
- **Run JavaScript examples**: `cd client-sdks/javascript && bun run example:basic`
- **Run box management example**: `cd client-sdks/javascript && bun run example:box`
- **Run error handling example**: `cd client-sdks/javascript && bun run example:error`

### Configuration
The service uses environment variables for configuration:

#### Server Configuration
- `ISOLATE_SANDBOX_PORT`: Server port (default: 3000)
- `ISOLATE_SANDBOX_CONFIG_DIR`: Configuration directory (default: ./config)
- `ISOLATE_SANDBOX_BOX_POOL_SIZE`: Number of sandbox boxes to pool (default: 10)
- `ISOLATE_SANDBOX_API_KEY`: Optional API key for authentication

#### Sandbox Resource Limits
- `ISOLATE_SANDBOX_DEFAULT_CG_MEM`: Memory limit in KB (default: 524288, 0 = unlimited)
- `ISOLATE_SANDBOX_DEFAULT_MEM`: Memory limit in KB (default: 512000, 0 = unlimited)
- `ISOLATE_SANDBOX_DEFAULT_TIME`: Time limit in seconds (default: 30, 0 = unlimited)
- `ISOLATE_SANDBOX_DEFAULT_WALL_TIME`: Wall time limit in seconds (default: 60, 0 = unlimited)
- `ISOLATE_SANDBOX_DEFAULT_EXTRA_TIME`: Extra time in seconds (default: 10, 0 = unlimited)
- `ISOLATE_SANDBOX_DEFAULT_STACK`: Stack limit in KB (default: 128000, 0 = unlimited)
- `ISOLATE_SANDBOX_DEFAULT_FSIZE`: File size limit in KB (default: 102400, 0 = unlimited)
- `ISOLATE_SANDBOX_DEFAULT_OPEN_FILES`: Open files limit (default: 64, 0 = unlimited)
- `ISOLATE_SANDBOX_DEFAULT_PROCESSES`: Processes limit (default: 0 = unlimited, uses `--processes` without value)

**Note**: Setting any resource limit to 0 disables that limit (unlimited). For processes, unlimited uses the `--processes` flag without a numeric value.

## Architecture

The project follows Clean Architecture principles with these layers:

### Domain Layer (`src/domain/`)
- **Entities**: Core business objects (`Language`, `ExecutionRequest`, `ExecutionResult`)
- **Services**: Business logic interfaces (`CompilerService`, `SandboxService`)
- **Repositories**: Data access interfaces (`LanguageRepository`)
- **Error Handling**: Domain-specific error types

### External Layer (`src/external/`)
- **Infrastructure implementations**: `IsolateSandboxService`, `CompilerServiceImpl`
- **File system operations**: `FileSystem`
- **Process execution**: `ProcessExecutor` (for running isolate commands)
- **Box pool management**: `BoxPool` (manages sandbox box IDs)

### Use Cases Layer (`src/use_cases/`)
- **Application logic**: `ExecuteCodeUseCase`, `ListLanguagesUseCase`, etc.
- **Orchestrates domain and external services**

### Adapters Layer (`src/adapters/`)
- **Web API**: Axum-based HTTP server with OpenAPI/Swagger documentation
- **Handlers**: HTTP request/response handling
- **Middleware**: CORS, logging, authentication

### Client SDK (`client-sdks/javascript/`)
- **TypeScript/JavaScript client**: Modern async/await API using native fetch
- **Zero dependencies**: Works in Node.js, Bun, and modern browsers
- **Full TypeScript support**: Type definitions and comprehensive error handling
- **Examples**: Basic usage, box management, and error handling examples

### Key Features

1. **Secure Code Execution**: Uses `isolate` sandbox tool for secure execution
2. **Multiple Language Support**: Configurable languages with compiler/runner scripts
3. **Resource Management**: Box pooling, file cleanup, memory/time limits
4. **API Documentation**: Auto-generated OpenAPI specs with Swagger UI
5. **File Management**: Post-execution file inspection and cleanup
6. **Client SDK**: Easy-to-use TypeScript/JavaScript client library

### Important Security Notes

- **Requires sudo**: The isolate tool requires privileged access for sandbox creation
- **Box Management**: Each execution gets a unique box ID that must be explicitly cleaned up
- **File Access**: Files remain accessible after execution for inspection until cleanup
- **Resource Limits**: Configurable memory, time, file size, open files, and process limits via environment variables (set to 0 for unlimited)

### Development Workflow

1. Language configurations are stored in `config/` directory
2. Each language needs: `setup.sh`, `compiler`, and `runner` scripts
3. The service validates isolate installation on startup
4. Box IDs are pooled and recycled to manage resource usage
5. Execution metadata (time, memory, exit code) is parsed from isolate output
6. Client SDK provides convenient interface to the API

### API Endpoints

- `GET /health`: Health check (no authentication required)
- `GET /languages`: List available languages (requires API key)
- `POST /execute`: Execute code in sandbox (requires API key)
- `GET /boxes/{box_id}/files`: List files in sandbox box (requires API key)
- `GET /boxes/{box_id}/files/{filename}`: Get file content (base64) (requires API key)
- `DELETE /boxes/{box_id}`: Clean up sandbox box (requires API key)

### Client SDK Usage

```typescript
import { IsolateSandboxClient } from '@isolate-sandbox/client';

const client = new IsolateSandboxClient({
  baseUrl: 'http://localhost:3000',
  apiKey: process.env.ISOLATE_SANDBOX_API_KEY,
});

// Execute code
const result = await client.execute({
  language: 'python',
  code: 'print("Hello from sandbox!")',
});

// Manage sandbox boxes
const files = await client.listBoxFiles(result.box_id);
const fileContent = await client.getBoxFile(result.box_id, 'output.txt');
await client.cleanupBox(result.box_id);
```

### Authentication

- Optional API key authentication via `X-API-Key` header
- Health endpoint is always accessible
- All other endpoints require valid API key when configured on server
- Client SDK automatically handles API key in all requests

### Testing

The service includes integration tests for the API endpoints. Tests require proper isolate setup and sudo access. The client SDK includes examples that can be used for manual testing.