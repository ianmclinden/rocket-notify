use std::fmt::Display;

use chrono::{DateTime, Local};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct User {
    #[serde(alias = "_id", default)]
    id: String,

    #[serde(default)]
    username: String,
}

impl User {
    #[must_use]
    pub fn id(&self) -> &String {
        &self.id
    }

    #[must_use]
    pub fn username(&self) -> &String {
        &self.username
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(default)]
    alias: String,

    #[serde(default)]
    msg: String,

    #[serde(default)]
    parse_urls: bool,

    #[serde(default)]
    groupable: bool,

    #[serde(default)]
    u: User,

    #[serde(default)]
    ts: DateTime<Local>,

    #[serde(default)]
    rid: String,

    #[serde(default)]
    updated_at: DateTime<Local>,

    #[serde(alias = "_id", default)]
    id: bool,
}

impl Message {
    #[must_use]
    pub fn alias(&self) -> &str {
        self.alias.as_ref()
    }

    #[must_use]
    pub fn msg(&self) -> &str {
        self.msg.as_ref()
    }

    #[must_use]
    pub fn parse_urls(&self) -> bool {
        self.parse_urls
    }

    #[must_use]
    pub fn groupable(&self) -> bool {
        self.groupable
    }

    #[must_use]
    pub fn u(&self) -> &User {
        &self.u
    }

    #[must_use]
    pub fn ts(&self) -> DateTime<Local> {
        self.ts
    }

    #[must_use]
    pub fn rid(&self) -> &str {
        self.rid.as_ref()
    }

    #[must_use]
    pub fn updated_at(&self) -> DateTime<Local> {
        self.updated_at
    }

    #[must_use]
    pub fn id(&self) -> bool {
        self.id
    }
}

/// Rocket.Chat API Response
///
/// ### Example:
/// ```no_run
/// use std::io;
/// use rocketchat::{client::Client, request::Message};
///
/// let message = Message::new()
///     .channel("@myusername")
///     .text("Hello, world!");
///
/// let response = Client::new("https://my.rocket.com/hooks/db78d646/b072678678e8c74a")
///     .send(&message)
///     .unwrap();
///
/// if response.success() {
///     // Action
/// }
/// ```
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    #[serde(default)]
    ts: DateTime<Local>,

    #[serde(default)]
    channel: String,

    #[serde(default)]
    message: Message,

    #[serde(default)]
    success: bool,

    #[serde(default)]
    error: String,

    #[serde(default)]
    error_type: String,
}

// TODO: Make this an enum type like hyper
impl Response {
    #[must_use]
    pub fn ts(&self) -> DateTime<Local> {
        self.ts
    }

    #[must_use]
    pub fn channel(&self) -> &String {
        &self.channel
    }

    #[must_use]
    pub fn message(&self) -> &Message {
        &self.message
    }

    #[must_use]
    pub fn success(&self) -> bool {
        self.success
    }

    #[must_use]
    pub fn error_message(&self) -> &String {
        &self.error
    }

    #[must_use]
    pub fn error_type(&self) -> &String {
        &self.error_type
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.success {
            f.write_str("ok")
        } else if !self.error.is_empty() {
            f.write_str(&self.error)
        } else if !self.error_type.is_empty() {
            f.write_str(&self.error_type)
        } else if self.error_type.is_empty() {
            f.write_str("unknown error")
        } else {
            f.write_str(&self.error)
        }
    }
}
