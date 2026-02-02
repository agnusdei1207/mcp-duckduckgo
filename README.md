# MCP Web Search Server (Rust)

High-performance Model Context Protocol (MCP) server providing web search functionality using DuckDuckGo HTML scraping.

## Features

- **Free & Unlimited**: No API keys required
- **Privacy-Focused**: Uses DuckDuckGo, a privacy-focused search engine
- **High Performance**: Rust implementation for speed and efficiency
- **Large Result Sets**: Supports fetching up to 9,999 results
- **Modular Architecture**: Clean separation of concerns
- **Zero System Dependencies**: Uses rustls instead of OpenSSL

## Project Structure

```
rust-mcp/
├── Cargo.toml              # Project manifest (edition = "2024")
├── Dockerfile              # Docker 1.92 image build
├── mcp-websearch           # Pre-built binary (from Docker)
├── src/
│   ├── main.rs             # Entry point
│   ├── lib.rs              # Library exports
│   ├── models/             # Data models
│   │   ├── mod.rs
│   │   └── tests.rs        # Model unit tests
│   ├── search/             # Search implementation
│   │   ├── mod.rs
│   │   └── tests.rs        # Search unit tests
│   └── mcp/                # MCP protocol
│       ├── mod.rs
│       └── tests.rs        # MCP unit tests
└── tests/                  # E2E tests
    └── e2e_tests.rs
```

## Building

### Docker Build (REQUIRED)

All builds must be done via Docker using Rust 1.92:

```bash
# Build Docker image
docker build -t mcp-websearch:latest .

# Extract the binary for local use
docker create --name temp mcp-websearch:latest
docker cp temp:/app/mcp-websearch ./mcp-websearch
docker rm temp
chmod +x ./mcp-websearch
```

### Push to Docker Hub

```bash
# Login to Docker Hub (first time only)
docker login

# Tag your image
docker tag mcp-websearch:latest <your-dockerhub-username>/mcp-websearch:latest

# Push to Docker Hub
docker push <your-dockerhub-username>/mcp-websearch:latest
```

Example:
```bash
docker tag mcp-websearch:latest myusername/mcp-websearch:latest
docker push myusername/mcp-websearch:latest
```

### Pull from Docker Hub

```bash
# Pull the image
docker pull <your-dockerhub-username>/mcp-websearch:latest

# Run the MCP server
docker run --rm -i <your-dockerhub-username>/mcp-websearch:latest
```

## Running Locally

### Option 1: Using the Extracted Binary

```bash
./mcp-websearch
```

The server will start and listen for JSON-RPC requests on stdin/stdout.

### Option 2: Using Docker (Auto-start)

```bash
# Interactive mode (for testing)
docker run --rm -i mcp-websearch:latest

# The MCP server starts automatically via ENTRYPOINT
```

## Usage with Claude Code

### Step 1: Locate Claude Code Config

Claude Code configuration file location:
- **Linux**: `~/.config/claude-code/config.json`
- **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`

### Step 2: Add MCP Server Configuration

Using the **extracted binary**:
```json
{
  "mcpServers": {
    "websearch": {
      "command": "/home/lepisode/workspace/mcp-websearch/rust-mcp/mcp-websearch"
    }
  }
}
```

Using **Docker** (recommended):
```json
{
  "mcpServers": {
    "websearch": {
      "command": "docker",
      "args": ["run", "--rm", "-i", "mcp-websearch:latest"]
    }
  }
}
```

Using **Docker Hub image**:
```json
{
  "mcpServers": {
    "websearch": {
      "command": "docker",
      "args": ["run", "--rm", "-i", "your-username/mcp-websearch:latest"]
    }
  }
}
```

### Step 3: Restart Claude Code

After updating the configuration, restart Claude Code to load the MCP server.

### Step 4: Verify Installation

In Claude Code, you can now use the `web_search` tool:

```
Search for "latest Rust programming news" and return 5 results.
```

## Tool: web_search

Searches the web using DuckDuckGo.

### Parameters

- `query` (required): Search query string
- `limit` (optional): Number of results (1-9999, default: 10)
- `offset` (optional): Pagination offset (default: 0)

### Response

```json
{
  "query": "search query",
  "results": [
    {
      "title": "Result title",
      "url": "https://example.com",
      "snippet": "Brief description"
    }
  ],
  "totalResults": 100,
  "returned": 10,
  "offset": 0
}
```

## Testing

```bash
# Run all tests
docker run --rm -w /app mcp-websearch:latest cargo test

# Run unit tests only
docker run --rm -w /app mcp-websearch:latest cargo test --lib

# Run E2E tests
docker run --rm -w /app mcp-websearch:latest cargo test --test e2e_tests
```

## Manual Testing (stdin/stdout)

You can test the MCP server manually by sending JSON-RPC requests:

```bash
# Start the server
./mcp-websearch

# In another terminal, send a request
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | ./mcp-websearch

# Or test web search
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"web_search","arguments":{"query":"Rust","limit":3}}}' | ./mcp-websearch
```

## Performance

- **Binary Size**: ~3MB (stripped, rustls)
- **Memory Usage**: ~2-5MB
- **Startup Time**: <10ms
- **Search Speed**: ~500ms per page
- **Docker Image**: ~70MB (debian base)

## Technical Details

- **Rust Edition**: 2024
- **Docker Base**: rust:1.92-slim
- **TLS**: rustls (no OpenSSL dependency)
- **Async Runtime**: tokio
- **HTML Parsing**: scraper
- **HTTP Client**: reqwest with rustls-tls

## License

MIT
# mcp-websearch
