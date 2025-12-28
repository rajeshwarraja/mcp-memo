// Project: MCP Memo App
// Author: Rajeshwar Raja
// Date: 2025-12-28
// License: Proprietary

use std::net::SocketAddr;

use tracing::info;
use anyhow::Result;
use rmcp::transport::streamable_http_server::StreamableHttpService;
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use axum::{routing::any_service, Router};
use crate::{mcp::MemoMCP, memos::service::auth::AuthService};

mod memos;
mod mcp;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(true)
        .with_line_number(true)
        .with_level(true)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into())
        )
        .init();


    let host = std::env::var("MEMOS_HOST").unwrap();
    let token = std::env::var("MEMOS_TOKEN").unwrap();

    info!("Verifying connection to memos server at {}...", host);
    {
        let server = memos::Server::new(&host, &token);
        let me = server.get_current_user().await?;
        info!("Successfully authenticated to memos server as user: {}", me.username);
    }

    info!("Initializing Memo MCP Service for host {}...", host);

    let mcp_service = StreamableHttpService::new(
        move || Ok(MemoMCP::new(&host, &token)),
        LocalSessionManager::default().into(),
        Default::default(),
    );

    info!("Starting Memo MCP Server...");
    let app = Router::new()
        .route("/mcp", any_service(mcp_service));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Server listening on {}", addr);
    
    axum::serve(listener, app).await?;    
    info!("Shutting down Memo MCP Server...");
    Ok(())
}
