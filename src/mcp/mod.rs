// Project: MCP Memo App
// Author: Rajeshwar Raja
// Date: 2025-12-28
// License: Proprietary

use rmcp::{
    ServerHandler, handler::server::{
        router::tool::ToolRouter,
        tool::Parameters,
    }, model::*, schemars, tool, tool_handler, tool_router
};
use serde_json::json;
use crate::memos:: {
    Server,
    service::{note::{Note, NoteService}},
};

#[derive(schemars::JsonSchema, serde::Deserialize)]
struct MemoNameParam {
    #[schemars(description = "The name of the memo.")]
    name: String,
}

#[derive(schemars::JsonSchema, serde::Deserialize)]
struct CommentMemoParam {
    #[schemars(description = "The name of the memo to comment on.")]
    memo_name: String,
    comment: Note,
}

pub struct MemoMCP {
    tool_router: ToolRouter<MemoMCP>,
    server: Server,
}

#[tool_router]
impl MemoMCP {
    pub fn new(host: &str, token: &str) -> Self {
        Self {
            tool_router: Self::tool_router(),
            server: Server::new(host, token),
        }
    }

    #[tool(description = "List all notes.", annotations(title = "List notes", read_only_hint = true))]
    async fn list_memos(
        &self,
        _params: Parameters<serde_json::Value>,
    ) -> String {
        tracing::debug!("Listing memos...");
        match self.server.list_notes().await {
            Ok(notes) => json!(notes).to_string(),
            Err(e) => json!({"error": e.to_string()}).to_string(),
        }
    }

    #[tool(description = "Get a memo (note) by its name field.", annotations(title = "Get a note", read_only_hint = true))]
    async fn get_memo(
        &self,
        Parameters(MemoNameParam { name }): Parameters<MemoNameParam>,
    ) -> String {
        match self.server.get_note(&name).await {
            Ok(note) => json!(note).to_string(),
            Err(e) => json!({"error": e.to_string()}).to_string(),
        }
    }

    #[tool(description = "Create a new memo (note) with given content.", annotations(title = "Create a note", read_only_hint = false))]
    async fn create_memo(
        &self,
        Parameters(note): Parameters<Note>,
    ) -> String {
        match self.server.create_note(&note).await {
            Ok(note) => json!(note).to_string(),
            Err(e) => json!({"error": e.to_string()}).to_string(),
        }
    }

    #[tool(description = "Update an existing memo (note) by its name field.", annotations(title = "Update a note", read_only_hint = false))]
    async fn update_memo(
        &self,
        Parameters(note): Parameters<Note>,
    ) -> String {
        match self.server.update_note(&note).await {
            Ok(note) => json!(note).to_string(),
            Err(e) => json!({"error": e.to_string()}).to_string(),
        }
    }

    #[tool(description = "Delete a memo (note) by its name field.", annotations(title = "Delete a note", read_only_hint = false))]
    async fn delete_memo(
        &self,
        Parameters(note): Parameters<Note>,
    ) -> String {
        match self.server.delete_note(note.name.as_ref().unwrap()).await {
            Ok(_) => json!({"status": "success"}).to_string(),
            Err(e) => json!({"error": e.to_string()}).to_string(),
        }
    }

    #[tool(description = "Create a memo (note) comment.", annotations(title = "Create a note comment", read_only_hint = false))]
    async fn create_memo_comment(
        &self,
        Parameters(CommentMemoParam{ memo_name, comment }): Parameters<CommentMemoParam>,
    ) -> String {
        match self.server.create_note_comment(&memo_name, &comment).await {
            Ok(comment) => json!(comment).to_string(),
            Err(e) => json!({"error": e.to_string()}).to_string(),
        }
    }

    #[tool(description = "List comments of a memo (note) by its name field.", annotations(title = "List note comments", read_only_hint = true))]
    async fn list_memo_comments(
        &self,
        Parameters(MemoNameParam { name }): Parameters<MemoNameParam>,
    ) -> String {
        match self.server.list_note_comments(&name).await {
            Ok(comments) => json!(comments).to_string(),
            Err(e) => json!({"error": e.to_string()}).to_string(),
        }
    }
}

#[tool_handler]
impl ServerHandler for MemoMCP {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            ..Default::default()
        }
    }
}