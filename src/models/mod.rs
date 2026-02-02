//! Data models for the MCP Web Search Server

use serde::{Deserialize, Serialize};

// ============== MCP Protocol Types ==============

/// JSON-RPC request from MCP client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

/// JSON-RPC response to MCP client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

/// JSON-RPC error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
}

impl JsonRpcError {
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

// ============== Tool Types ==============

/// Tool definition for MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

/// Tool content item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}

/// Tool response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResponse {
    pub content: Vec<ToolContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}

// ============== Search Types ==============

/// Single search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
}

/// Search response with pagination info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub query: String,
    pub results: Vec<SearchResult>,
    pub total_results: usize,
    pub returned: usize,
    pub offset: usize,
}

/// Search parameters
#[derive(Debug, Clone)]
pub struct SearchParams {
    pub query: String,
    pub limit: usize,
    pub offset: usize,
}

impl SearchParams {
    pub fn new(query: impl Into<String>, limit: usize, offset: usize) -> Self {
        Self {
            query: query.into(),
            limit: limit.clamp(1, 9999),
            offset,
        }
    }
}

#[cfg(test)]
mod tests;
