pub mod rocketchat {

    use chrono::{DateTime, Local};
    use reqwest::blocking::Client as ReqwestClient;
    use serde::{Deserialize, Serialize};
    use std::fmt::Display;

    #[derive(Debug)]
    pub struct Client {
        url: String,
    }

    impl Client {
        pub fn new<S: Into<String>>(url: S) -> Client {
            Client { url: url.into() }
        }

        pub fn post_message(
            &self,
            message: &Message,
        ) -> Result<Response, Box<dyn std::error::Error>> {
            let res = ReqwestClient::new()
                .post(&self.url)
                .json(message)
                .send()?
                .json::<Response>()?;
            Ok(res)
        }
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct Attachment {
        #[serde(skip_serializing_if = "String::is_empty")]
        title: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        text: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        color: String,
        ts: DateTime<Local>,
        #[serde(skip_serializing_if = "std::ops::Not::not")]
        collapsed: bool,
    }

    impl Attachment {
        fn update_ts(mut self) -> Self {
            self.ts = Local::now();
            self
        }

        pub fn new() -> Attachment {
            Attachment {
                ts: Local::now(),
                collapsed: false,
                ..Default::default()
            }
        }

        pub fn title<S: Into<String>>(mut self, title: S) -> Self {
            self.title = title.into();
            self.update_ts()
        }

        pub fn text<S: Into<String>>(mut self, text: S) -> Self {
            self.text = text.into();
            self.update_ts()
        }

        pub fn color<S: Into<String>>(mut self, color: S) -> Self {
            self.color = color.into();
            self.update_ts()
        }

        pub fn collapsed(mut self, collapsed: bool) -> Self {
            self.collapsed = collapsed;
            self
        }
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct Message {
        #[serde(skip_serializing_if = "String::is_empty")]
        room_id: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        channel: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        text: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        alias: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        avatar: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        emoji: String,
        attachments: Vec<Attachment>,
    }

    impl Message {
        pub fn new() -> Self {
            Self {
                ..Default::default()
            }
        }

        pub fn room_id<S: Into<String>>(mut self, room_id: S) -> Self {
            self.room_id = room_id.into();
            self
        }

        pub fn channel<S: Into<String>>(mut self, channel: S) -> Self {
            self.channel = channel.into();
            self
        }

        pub fn text<S: Into<String>>(mut self, text: S) -> Self {
            self.text = text.into();
            self
        }

        pub fn attachment(mut self, attachment: Attachment) -> Self {
            self.attachments.push(attachment);
            self
        }

        pub fn attachments(mut self, attachments: Vec<Attachment>) -> Self {
            self.attachments = attachments;
            self
        }

        pub fn emoji<S: Into<String>>(mut self, emoji: S) -> Self {
            self.emoji = emoji.into();
            self
        }

        pub fn alias<S: Into<String>>(mut self, alias: S) -> Self {
            self.alias = alias.into();
            self
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Response {
        #[serde(default)]
        success: bool,
        #[serde(default)]
        error: String,
    }

    impl Response {
        pub fn is_success(&self) -> bool {
            self.success
        }
    }

    impl Display for Response {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.success {
                f.write_str("ok")
            } else if self.error.is_empty() {
                f.write_str("unknown error")
            } else {
                f.write_str(&self.error)
            }
        }
    }
}
