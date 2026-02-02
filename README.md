# MCP 웹 검색 서버 (Rust)

**DuckDuckGo** HTML 스크래핑을 사용하는 고성능 Model Context Protocol (MCP) 서버입니다.

## 특징

- **🔍 DuckDuckGo 검색**: API 키 없이 사용 가능한 프라이버시 중심 검색 엔진
- **⚡ 고성능**: 속도와 효율성을 위한 Rust 구현
- **🔒 속도 제한**: 차단 방지를 위해 분당 30회 요청 제한
- **🌐 다국어 지원**: 한국어, 영어 및 기타 언어 지원
- **📦 초소형 Docker**: 런타임 호환성을 위한 gcompat이 포함된 ~10MB Alpine 기반 이미지
- **🎯 LLM 친화적 출력**: 자연어 형식의 결과
- **🛡️ 광고 필터링**: 스폰서 결과 자동 필터링
- **🚀 Zero Dependencies**: POST 요청 사용 (GET보다 더 안정적)

---

## 빠른 시작 (Docker Hub)

### 이미지 받기 & 테스트

```bash
# 이미지 받기
docker pull lepisoderegistry/mcp-websearch:latest

# 테스트: 사용 가능한 도구 목록
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | docker run --rm -i lepisoderegistry/mcp-websearch:latest 2>/dev/null

# 테스트: 웹 검색
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"web_search","arguments":{"query":"Rust 프로그래밍","limit":3}}}' | docker run --rm -i lepisoderegistry/mcp-websearch:latest 2>/dev/null

# 테스트: 웹페이지 내용 가져오기
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"fetch_content","arguments":{"url":"https://www.rust-lang.org/"}}}' | docker run --rm -i lepisoderegistry/mcp-websearch:latest 2>/dev/null
```

### 예상 출력

```
Found 3 search results for "Rust 프로그래밍":

1. Rust Programming Language
   URL: https://rust-lang.org/
   Summary: Rust is a fast, reliable, and productive programming language...

2. Rust (programming language) - Wikipedia
   URL: https://en.wikipedia.org/wiki/Rust_(programming_language)
   Summary: Rust is a general-purpose programming language...
```

---

## Claude Code 연동

### 설정 파일 위치

- **Linux**: `~/.config/claude-code/config.json`
- **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`

### MCP 서버 추가

```json
{
  "mcpServers": {
    "duckduckgo": {
      "command": "docker",
      "args": ["run", "--rm", "-i", "lepisoderegistry/mcp-websearch:latest"]
    }
  }
}
```

다른 MCP 서버가 있는 경우:
```json
{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/allowed/path"]
    },
    "duckduckgo": {
      "command": "docker",
      "args": ["run", "--rm", "-i", "lepisoderegistry/mcp-websearch:latest"]
    }
  }
}
```

### 재시작 및 확인

1. Claude Code를 닫았다가 다시 엽니다
2. 다음을 시도해 보세요: `"Claude AI" 검색 (결과 5개)`

---

## 도구: web_search

자연어 출력과 함께 DuckDuckGo 웹 검색을 수행합니다.

**파라미터:**
- `query` (필수): 검색어 문자열
- `limit` (선택): 결과 수 (1-100, 기본값: 10)
- `offset` (선택): 페이지 오프셋 (기본값: 0)

---

## 도구: fetch_content

웹페이지 내용을 가져와서 파싱합니다. 스크립트, 스타일, 네비게이션 요소를 제거하고 주요 텍스트를 추출합니다.

**파라미터:**
- `url` (필수): 내용을 가져올 웹페이지 URL

---

## 기술적 세부사항

- **검색 엔진**: DuckDuckGo HTML 스크래핑
- **HTTP 메서드**: POST 요청 (GET보다 더 안정적)
- **속도 제한**: IP 차단 방지를 위해 분당 30회 요청
- **광고 필터링**: 스폰서 결과 제거 (`y.js` 링크)
- **URL 추출**: DuckDuckGo 리다이렉트 URL을 실제 URL로 디코딩
- **CAPTCHA 감지**: 봇 감지를 우아하게 처리
- **JSON-RPC 2.0**: 스펙 엄격 준수 (`error` 필드가 없을 때는 생략)

### 사용 기술

- **Rust Edition**: 2024
- **Build**: rust:1.92-alpine
- **Runtime**: alpine:latest (musl + gcompat for glibc 호환성)
- **TLS**: rustls (정적 링킹)
- **HTTP**: reqwest with rustls-tls
- **HTML Parsing**: scraper crate

---

## 다른 프로젝트와 비교

| 기능 | 이 프로젝트 (Rust) | Python 버전 (nickclyde/duckduckgo-mcp-server) |
|---------|---------------------|--------------------------------------------------|
| **언어** | Rust | Python |
| **크기** | ~10MB Docker | ~100MB+ (Python runtime) |
| **시작 속도** | <10ms | ~100ms+ |
| **메모리** | 2-5MB | 50-100MB+ |
| **HTTP 메서드** | POST | POST |
| **속도 제한** | ✅ 30 req/min | ✅ 30 req/min |
| **광고 필터링** | ✅ | ✅ |
| **fetch_content** | ✅ | ✅ (webpage content fetcher) |

---

## 소스에서 빌드하기

```bash
docker build -t mcp-websearch:latest .
```

### 바이너리 추출

```bash
docker create --name temp mcp-websearch:latest
docker cp temp:/app/mcp-websearch ./mcp-websearch
docker rm temp
chmod +x ./mcp-websearch
```

### Docker Hub에 푸시

```bash
docker login
docker tag mcp-websearch:latest lepisoderegistry/mcp-websearch:latest
docker push lepisoderegistry/mcp-websearch:latest
```

---

## 프로젝트 구조

```
mcp-websearch/
├── Cargo.toml              # 프로젝트 매니페스트 (edition = "2024")
├── Dockerfile              # Alpine 기반 멀티스테이지 빌드
├── README.md               # 이 파일
├── src/
│   ├── main.rs             # 진입점
│   ├── lib.rs              # 라이브러리 내보내기
│   ├── models/             # 데이터 모델 + 테스트
│   ├── search/             # DuckDuckGo 스크래퍼 + 테스트
│   │   └── mod.rs          # 속도 제한, POST 요청, HTML 파싱
│   └── mcp/                # MCP 프로토콜 + 테스트
└── tests/                  # E2E 테스트
    └── e2e_tests.rs
```

---

## 성능

| 지표 | 값 |
|--------|-------|
| Docker 이미지 | ~10MB (Alpine) |
| 바이너리 크기 | ~3MB |
| 메모리 사용량 | ~2-5MB |
| 시작 시간 | <10ms |
| 검색 속도 | ~500ms/페이지 |

---

## 라이선스

MIT
