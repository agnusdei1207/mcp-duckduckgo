# MCP Web Search Server (Rust)

High-performance Model Context Protocol (MCP) server providing web search functionality using DuckDuckGo HTML scraping.

## Features

- **Free & Unlimited**: No API keys required
- **Privacy-Focused**: Uses DuckDuckGo, a privacy-focused search engine
- **High Performance**: Rust implementation for speed and efficiency
- **Large Result Sets**: Supports fetching up to 9,999 results
- **Modular Architecture**: Clean separation of concerns
- **Zero System Dependencies**: Uses rustls instead of OpenSSL
- **Ultra-Small Docker**: ~10MB Alpine-based image

---

## Quick Start (Docker Hub)

### Pull & Test

```bash
# Pull the image
docker pull agnusdei1207/mcp-websearch:latest

# Quick test
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | docker run --rm -i agnusdei1207/mcp-websearch:latest

# Web search test
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"web_search","arguments":{"query":"Rust programming","limit":3}}}' | docker run --rm -i agnusdei1207/mcp-websearch:latest
```

---

## Claude Code Integration

### Config File Location

- **Linux**: `~/.config/claude-code/config.json`
- **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`

### Add MCP Server

```json
{
  "mcpServers": {
    "websearch": {
      "command": "docker",
      "args": ["run", "--rm", "-i", "agnusdei1207/mcp-websearch:latest"]
    }
  }
}
```

If you have other MCP servers:
```json
{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/allowed/path"]
    },
    "websearch": {
      "command": "docker",
      "args": ["run", "--rm", "-i", "agnusdei1207/mcp-websearch:latest"]
    }
  }
}
```

### Restart & Verify

1. Close and reopen Claude Code
2. Try: `Search for "Claude AI" with limit 5`

---

## Tool: web_search

**Parameters:**
- `query` (required): Search query string
- `limit` (optional): 1-9999 results (default: 10)
- `offset` (optional): Pagination offset (default: 0)

---

## Building from Source

```bash
docker build -t mcp-websearch:latest .
```

### Extract Binary

```bash
docker create --name temp mcp-websearch:latest
docker cp temp:/app/mcp-websearch ./mcp-websearch
docker rm temp
chmod +x ./mcp-websearch
```

### Push to Docker Hub

```bash
docker login
docker tag mcp-websearch:latest agnusdei1207/mcp-websearch:latest
docker push agnusdei1207/mcp-websearch:latest
```

---

## Project Structure

```
mcp-websearch/
├── Cargo.toml              # Project manifest (edition = "2024")
├── Dockerfile              # Alpine-based multi-stage build
├── README.md               # This file
├── src/
│   ├── main.rs             # Entry point
│   ├── lib.rs              # Library exports
│   ├── models/             # Data models + tests
│   ├── search/             # DuckDuckGo scraper + tests
│   └── mcp/                # MCP protocol + tests
└── tests/                  # E2E tests
    └── e2e_tests.rs
```

---

## Testing

```bash
# MCP tools list
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | docker run --rm -i agnusdei1207/mcp-websearch:latest

# Web search
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"web_search","arguments":{"query":"Rust","limit":3}}}' | docker run --rm -i agnusdei1207/mcp-websearch:latest

# Unit tests (requires cargo in builder)
docker run --rm -w /app agnusdei1207/mcp-websearch:latest cargo test --lib
```

---

## Performance

| Metric | Value |
|--------|-------|
| Docker Image | ~10MB (Alpine) |
| Binary Size | ~3MB |
| Memory Usage | ~2-5MB |
| Startup Time | <10ms |
| Search Speed | ~500ms/page |

---

## Technical Details

- **Rust Edition**: 2024
- **Build**: rust:1.92-alpine
- **Runtime**: alpine:latest (musl)
- **TLS**: rustls (static linking)
- **HTTP**: reqwest with rustls-tls
- **Search**: DuckDuckGo HTML scraping

---

## License

MIT
