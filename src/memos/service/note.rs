// Project: MCP Memo App
// Author: Rajeshwar Raja
// Date: 2025-12-28
// License: Proprietary

use anyhow::Result;
use chrono::{DateTime, Utc};
use rmcp::schemars;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, schemars::JsonSchema, Debug)]
pub enum State {
    #[serde(rename = "STATE_UNSPECIFIED")]
    StateUnspecified,
    #[serde(rename = "NORMAL")]
    Normal,
    #[serde(rename = "ARCHIVED")]
    Archived,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema, Debug)]
pub enum Visibility {
    #[serde(rename = "VISIBILITY_UNSPECIFIED")]
    VisibilityUnspecified,
    #[serde(rename = "PRIVATE")]
    Private,
    #[serde(rename = "PROTECTED")]
    Protected,
    #[serde(rename = "PUBLIC")]
    Public,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    #[schemars(description = "Unique identifier for the note.")]
    #[serde(default)]
    pub name: Option<String>,
    #[schemars(required, description = "The state of the note.")]
    state: State,
    #[serde(default)]
    creator: Option<String>,
    #[serde(default)]
    #[schemars(description = "The creation time of the note.")]
    create_time: Option<DateTime<Utc>>,
    #[schemars(description = "The last update time of the note.")]
    update_time: Option<DateTime<Utc>>,
    #[schemars(description = "The display time of the note.")]
    display_time: Option<DateTime<Utc>>,
    #[schemars(required, description = "The content of the note in Markdown format.")]
    pub content: String,
    #[schemars(required, description = "The visibility level of the note.")]
    visibility: Visibility,
    #[schemars(
        description = "Tags associated with the note. To update tags, add tags in `#<tag>` format within the content."
    )]
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    pinned: bool,
    #[serde(default)]
    attachments: Vec<Attachment>,
    #[serde(default)]
    relations: Vec<Relation>,
    #[serde(default)]
    reactions: Vec<Reaction>,
    #[serde(default)]
    property: Option<serde_json::Value>,
    #[serde(default)]
    parent: String,
    #[serde(default)]
    snippet: String,
    #[serde(default)]
    location: Option<String>,
}

impl Note {
    pub fn new(content: &str) -> Self {
        Note {
            name: None,
            state: State::Normal,
            creator: None,
            create_time: None,
            update_time: None,
            display_time: None,
            content: content.to_string(),
            visibility: Visibility::Private,
            tags: vec![],
            pinned: false,
            attachments: vec![],
            relations: vec![],
            reactions: vec![],
            property: None,
            parent: "".to_string(),
            snippet: "".to_string(),
            location: None,
        }
    }
}

#[derive(Serialize, Deserialize, schemars::JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    #[serde(default)]
    name: String,
    #[serde(default)]
    create_time: DateTime<Utc>,
    #[serde(default)]
    filename: String,
    #[serde(default)]
    external_link: String,
    #[serde(rename = "type")]
    mime_type: String,
    #[serde(default)]
    size: String,
    #[serde(default)]
    memo: String,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema, Debug)]
