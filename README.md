# MCP Web Search Server (Rust)

High-performance Model Context Protocol (MCP) server providing web search functionality using **DuckDuckGo** HTML scraping.

## Features

- **🔍 DuckDuckGo Search**: Privacy-focused search engine, no API keys required
- **⚡ High Performance**: Rust implementation for speed and efficiency
- **🔒 Rate Limiting**: 30 requests/minute to avoid blocking
- **🌐 Multi-language**: Supports Korean, English, and other languages
- **📦 Ultra-Small Docker**: ~10MB Alpine-based image with gcompat for runtime compatibility
- **🎯 LLM-Friendly Output**: Natural language formatted results
- **🛡️ Ad Filtering**: Automatically filters out sponsored results
- **🚀 Zero Dependencies**: Uses POST requests (more reliable than GET)

---

## Quick Start (Docker Hub)

### Pull & Test

```bash
# Pull the image
docker pull agnusdei1207/mcp-websearch:latest

# Test: List available tools
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | docker run --rm -i agnusdei1207/mcp-websearch:latest

# Test: Web search with JSON-RPC
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"web_search","arguments":{"query":"Rust programming","limit":3}}}' | docker run --rm -i agnusdei1207/mcp-websearch:latest
```

### Expected Output

```
Found 3 search results for "Rust programming":

1. Rust Programming Language
   URL: https://rust-lang.org/
   Summary: Rust is a fast, reliable, and productive programming language...

2. Rust (programming language) - Wikipedia
   URL: https://en.wikipedia.org/wiki/Rust_(programming_language)
   Summary: Rust is a general-purpose programming language...
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

DuckDuckGo web search with natural language output.

**Parameters:**
- `query` (required): Search query string
- `limit` (optional): Number of results (1-100, default: 10)
- `offset` (optional): Pagination offset (default: 0)

---

## Technical Details

- **Search Engine**: DuckDuckGo HTML scraping
- **HTTP Method**: POST with form data (more reliable than GET)
- **Rate Limiting**: 30 requests/minute to avoid IP blocking
- **Ad Filtering**: Removes sponsored results (`y.js` links)
- **URL Extraction**: Decodes DuckDuckGo redirect URLs to real URLs
- **CAPTCHA Detection**: Gracefully handles bot detection

### Technologies

- **Rust Edition**: 2024
- **Build**: rust:1.92-alpine
- **Runtime**: alpine:latest (musl + gcompat for glibc compatibility)
- **TLS**: rustls (static linking)
- **HTTP**: reqwest with rustls-tls
- **HTML Parsing**: scraper crate

---

## Comparison with Other Projects

| Feature | This Project (Rust) | Python version (nickclyde/duckduckgo-mcp-server) |
|---------|---------------------|--------------------------------------------------|
| **Language** | Rust | Python |
| **Size** | ~10MB Docker | ~100MB+ (Python runtime) |
| **Startup** | <10ms | ~100ms+ |
| **Memory** | 2-5MB | 50-100MB+ |
| **HTTP Method** | POST | POST |
| **Rate Limiting** | ✅ 30 req/min | ✅ 30 req/min |
| **Ad Filtering** | ✅ | ✅ |
| **fetch_content** | ❌ | ✅ (webpage content fetcher) |

**Note**: The Python version includes a `fetch_content` tool that fetches and parses webpage content. This Rust implementation focuses on search only for minimal resource usage.

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
│   │   └── mod.rs          # Rate limiter, POST requests, HTML parsing
│   └── mcp/                # MCP protocol + tests
└── tests/                  # E2E tests
    └── e2e_tests.rs
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

## License

MIT
