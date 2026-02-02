//! MCP Web Search Server - High-performance Rust implementation
//!
//! A Model Context Protocol server that provides web search functionality
//! using DuckDuckGo HTML scraping. Free, unlimited, no API keys required.

pub mod models;
pub mod search;
pub mod mcp;

pub use models::{JsonRpcRequest, JsonRpcResponse, Tool, ToolContent, ToolResponse};
pub use models::{SearchResult, SearchResponse, SearchParams};
pub use search::DuckDuckGoScraper;
pub use mcp::McpServer;