pub enum RelationType {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    RelationTypeUnspecified,
    #[serde(rename = "REFERENCE")]
    Reference,
    #[serde(rename = "COMMENT")]
    Comment,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Relation {
    #[serde(default)]
    memo: serde_json::Value,
    #[serde(default)]
    related_memo: serde_json::Value,
    #[serde(rename = "type")]
    relation_type: RelationType,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Reaction {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    creator: Option<String>,
    #[serde(default)]
    content_id: String,
    #[serde(default)]
    reaction_type: String,
    #[serde(default)]
    create_time: Option<DateTime<Utc>>,
}

impl Reaction {
    pub fn new(content_id: &str, reaction_type: &str) -> Self {
        Reaction {
            name: None,
            creator: None,
            content_id: content_id.to_string(),
            reaction_type: reaction_type.to_string(),
            create_time: None,
        }
    }
}

pub trait NoteService {
    async fn create_note(&self, note: &Note) -> Result<Note>;

    async fn create_note_comment(&self, note_name: &str, comment: &Note) -> Result<Note>;

    async fn delete_note(&self, note_name: &str) -> Result<()>;
    async fn delete_note_reaction(&self, reaction_name: &str) -> Result<()>;

    async fn get_note(&self, note_name: &str) -> Result<Note>;

    async fn list_note_attachments(&self, note_name: &str) -> Result<Vec<Attachment>>;

    async fn list_note_comments(&self, note_name: &str) -> Result<Vec<Note>>;

    async fn list_note_reactions(&self, note_name: &str) -> Result<Vec<Reaction>>;
    async fn list_note_relations(&self, note_name: &str) -> Result<Vec<Relation>>;

    async fn list_notes(&self) -> Result<Vec<Note>>;

    async fn set_note_attachments(&self, note_name: &str, attachments: &Vec<Attachment>) -> Result<()>;

    async fn set_note_relations(&self, note_name: &str, relations: &Vec<Relation>) -> Result<()>;

    async fn update_note(&self, note: &Note) -> Result<Note>;
    async fn upsert_note_reaction(&self, note_name: &str, reaction: &Reaction) -> Result<Reaction>;
}

impl<T> NoteService for T
where
    T: crate::memos::HttpServer,
{
    async fn create_note(&self, note: &Note) -> Result<Note> {
        let rsp = self.build_post_request("memos").json(note).send().await?;

        self.validate_data_response::<Note>(rsp).await
    }

    async fn create_note_comment(&self, note_name: &str, comment: &Note) -> Result<Note> {
        let rsp = self
            .build_post_request(format!("{}/comments", note_name).as_str())
            .json(comment)
            .send()
            .await?;

        self.validate_data_response::<Note>(rsp).await
    }

    async fn delete_note(&self, note_name: &str) -> Result<()> {
        let rsp = self
            .build_delete_request(note_name)
            .send()
            .await?;

        self.validate_response(rsp).await
    }

    async fn delete_note_reaction(&self, reaction_name: &str) -> Result<()> {
        let rsp = self
            .build_delete_request(format!("{}", reaction_name).as_str())
            .send()
            .await?;

        self.validate_response(rsp).await
    }

    async fn get_note(&self, note_name: &str) -> Result<Note> {
        let rsp = self.build_get_request(note_name).send().await?;

        self.validate_data_response::<Note>(rsp).await
    }

    async fn list_note_attachments(&self, note_name: &str) -> Result<Vec<Attachment>> {
        #[derive(Deserialize, Debug)]
        struct AttachmentsResponse {
            pub attachments: Vec<Attachment>,
        }

        let rsp = self
            .build_get_request(format!("{}/attachments", note_name).as_str())
            .send()
            .await?;

        Ok(self
            .validate_data_response::<AttachmentsResponse>(rsp)
            .await?
            .attachments)
    }

    async fn list_note_comments(&self, note_name: &str) -> Result<Vec<Note>> {
        #[derive(Deserialize, Debug)]
        struct CommentsResponse {
            pub memos: Vec<Note>,
        }

        let rsp = self
            .build_get_request(format!("{}/comments", note_name).as_str())
            .send()
            .await?;

        Ok(self
            .validate_data_response::<CommentsResponse>(rsp)
            .await?
            .memos)
    }

    async fn list_note_reactions(&self, note_name: &str) -> Result<Vec<Reaction>> {
        #[derive(Deserialize, Debug)]
        struct ReactionsResponse {
            pub reactions: Vec<Reaction>,
        }

        let rsp = self
            .build_get_request(format!("{}/reactions", note_name).as_str())
            .send()
            .await?;

        Ok(self
            .validate_data_response::<ReactionsResponse>(rsp)
            .await?
            .reactions)
    }

    async fn list_note_relations(&self, note_name: &str) -> Result<Vec<Relation>> {
        #[derive(Deserialize, Debug)]
        struct RelationsResponse {
            pub relations: Vec<Relation>,
        }

        let rsp = self
            .build_get_request(format!("{}/relations", note_name).as_str())
            .send()
            .await?;

        Ok(self
            .validate_data_response::<RelationsResponse>(rsp)
            .await?
            .relations)
    }

    async fn list_notes(&self) -> Result<Vec<Note>> {
        #[derive(Deserialize)]
        struct NotesRespones {
            pub memos: Vec<Note>,
            #[serde(default, rename = "nextPageToken")]
            pub next_page_token: String,
        }

        let mut memos = Vec::<Note>::new();
        let mut next_page_token: String = String::new();

        loop {
            let endpoint = if !next_page_token.is_empty() {
                format!("memos?pageToken={}", next_page_token)
            } else {
                "memos".to_string()
            };

            let rsp = self.build_get_request(endpoint.as_str()).send().await?;

            let rsp = self.validate_data_response::<NotesRespones>(rsp).await?;
            memos.extend(rsp.memos);

            if !rsp.next_page_token.is_empty() {
                next_page_token = rsp.next_page_token;
            } else {
                break;
            }
        }
        Ok(memos)
    }

    async fn set_note_attachments(&self, note_name: &str, attachments: &Vec<Attachment>) -> Result<()> {
        #[derive(Serialize)]
        struct RequestBody<'a> {
            name: &'a str,
            attachments: &'a Vec<Attachment>,
        }

        let body = RequestBody {
            name: note_name,
            attachments,
        };

        let rsp = self
            .build_post_request(format!("{}/attachments", note_name).as_str())
            .json(&body)
            .send()
            .await?;

        self.validate_response(rsp).await
    }

    async fn set_note_relations(&self, note_name: &str, relations: &Vec<Relation>) -> Result<()> {
        #[derive(Serialize)]
        struct RequestBody<'a> {
            name: &'a str,
            relations: &'a Vec<Relation>,
        }

        let body = RequestBody {
            name: note_name,
            relations,
        };

        let rsp = self
            .build_post_request(format!("{}/relations", note_name).as_str())
            .json(&body)
            .send()
            .await?;

        self.validate_response(rsp).await
    }

