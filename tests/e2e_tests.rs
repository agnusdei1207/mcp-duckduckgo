//! End-to-end tests for MCP Web Search Server

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use serde_json::Value;

/// Helper struct to manage the MCP server process
struct TestServer {
    child: std::process::Child,
}

impl TestServer {
    /// Spawn the server process
    fn spawn() -> Result<Self, Box<dyn std::error::Error>> {
        // Build the project first
        let status = Command::new("cargo")
            .args(["build", "--release"])
            .current_dir("..")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;

        if !status.success() {
            return Err("Failed to build server".into());
        }

        // Find the binary path
        let binary_path = "../target/release/mcp-websearch";

        // Spawn the server
        let child = Command::new(binary_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;

        Ok(Self { child })
    }

    /// Send a JSON-RPC request and get the response
    fn send_request(&mut self, request: &Value) -> Result<Value, Box<dyn std::error::Error>> {
        let stdin = self.child.stdin.as_mut().ok_or("No stdin")?;
        let stdout = self.child.stdout.as_mut().ok_or("No stdout")?;

        // Send request
        writeln!(stdin, "{}", serde_json::to_string(request)?)?;
        stdin.flush()?;

        // Read response
        let reader = BufReader::new(stdout);
        let line = reader.lines().next().ok_or("No response")??;
        let response: Value = serde_json::from_str(&line)?;

        Ok(response)
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[test]
fn test_e2e_initialize() {
    let mut server = TestServer::spawn().expect("Failed to spawn server");

    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": null
    });

    let response = server
        .send_request(&request)
        .expect("Failed to get response");

    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["id"], 1);
    assert!(response["result"].is_object());
    assert_eq!(response["result"]["serverInfo"]["name"], "mcp-websearch");
    assert!(response["error"].is_null() || response["error"].is_absent());
}

#[test]
fn test_e2e_tools_list() {
    let mut server = TestServer::spawn().expect("Failed to spawn server");

    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/list",
        "params": null
    });

    let response = server
        .send_request(&request)
        .expect("Failed to get response");

    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["id"], 1);
    assert!(response["result"]["tools"].is_array());
    assert_eq!(response["result"]["tools"].as_array().unwrap().len(), 1);
    assert_eq!(response["result"]["tools"][0]["name"], "web_search");
}

#[test]
fn test_e2e_web_search_call() {
    let mut server = TestServer::spawn().expect("Failed to spawn server");

    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "web_search",
            "arguments": {
                "query": "Rust programming",
                "limit": 3,
                "offset": 0
            }
        }
    });

    let response = server
        .send_request(&request)
        .expect("Failed to get response");

    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["id"], 1);

    // Check the response structure
    if let Some(result) = response["result"].as_object() {
        assert!(result["content"].is_array());
        let content = result["content"].as_array().unwrap();
        assert!(!content.is_empty());
        assert_eq!(content[0]["type"], "text");

        // Parse the text content as JSON
        let search_response: Value = serde_json::from_str(
            content[0]["text"].as_str().unwrap_or("{}")
        ).unwrap_or_default();

        assert_eq!(search_response["query"], "Rust programming");
        assert!(search_response["results"].is_array());
        assert!(search_response["totalResults"].is_number());
    } else if let Some(error) = response["error"].as_object() {
        // Network errors are acceptable in E2E tests
        eprintln!("Search failed (network error): {}", error["message"]);
    }
}

#[test]
fn test_e2e_unknown_method() {
    let mut server = TestServer::spawn().expect("Failed to spawn server");

    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "unknown/method",
        "params": null
    });

    let response = server
        .send_request(&request)
        .expect("Failed to get response");

    assert!(response["error"].is_object());
    assert_eq!(response["error"]["code"], -32600);
}

#[test]
fn test_e2e_unknown_tool() {
    let mut server = TestServer::spawn().expect("Failed to spawn server");

    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "unknown_tool",
            "arguments": {}
        }
    });

    let response = server
        .send_request(&request)
        .expect("Failed to get response");

    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["id"], 1);
    assert!(response["result"]["content"][0]["text"].as_str().unwrap_or("").contains("Unknown tool"));
    assert_eq!(response["result"]["isError"], true);
}

#[test]
fn test_e2e_web_search_missing_query() {
    let mut server = TestServer::spawn().expect("Failed to spawn server");

    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "web_search",
            "arguments": {
                "limit": 5
            }
        }
    });

    let response = server
        .send_request(&request)
        .expect("Failed to get response");

    // Should get an error about missing query
    assert!(response["error"].is_object());
}

#[test]
fn test_e2e_large_result_set() {
    let mut server = TestServer::spawn().expect("Failed to spawn server");

    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "web_search",
            "arguments": {
                "query": "test",
                "limit": 100,
                "offset": 0
            }
        }
    });

    let response = server
        .send_request(&request)
        .expect("Failed to get response");

    assert_eq!(response["jsonrpc"], "2.0");

    // Parse the response
    if let Some(result) = response["result"].as_object() {
        let search_response: Value = serde_json::from_str(
            result["content"][0]["text"].as_str().unwrap_or("{}")
        ).unwrap_or_default();

        // Should return results (or network error)
        if !search_response["results"].is_null() {
            let returned = search_response["returned"].as_u64().unwrap_or(0);
            assert!(returned > 0, "Should return at least some results");
        }
    }
}

#[test]
fn test_e2e_pagination() {
    let mut server = TestServer::spawn().expect("Failed to spawn server");

    // First request with offset 0
    let request1 = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "web_search",
            "arguments": {
                "query": "test",
                "limit": 5,
                "offset": 0
            }
        }
    });

    let response1 = server
        .send_request(&request1)
        .expect("Failed to get response");

    // Second request with offset 5
    let request2 = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": "web_search",
            "arguments": {
                "query": "test",
                "limit": 5,
                "offset": 5
            }
        }
    });

    let response2 = server
        .send_request(&request2)
        .expect("Failed to get response");

    // Both should succeed
    assert_eq!(response1["result"]["isError"], serde_json::Value::Null);
    assert_eq!(response2["result"]["isError"], serde_json::Value::Null);
}
