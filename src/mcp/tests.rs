//! Unit tests for MCP module

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{JsonRpcRequest, SearchParams};
    use serde_json::json;

    #[tokio::test]
    async fn test_get_tools() {
        let server = McpServer::new();
        let tools = server.get_tools();

        assert!(tools["tools"].is_array());
        let tools_array = tools["tools"].as_array().unwrap();
        assert_eq!(tools_array.len(), 1);

        let tool = &tools_array[0];
        assert_eq!(tool["name"], "web_search");
        assert!(tool["description"].is_string());
        assert!(tool["inputSchema"]["properties"]["query"]["type"] == "string");
    }

    #[tokio::test]
    async fn test_handle_initialize() {
        let server = McpServer::new();
        let result = server.handle_initialize();

        assert_eq!(result["protocolVersion"], "2024-11-05");
        assert_eq!(result["serverInfo"]["name"], "mcp-websearch");
        assert_eq!(result["serverInfo"]["version"], "1.0.0");
    }

    #[tokio::test]
    async fn test_handle_request_tools_list() {
        let server = McpServer::new();
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(1)),
            method: "tools/list".to_string(),
            params: None,
        };

        let response = server.handle_request(request).await;

        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, Some(json!(1)));
        assert!(response.result.is_some());
        assert!(response.error.is_none());
    }

    #[tokio::test]
    async fn test_handle_request_initialize() {
        let server = McpServer::new();
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(1)),
            method: "initialize".to_string(),
            params: None,
        };

        let response = server.handle_request(request).await;

        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.result.is_some());
        assert_eq!(response.result.unwrap()["serverInfo"]["name"], "mcp-websearch");
    }

    #[tokio::test]
    async fn test_handle_request_unknown_method() {
        let server = McpServer::new();
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(1)),
            method: "unknown/method".to_string(),
            params: None,
        };

        let response = server.handle_request(request).await;

        assert!(response.result.is_none());
        assert!(response.error.is_some());
        assert_eq!(response.error.unwrap().code, -32600);
    }

    #[tokio::test]
    async fn test_call_tool_unknown_tool() {
        let server = McpServer::new();
        let params = json!({"query": "test"});

        let result = server.call_tool("unknown_tool", &params).await.unwrap();

        assert_eq!(result.content.len(), 1);
        assert!(result.content[0].text.contains("Unknown tool"));
        assert_eq!(result.is_error, Some(true));
    }

    #[tokio::test]
    async fn test_call_tool_web_search_missing_query() {
        let server = McpServer::new();
        let params = json!({}); // Missing query

        let result = server.call_tool("web_search", &params).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Missing 'query'"));
    }

    #[tokio::test]
    async fn test_call_tool_web_search_valid_params() {
        let server = McpServer::new();
        let params = json!({
            "query": "test query",
            "limit": 5,
            "offset": 0
        });

        // This will actually make a network request, so we just verify it doesn't error immediately
        let result = server.call_tool("web_search", &params).await;

        // Result could be Ok or Err depending on network, but should not panic
        match result {
            Ok(response) => {
                assert_eq!(response.content.len(), 1);
                assert_eq!(response.is_error, None);
            }
            Err(_) => {
                // Network errors are acceptable in tests
            }
        }
    }

    #[test]
    fn test_default_server() {
        let server = McpServer::default();
        // Just verify it can be created
        assert!(server.scraper.client.timeout().is_some());
    }
}
