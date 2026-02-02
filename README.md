# MCP Web Search Server (Rust)

High-performance Model Context Protocol (MCP) server providing web search functionality using **DuckDuckGo** HTML scraping.

## 🔍 DuckDuckGo 검색 엔진 사용

이 서버는 **DuckDuckGo**를 사용하여 웹 검색을 수행합니다:
- ✅ **완전 무료**: API 키 불필요
- ✅ **개인 정보 보호**: DuckDuckGo는 사용자를 추적하지 않음
- ✅ **무제한**: 검색 횟수 제한 없음 (Rate limiting: 30 req/min)

## Features

- **🔍 DuckDuckGo Search**: Privacy-focused search engine, no API keys required
- **⚡ High Performance**: Rust implementation for speed and efficiency
- **🔒 Rate Limiting**: 30 requests/minute to avoid blocking
- **🌐 Multi-language**: Supports Korean, English, and other languages
- **📦 Ultra-Small Docker**: ~10MB Alpine-based image with gcompat for runtime compatibility
- **🎯 LLM-Friendly Output**: Natural language formatted results
- **🛡️ Ad Filtering**: Automatically filters out sponsored results

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

DuckDuckGo로 웹 검색을 수행합니다. 자연어 형식으로 결과를 반환합니다.

**Parameters:**
- `query` (required): 검색어
- `limit` (optional): 결과 개수 (1-100, 기본값: 10)
- `offset` (optional): 페이지 오프셋 (기본값: 0)

**Example Output:**
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
- **Runtime**: alpine:latest (musl + gcompat for glibc compatibility)
- **TLS**: rustls (static linking)
- **HTTP**: reqwest with rustls-tls
- **Search**: DuckDuckGo HTML scraping

---

## License

MIT
