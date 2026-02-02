# MCP Web Search Server (Rust)

A high-performance Model Context Protocol (MCP) server using **DuckDuckGo** HTML scraping.

## Features

- **🔍 DuckDuckGo Search**: Privacy-focused search engine with no API key required
- **⚡ High Performance**: Rust implementation for speed and efficiency
- **🔒 Rate Limiting**: 30 requests per minute to avoid blocking
- **🌐 Multi-language Support**: Korean, English, and other languages supported
- **📦 Tiny Docker**: ~10MB Alpine-based image with gcompat for runtime compatibility
- **🎯 LLM-friendly Output**: Natural language formatted results
- **🛡️ Ad Filtering**: Automatically filters sponsored results
- **🚀 Zero Dependencies**: Uses POST requests (more stable than GET)

---

## Quick Start (Docker Hub)

### Pull & Test

```bash
# Pull image
docker pull agnusdei1207/mcp-websearch:latest

# Test: list available tools
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | docker run --rm -i agnusdei1207/mcp-websearch:latest 2>/dev/null

# Test: web search
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"web_search","arguments":{"query":"Rust programming","limit":3}}}' | docker run --rm -i agnusdei1207/mcp-websearch:latest 2>/dev/null

# Test: fetch webpage content
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"fetch_content","arguments":{"url":"https://www.rust-lang.org/"}}}' | docker run --rm -i agnusdei1207/mcp-websearch:latest 2>/dev/null
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
    "duckduckgo": {
      "command": "docker",
      "args": ["run", "--rm", "-i", "agnusdei1207/mcp-websearch:latest"]
    }
  }
}
```

With other MCP servers:
```json
{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/allowed/path"]
    },
    "duckduckgo": {
      "command": "docker",
      "args": ["run", "--rm", "-i", "agnusdei1207/mcp-websearch:latest"]
    }
  }
}
```

### Restart & Verify

1. Restart Claude Code
2. Try: `"Search for Claude AI (5 results)"`

---

## Tool: web_search

Performs DuckDuckGo web search with natural language output.

**Parameters:**
- `query` (required): Search query string
- `limit` (optional): Number of results (1-100, default: 10)
- `offset` (optional): Pagination offset (default: 0)

---

## Tool: fetch_content

Fetches and parses webpage content. Removes scripts, styles, and navigation elements to extract main text.

**Parameters:**
- `url` (required): URL of the webpage to fetch and parse

---

## Technical Details

- **Search Engine**: DuckDuckGo HTML scraping
- **HTTP Method**: POST requests (more stable than GET)
- **Rate Limiting**: 30 requests per minute to prevent IP blocking
- **Ad Filtering**: Removes sponsored results (`y.js` links)
- **URL Extraction**: Decodes DuckDuckGo redirect URLs to actual URLs
- **CAPTCHA Detection**: Gracefully handles bot detection
- **JSON-RPC 2.0**: Strict spec compliance (omits `error` field when not present)

### Tech Stack

- **Rust Edition**: 2024
- **Build**: rust:1.92-alpine
- **Runtime**: alpine:latest (musl + gcompat for glibc compatibility)
- **TLS**: rustls (statically linked)
- **HTTP**: reqwest with rustls-tls
- **HTML Parsing**: scraper crate

---

## Comparison with Other Projects

| Feature | This Project (Rust) | Python Version (nickclyde/duckduckgo-mcp-server) |
|---------|---------------------|--------------------------------------------------|
| **Language** | Rust | Python |
| **Size** | ~10MB Docker | ~100MB+ (Python runtime) |
| **Startup Time** | <10ms | ~100ms+ |
| **Memory** | 2-5MB | 50-100MB+ |
| **HTTP Method** | POST | POST |
| **Rate Limiting** | ✅ 30 req/min | ✅ 30 req/min |
| **Ad Filtering** | ✅ | ✅ |
| **fetch_content** | ✅ | ✅ (webpage content fetcher) |

---

## Build from Source

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
│   │   └── mod.rs          # Rate limiting, POST requests, HTML parsing
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
