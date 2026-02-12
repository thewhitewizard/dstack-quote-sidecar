# Dstack Quote Sidecar

A lightweight HTTP sidecar service for interacting with [Dstack TEE](https://github.com/Dstack-TEE/dstack) to generate attestation quotes and perform attestation operations.

## Overview

This service provides a REST API interface to the Dstack SDK, enabling easy integration with Trusted Execution Environment (TEE) attestation capabilities. It allows applications to generate cryptographic quotes and attestations for secure computation verification.

## Features

- 🔐 **Quote Generation**: Generate TEE quotes with custom data
- ✅ **Attestation**: Create attestation proofs for application state
- 📊 **RTMR Replay**: Automatic replay of Runtime Measurement Registers from event logs
- 🚀 **Fast & Lightweight**: Built with Axum for high-performance async operations
- 📝 **JSON API**: Simple REST endpoints with JSON responses
- 🔍 **Health Checks**: Built-in health monitoring endpoints

## Tech Stack

- **Rust**
- **Axum** - Web framework
- **Dstack SDK** - TEE attestation library
- **Tokio** - Async runtime
- **Serde** - JSON serialization

## Configuration

The service can be configured using environment variables:

| Variable                      | Description         | Default   |
|-------------------------------|---------------------|-----------|
| `QUOTE_SIDECAR__SERVER__HOST` | Server bind address | `0.0.0.0` |
| `QUOTE_SIDECAR__SERVER__PORT` | Server port         | `9999`    |

### Example

```bash
export QUOTE_SIDECAR__SERVER__HOST=127.0.0.1
export QUOTE_SIDECAR__SERVER__PORT=8080
```

## Usage

### Running the Service

```bash
# Development mode
cargo run

# Production mode (release build)
cargo run --release
```

The server will start on `http://0.0.0.0:9999` by default.

## API Endpoints

### 1. Root Endpoint

**`GET /`**

Returns service information and current timestamp.

**Response:**

```json
{
  "service": "dstack-quote-sidecar",
  "timestamp": "2026-02-12T09:30:45.123456Z"
}
```

### 2. Health Check

**`GET /health`**

Health check endpoint for monitoring.

**Response:**

```json
{
  "status": "ok"
}
```

### 3. Generate Quote

**`GET /quote`**

Generates a TEE quote for the provided data and replays RTMRs from the event log.

**Query Parameters:**

- `data` (optional): Custom data to include in the quote. Defaults to `"hello world"` if not provided.

**Examples:**

```bash
# With default data
curl http://localhost:9999/quote

# With custom data
curl "http://localhost:9999/quote?data=user:alice:nonce123"
```

**Success Response:**

```json
{
  "quote": "Quote { ... }",
  "rtmrs": "Rtmrs { ... }"
}
```

**Error Response:**

```json
{
  "error": "Failed to get quote: ..."
}
```

### 4. Generate Attestation

**`GET /attest`**

Generates an attestation quote for the provided application state.

**Query Parameters:**

- `data` (optional): Custom data to include in the attestation. Defaults to `"hello world"` if not provided.

**Examples:**

```bash
# With default data
curl http://localhost:9999/attest

# With custom data
curl "http://localhost:9999/attest?data=my-app-state"
```

**Success Response:**

```json
{
  "attestation": "eyJ0eXAiOiJKV1QiLCJhbGc..."
}
```

**Error Response:**

```json
{
  "error": "Failed to attest: ..."
}
```

## Project Structure

```text
dstack-quote-sidecar/
├── src/
│   ├── main.rs           # Application entry point
│   ├── application.rs    # Application setup and routing
│   ├── config.rs         # Configuration management
│   ├── handlers.rs       # HTTP request handlers
│   └── errors.rs         # Error types
├── Cargo.toml            # Project dependencies
└── README.md             # This file
```

## Development with Simulator

For local development without TDX hardware, use the Dstack simulator:

### 1. Clone and Build the Simulator

```bash
git clone https://github.com/Dstack-TEE/dstack.git
cd dstack/sdk/simulator
./build.sh
```

### 2. Configure the Simulator

**Important:** The simulator needs to expose the internal API on HTTP instead of Unix sockets. Edit `dstack.toml`:

```toml
[internal]
address = "0.0.0.0:8090"
reuse = true
```

### 3. Start the Simulator

```bash
./dstack-simulator
```

The simulator will now listen on `http://0.0.0.0:8090`.

### 4. Run the Sidecar

In a separate terminal:

```bash
cd /path/to/dstack-quote-sidecar
export DSTACK_SIMULATOR_ENDPOINT=http://localhost:8090
cargo run
```

### 5. Test the Endpoints

```bash
# Test quote endpoint
curl "http://localhost:9999/quote?data=test123"

# Test attestation endpoint
curl "http://localhost:9999/attest?data=my-app-state"
```

## Development

### Run Tests

```bash
cargo test
```

### Check Code

```bash
cargo check
```

### Format Code

```bash
cargo fmt
```

### Lint

```bash
cargo clippy
```

## Logging

The service uses `tracing` for structured logging. Set the `RUST_LOG` environment variable to control log levels:

```bash
# Debug level
RUST_LOG=debug cargo run

# Info level (default)
RUST_LOG=info cargo run

# Trace level (verbose)
RUST_LOG=trace cargo run
```

## Graceful Shutdown

The service handles graceful shutdown on:

- `CTRL+C` (SIGINT)
- `SIGTERM` (Unix-like systems)

## License

MIT License - See LICENSE file for details

## Related Projects

- [Dstack TEE](https://github.com/Dstack-TEE/dstack) - The underlying TEE attestation framework