    async fn update_note(&self, note: &Note) -> Result<Note> {
        let endpoint = format!("{}?updateMask=content,state,visibility,tags,pinned", note.name.as_ref().unwrap());
        let rsp = self
            .build_patch_request(endpoint.as_str())
            .json(note)
            .send()
            .await?;

        self.validate_data_response::<Note>(rsp).await
    }

    async fn upsert_note_reaction(&self, note_name: &str, reaction: &Reaction) -> Result<Reaction> {
        #[derive(Serialize)]
        struct RequestBody<'a> {
            pub name: &'a str,
            pub reaction: &'a Reaction,
        }

        let body = RequestBody {
            name: note_name,
            reaction,
        };

        let rsp = self
            .build_post_request(format!("{}/reactions", note_name).as_str())
            .json(&body)
            .send()
            .await?;

        self.validate_data_response::<Reaction>(rsp).await
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{
            super::{HttpServer, Server},
            auth::AuthService,
            user::{User, UserService},
        },
        *,
    };

    struct UserScopedServer {
        parent: Server,
        child: Option<Server>,
        user: User,
    }

    impl UserScopedServer {
        async fn new(host: &str, token: &str) -> Result<Self> {
            let parent = Server::new(host, token);

            let rand_suffix = chrono::Utc::now().timestamp_nanos_opt().unwrap();
            let username = format!("user-{}-{}", std::process::id(), rand_suffix);
            let password = "TestPassword123!";
            let user = User::new(&username, &password, "test@test.com");
            let test_user = parent.create_user(&user).await.unwrap();

            let child = parent.sign_in(&username, &password).await.unwrap();

            Ok(UserScopedServer {
                parent,
                child: Some(child),
                user: test_user,
            })
        }

        async fn cleanup(&self) -> Result<()> {
            self.parent.delete_user(&self.user).await
        }
    }

    impl HttpServer for UserScopedServer {
        fn base_url(&self) -> &str {
            self.parent.base_url()
        }

        fn token(&self) -> &str {
            &self.child.as_ref().unwrap().token()
        }
    }

    async fn create_server() -> Result<UserScopedServer> {
        Ok(UserScopedServer::new(
            "localhost:5230",
            "memos_pat_t3pjYKgGSzYqOqMgR4mZR768afCNG6sW",
        )
        .await?)
    }

    #[tokio::test]
    async fn test_create_and_delete_memo() {
        let server = create_server().await.unwrap();
        let memo = Note::new("Test memo from unit test");

        let created_memo = server.create_note(&memo).await.unwrap();
        assert_eq!(created_memo.content, "Test memo from unit test");

        server.delete_note(created_memo.name.as_ref().unwrap()).await.unwrap();

        server.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_create_and_delete_comment() {
        let server = create_server().await.unwrap();
        let memo = Note::new("Test memo for comment unit test");

        let created_memo = server.create_note(&memo).await.unwrap();
        assert_eq!(created_memo.content, "Test memo for comment unit test");

        let comment = Note::new("This is a test comment");
        let created_comment = server
            .create_note_comment(created_memo.name.as_ref().unwrap(), &comment)
            .await
            .unwrap();
        assert_eq!(created_comment.content, "This is a test comment");

        let fetched_comment = server
            .get_note(created_comment.name.as_ref().unwrap())
            .await
            .unwrap();
        assert_eq!(fetched_comment.content, "This is a test comment");

        server.delete_note(created_comment.name.as_ref().unwrap()).await.unwrap();
        server.delete_note(created_memo.name.as_ref().unwrap()).await.unwrap();

        server.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_list_notes() {
        let server = create_server().await.unwrap();
        let notes = server.list_notes().await.unwrap();
        let count = notes.len();

        let note = Note::new("Another test note for listing");
        let created_note = server.create_note(&note).await.unwrap();
        let notes_after = server.list_notes().await.unwrap();

        assert_eq!(notes_after.len(), count + 1);

        server.delete_note(created_note.name.as_ref().unwrap()).await.unwrap();
        server.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_reactions() {
        let server = create_server().await.unwrap();
        let note = Note::new("Test note for reactions unit test");

        let created_note = server.create_note(&note).await.unwrap();
        assert_eq!(created_note.content, "Test note for reactions unit test");

        let reaction = Reaction::new(created_note.name.as_ref().unwrap(), "👍");

        let created_reaction = server
            .upsert_note_reaction(created_note.name.as_ref().unwrap(), &reaction)
            .await
            .unwrap();
        assert_eq!(created_reaction.reaction_type, "👍");

        let reactions = server.list_note_reactions(created_note.name.as_ref().unwrap()).await.unwrap();
        assert!(reactions.iter().any(|r| r.reaction_type == "👍"));

        server
            .delete_note_reaction(created_reaction.name.as_ref().unwrap())
            .await
            .unwrap();

        let reactions_after = server.list_note_reactions(created_note.name.as_ref().unwrap()).await.unwrap();
        assert!(!reactions_after.iter().any(|r| r.reaction_type == "👍"));

        server.delete_note(created_note.name.as_ref().unwrap()).await.unwrap();

        server.cleanup().await.unwrap();
    }
}
