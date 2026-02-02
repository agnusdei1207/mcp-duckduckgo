//! Unit tests for models

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_search_params_clamping() {
        // Test lower bound
        let params = SearchParams::new("test", 0, 0);
        assert_eq!(params.limit, 1); // Should clamp to minimum

        // Test upper bound
        let params = SearchParams::new("test", 10000, 0);
        assert_eq!(params.limit, 9999); // Should clamp to maximum

        // Test normal values
        let params = SearchParams::new("test", 10, 5);
        assert_eq!(params.limit, 10);
        assert_eq!(params.offset, 5);
        assert_eq!(params.query, "test");
    }

    #[test]
    fn test_json_rpc_error_creation() {
        let error = JsonRpcError::new(-32600, "Test error");
        assert_eq!(error.code, -32600);
        assert_eq!(error.message, "Test error");
    }

    #[test]
    fn test_search_result_serialization() {
        let result = SearchResult {
            title: "Test Title".to_string(),
            url: "https://example.com".to_string(),
            snippet: "Test snippet".to_string(),
        };

        let json = serde_json::to_string(&result).unwrap();
        let parsed: SearchResult = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.title, "Test Title");
        assert_eq!(parsed.url, "https://example.com");
        assert_eq!(parsed.snippet, "Test snippet");
    }

    #[test]
    fn test_search_response_serialization() {
        let response = SearchResponse {
            query: "test query".to_string(),
            results: vec![SearchResult {
                title: "Test".to_string(),
                url: "https://example.com".to_string(),
                snippet: "Snippet".to_string(),
            }],
            total_results: 1,
            returned: 1,
            offset: 0,
        };

        let json = serde_json::to_string(&response).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(value["query"], "test query");
        assert_eq!(value["totalResults"], 1);
        assert_eq!(value["results"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_json_rpc_request_deserialization() {
        let json = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/list",
            "params": null
        });

        let request: JsonRpcRequest = serde_json::from_value(json).unwrap();
        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.method, "tools/list");
        assert_eq!(request.id, Some(serde_json::json!(1)));
    }

    #[test]
    fn test_json_rpc_response_serialization() {
        let response = JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            result: Some(json!({"status": "ok"})),
            error: None,
        };

        let json = serde_json::to_string(&response).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(value["jsonrpc"], "2.0");
        assert_eq!(value["id"], 1);
        assert_eq!(value["result"]["status"], "ok");
        assert!(value["error"].is_null());
    }

    #[test]
    fn test_tool_response_is_error_flag() {
        let success_response = ToolResponse {
            content: vec![ToolContent {
                content_type: "text".to_string(),
                text: "Success".to_string(),
            }],
            is_error: None,
        };

        let error_response = ToolResponse {
            content: vec![ToolContent {
                content_type: "text".to_string(),
                text: "Error".to_string(),
            }],
            is_error: Some(true),
        };

        // Serialize and check that is_error is omitted when None
        let json_success = serde_json::to_string(&success_response).unwrap();
        assert!(!json_success.contains("isError"));

        // And included when Some(true)
        let json_error = serde_json::to_string(&error_response).unwrap();
        assert!(json_error.contains("\"isError\":true"));
    }
}
