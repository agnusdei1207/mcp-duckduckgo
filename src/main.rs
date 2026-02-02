//! MCP Web Search Server - Main entry point

use mcp_websearch::McpServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = McpServer::new();
    server.run().await
}
