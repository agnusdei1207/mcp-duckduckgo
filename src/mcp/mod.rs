//! MCP (Model Context Protocol) server implementation

use crate::models::{JsonRpcRequest, JsonRpcResponse, ToolContent, ToolResponse};
use crate::search::DuckDuckGoScraper;
use serde_json::json;
use std::io::{self, BufRead, BufReader, Write};

/// MCP server
pub struct McpServer {
    scraper: DuckDuckGoScraper,
}

impl McpServer {
    /// Create a new MCP server instance
    pub fn new() -> Self {
        Self {
            scraper: DuckDuckGoScraper::new(),
        }
    }

    /// Get available tools
    pub fn get_tools(&self) -> serde_json::Value {
        json!({
            "tools": [
                {
                    "name": "web_search",
                    "description": "Search the web using DuckDuckGo. Returns relevant results with title, URL, and snippet. Supports fetching up to 9999 results.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "query": {
                                "type": "string",
                                "description": "The search query string"
                            },
                            "limit": {
                                "type": "integer",
                                "description": "Number of results to return (1-9999, default: 10)",
                                "minimum": 1,
                                "maximum": 9999,
                                "default": 10
                            },
                            "offset": {
                                "type": "integer",
                                "description": "Pagination offset (default: 0)",
                                "minimum": 0,
                                "default": 0
                            }
                        },
                        "required": ["query"]
                    }
                }
            ]
        })
    }

    /// Handle initialize request
    pub fn handle_initialize(&self) -> serde_json::Value {
        json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {}
            },
            "serverInfo": {
                "name": "mcp-websearch",
                "version": "1.0.0"
            }
        })
    }

    /// Call a tool
    pub async fn call_tool(
        &self,
        name: &str,
        params: &serde_json::Value,
    ) -> Result<ToolResponse, anyhow::Error> {
        match name {
            "web_search" => {
                let query = params["query"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing 'query' parameter"))?;

                let limit = params["limit"].as_u64().unwrap_or(10) as usize;
                let offset = params["offset"].as_u64().unwrap_or(0) as usize;

                let search_params =
                    crate::models::SearchParams::new(query, limit, offset);

                let response = self.scraper.search(&search_params).await?;

                Ok(ToolResponse {
                    content: vec![ToolContent {
                        content_type: "text".to_string(),
                        text: serde_json::to_string_pretty(&response)?,
                    }],
                    is_error: None,
                })
            }
            _ => Ok(ToolResponse {
                content: vec![ToolContent {
                    content_type: "text".to_string(),
                    text: format!("Unknown tool: {}", name),
                }],
                is_error: Some(true),
            }),
        }
    }

    /// Handle an incoming JSON-RPC request
    pub async fn handle_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        let (result, error) = match request.method.as_str() {
            "tools/list" => (Some(self.get_tools()), None),
            "tools/call" => {
                if let Some(params) = &request.params {
                    let name = params["name"].as_str().unwrap_or("");
                    let arguments = &params["arguments"];

                    match self.call_tool(name, arguments).await {
                        Ok(r) => (Some(json!(r)), None),
                        Err(e) => (None, Some(crate::models::JsonRpcError::new(-32600, e.to_string()))),
                    }
                } else {
                    (None, Some(crate::models::JsonRpcError::new(-32600, "Missing params")))
                }
            }
            "initialize" => (Some(self.handle_initialize()), None),
            _ => (None, Some(crate::models::JsonRpcError::new(-32600, format!("Unknown method: {}", request.method)))),
        };

        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result,
            error,
        }
    }

    /// Run the MCP server on stdio
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let stdin = io::stdin();
        let stdout = io::stdout();
        let mut stdout = stdout.lock();

        eprintln!("MCP Web Search Server (Rust) starting on stdio...");

        for line in BufReader::new(stdin).lines() {
            let line = line?;

            if line.trim().is_empty() {
                continue;
            }

            let request: JsonRpcRequest = match serde_json::from_str(&line) {
                Ok(req) => req,
                Err(e) => {
                    eprintln!("JSON parse error: {}", e);
                    continue;
                }
            };

            let response = self.handle_request(request).await;
            let output = serde_json::to_string(&response)?;

            writeln!(stdout, "{}", output)?;
            stdout.flush()?;
        }

        Ok(())
    }
}

impl Default for McpServer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;
