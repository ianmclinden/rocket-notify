/// [Rocket.Chat]() Message API Client
///
/// - [Message API Documentation](https://developer.rocket.chat/reference/api/rest-api/endpoints/core-endpoints/chat-endpoints/postmessage)
/// - [Full API Documentation](https://developer.rocket.chat/reference/api)
///
/// ### Example:
/// _Send a message and return the result_
/// ```no_run
/// use rocket_notify::rocketchat::{Attachment, Client, Message};
///
/// let message = Message::new()
///     .channel("@myuser")
///     .alias("My.Bot")
///     .emoji(":rocket:")
///     .attachment(
///         Attachment::new()
///             .title("Hello, World")
///             .text("A sample message body")
///             .color("green")
///             .collapsed(false),
///     );
///
/// let webhook_url = "https://my.rocket.com/hooks/db78d646/b072678678e8c74a";
/// let response = Client::new(webhook_url)
///     .post_message(&message)
///     .expect("Failed to POST message");
///
/// match response.success() {
///     true => println!("OK: Posted to {}", response.channel()),
///     false => println!("{}: {}", response.error_type(), response.error_message()),
/// };
/// ```
pub mod rocketchat {

    use chrono::{DateTime, Local};
    use reqwest::blocking::Client as ReqwestClient;
    use serde::{Deserialize, Serialize};
    use std::fmt::Display;

    /// Attachment fields that annotate an [`Attachment`]. Allows for "tables" or "columns" to be displayed on messages.
    ///
    /// ### Example:
    /// ```
    /// use rocket_notify::rocketchat::AttachmentField;
    /// let attachment_field = AttachmentField::new()
    ///     .short(true)
    ///     .title("");
    /// ```
    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct AttachmentField {
        #[serde(skip_serializing_if = "std::ops::Not::not")]
        short: bool,

        #[serde(skip_serializing_if = "String::is_empty")]
        title: String,

        #[serde(skip_serializing_if = "String::is_empty")]
        value: String,
    }

    impl AttachmentField {
        /// Create a new [`AttachmentField`].
        pub fn new() -> Self {
            Self {
                ..Default::default()
            }
        }

        /// Set whether the field should be a short field.
        pub fn short(mut self, short: bool) -> Self {
            self.short = short;
            self
        }

        /// Set the title of the field.
        pub fn title<S: Into<String>>(mut self, title: S) -> Self {
            self.title = title.into();
            self
        }

        /// Set the value of the field, displayed underneath the title value.
        pub fn value<S: Into<String>>(mut self, value: S) -> Self {
            self.value = value.into();
            self
        }
    }

    /// Message attachment, containing additional data to be rendered in the message.
    ///
    /// ### Example:
    /// ```
    /// use rocket_notify::rocketchat::Attachment;
    /// let attachment = Attachment::new()
    ///     .title("Example attachment")
    ///     .text("This is an example attachment");
    ///     
    /// ```
    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct Attachment {
        #[serde(skip_serializing_if = "String::is_empty")]
        color: String,

        #[serde(skip_serializing_if = "String::is_empty")]
        text: String,

        ts: DateTime<Local>,

        #[serde(skip_serializing_if = "String::is_empty")]
        thumb_url: String,

        #[serde(skip_serializing_if = "String::is_empty")]
        message_link: String,

        #[serde(skip_serializing_if = "std::ops::Not::not")]
        collapsed: bool,

        #[serde(skip_serializing_if = "String::is_empty")]
        author_name: String,

        #[serde(skip_serializing_if = "String::is_empty")]
        author_link: String,

        #[serde(skip_serializing_if = "String::is_empty")]
        author_icon: String,

        #[serde(skip_serializing_if = "String::is_empty")]
        title: String,

        #[serde(skip_serializing_if = "String::is_empty")]
        title_link: String,

        #[serde(skip_serializing_if = "std::ops::Not::not")]
        title_link_download: bool,

        #[serde(skip_serializing_if = "String::is_empty")]
        image_url: String,

        #[serde(skip_serializing_if = "String::is_empty")]
        audio_url: String,

        #[serde(skip_serializing_if = "String::is_empty")]
        video_url: String,

        #[serde(skip_serializing_if = "Vec::is_empty")]
        fields: Vec<AttachmentField>,
    }

    impl Attachment {
        /// Create a new [`Attachment`].
        pub fn new() -> Self {
            Self {
                ts: Local::now(),
                ..Default::default()
            }
        }

        /// Set the attachment left border color. Can be any [background-color](https://developer.mozilla.org/en-US/docs/Web/CSS/background-color) value.
        pub fn color<S: Into<String>>(mut self, color: S) -> Self {
            self.color = color.into();
            self.update_ts()
        }

        /// Set the text to display for the attachment, it is different than the message's text.
        pub fn text<S: Into<String>>(mut self, text: S) -> Self {
            self.text = text.into();
            self.update_ts()
        }

        fn update_ts(mut self) -> Self {
            self.ts = Local::now();
            self
        }

        /// An image that displays to the left of the text, looks better when this is relatively small.
        pub fn thumb_url<S: Into<String>>(mut self, thumb_url: S) -> Self {
            self.thumb_url = thumb_url.into();
            self
        }

        /// Set the attachment time to a clickable link.
        pub fn message_link<S: Into<String>>(mut self, message_link: S) -> Self {
            self.message_link = message_link.into();
            self
        }

        /// Set the image, audio, and video sections to be collapsed.
        pub fn collapsed(mut self, collapsed: bool) -> Self {
            self.collapsed = collapsed;
            self
        }

        /// Set the name of the message author.
        pub fn author_name<S: Into<String>>(mut self, author_name: S) -> Self {
            self.author_name = author_name.into();
            self
        }

        /// Set the author's name to a clickable link.
        pub fn author_link<S: Into<String>>(mut self, author_link: S) -> Self {
            self.author_link = author_link.into();
            self
        }

        /// Set a tiny icon to the left of the Author's name.
        pub fn author_icon<S: Into<String>>(mut self, author_icon: S) -> Self {
            self.author_icon = author_icon.into();
            self
        }

        /// Set the title for the attachment. Displays under the author.
        pub fn title<S: Into<String>>(mut self, title: S) -> Self {
            self.title = title.into();
            self.update_ts()
        }

        /// Set the this attachment title to a clickable link.
        pub fn title_link<S: Into<String>>(mut self, title_link: S) -> Self {
            self.title_link = title_link.into();
            self
        }

        /// Enable a download icon that appears on the attachment. Clicking this saves the link to file.
        pub fn title_link_download(mut self, title_link_download: bool) -> Self {
            self.title_link_download = title_link_download;
            self
        }

        /// Set an image to display, will be "big" and easy to see.
        pub fn image_url<S: Into<String>>(mut self, image_url: S) -> Self {
            self.image_url = image_url.into();
            self
        }

        /// Audio file to play, supports any [html audio](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio).
        pub fn audio_url<S: Into<String>>(mut self, audio_url: S) -> Self {
            self.audio_url = audio_url.into();
            self
        }

        /// Video file to play, supports any [html video](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/video).
        pub fn video_url<S: Into<String>>(mut self, video_url: S) -> Self {
            self.video_url = video_url.into();
            self
        }

        /// Add an [`AttachmentField`] to the attachment.
        pub fn field(mut self, field: AttachmentField) -> Self {
            self.fields.push(field);
            self
        }

        /// Add multiple [`AttachmentField`] to the attachment.
        pub fn fields(mut self, fields: Vec<AttachmentField>) -> Self {
            self.fields = fields;
            self
        }
    }

    /// A Rocket.Chat Message payload.
    ///
    /// ### Example:
    /// ```
    /// use rocket_notify::rocketchat::Message;
    /// let message = Message::new()
    ///     .room_id("ByehQjC44FwMeiLbX")
    ///     // either room_id or channel can be used
    ///     // .channel("@myusername")
    ///     .text("Hello, world!")
    ///     .alias("My.Bot")
    ///     // Rocket.Chat preferentially uses avatar instead of emoji
    ///     // .avatar("https://foo.bar/computer.png)
    ///     .emoji(":computer:");
    /// ```
    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct Message {
        #[serde(alias = "roomId", skip_serializing_if = "String::is_empty")]
        room_id: String,

        #[serde(skip_serializing_if = "String::is_empty")]
        channel: String,

        #[serde(skip_serializing_if = "String::is_empty")]
        text: String,

        #[serde(skip_serializing_if = "String::is_empty")]
        alias: String,

        #[serde(skip_serializing_if = "String::is_empty")]
        emoji: String,

        #[serde(skip_serializing_if = "String::is_empty")]
        avatar: String,

        attachments: Vec<Attachment>,
    }

    impl Message {
        /// Create a new [`Message`].
        pub fn new() -> Self {
            Self {
                ..Default::default()
            }
        }

        /// Set the room id of where the message is to be sent. Resets
        pub fn room_id<S: Into<String>>(mut self, room_id: S) -> Self {
            self.room_id = room_id.into();
            if !self.channel.is_empty() && !self.room_id.is_empty() {
                self.channel = String::default();
            }
            self
        }

        /// Set the channel where the message is to be sent. Set the channel where the message is to be sent, with the prefix in front of it. `#` refers to channel, `@` refers to a username
        pub fn channel<S: Into<String>>(mut self, channel: S) -> Self {
            self.channel = channel.into();
            if !self.room_id.is_empty() && !self.channel.is_empty() {
                self.room_id = String::default();
            }
            self
        }

        /// Set the text of the message. Text is optional because of attachments.
        pub fn text<S: Into<String>>(mut self, text: S) -> Self {
            self.text = text.into();
            self
        }

        /// Set the message's name to appear as the given alias. The posting your username will still display.
        pub fn alias<S: Into<String>>(mut self, alias: S) -> Self {
            self.alias = alias.into();
            self
        }

        /// Set the avatar on the message to be an emoji.
        pub fn emoji<S: Into<String>>(mut self, emoji: S) -> Self {
            self.emoji = emoji.into();
            self
        }

        /// Set the avatar of the message to the provided image url.
        pub fn avatar<S: Into<String>>(mut self, avatar: S) -> Self {
            self.avatar = avatar.into();
            self
        }

        /// Add an [`Attachment`] to the message
        pub fn attachment(mut self, attachment: Attachment) -> Self {
            self.attachments.push(attachment);
            self
        }

        /// Add multiple [`Attachment`] to the message
        pub fn attachments(mut self, attachments: Vec<Attachment>) -> Self {
            self.attachments = attachments;
            self
        }
    }

    /// A Rocket.Chat Message API Client
    #[derive(Debug)]
    pub struct Client {
        url: String,
    }

    impl Client {
        /// Create a new Rocket.Chat [`Client`].
        ///
        /// ### Example:
        /// ```
        /// use rocket_notify::rocketchat::Client;
        /// let client = Client::new("https://my.rocket.com/hooks/db78d646/b072678678e8c74a");
        /// ```
        pub fn new<S: Into<String>>(url: S) -> Client {
            Client { url: url.into() }
        }

        /// POST a [`Message`] to the Rocket.Chat Message API
        ///
        /// ### Example
        /// ```no_run
        /// use rocket_notify::rocketchat::{Client, Message};
        ///
        /// let message = Message::new()
        ///     .channel("@myusername")
        ///     .text("Hello, world!");
        ///
        /// Client::new("https://my.rocket.com/hooks/db78d646/b072678678e8c74a")
        ///     .post_message(&message)
        ///     .unwrap();
        /// ```
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

    #[derive(Debug, Default, Deserialize)]
    pub struct ResponseMessageU {
        #[serde(alias = "_id", default)]
        id: String,

        #[serde(default)]
        username: String,
    }

    impl ResponseMessageU {
        pub fn id(&self) -> &String {
            &self.id
        }

        pub fn username(&self) -> &String {
            &self.username
        }
    }

    #[derive(Debug, Default, Deserialize)]
    pub struct ResponseMessage {
        #[serde(default)]
        alias: String,

        #[serde(default)]
        msg: String,

        #[serde(alias = "parseUrls", default)]
        parse_urls: bool,

        #[serde(default)]
        groupable: bool,

        #[serde(default)]
        u: ResponseMessageU,

        #[serde(default)]
        ts: DateTime<Local>,

        #[serde(default)]
        rid: String,

        #[serde(alias = "updatedAt", default)]
        updated_at: DateTime<Local>,

        #[serde(alias = "_id", default)]
        id: bool,
    }

    impl ResponseMessage {
        pub fn alias(&self) -> &str {
            self.alias.as_ref()
        }

        pub fn msg(&self) -> &str {
            self.msg.as_ref()
        }

        pub fn parse_urls(&self) -> bool {
            self.parse_urls
        }

        pub fn groupable(&self) -> bool {
            self.groupable
        }

        pub fn u(&self) -> &ResponseMessageU {
            &self.u
        }

        pub fn ts(&self) -> DateTime<Local> {
            self.ts
        }

        pub fn rid(&self) -> &str {
            self.rid.as_ref()
        }

        pub fn updated_at(&self) -> DateTime<Local> {
            self.updated_at
        }

        pub fn id(&self) -> bool {
            self.id
        }
    }

    /// Rocket.Chat API Response
    ///
    /// ### Example:
    /// ```no_run
    /// use std::io;
    /// use rocket_notify::rocketchat::{Client, Message};
    ///
    /// let message = Message::new()
    ///     .channel("@myusername")
    ///     .text("Hello, world!");
    ///
    /// let response = Client::new("https://my.rocket.com/hooks/db78d646/b072678678e8c74a")
    ///     .post_message(&message)
    ///     .unwrap();
    ///
    /// if response.success() {
    ///     // Action
    /// }
    /// ```
    #[derive(Debug, Deserialize)]
    pub struct Response {
        #[serde(default)]
        ts: DateTime<Local>,

        #[serde(default)]
        channel: String,

        #[serde(default)]
        message: ResponseMessage,

        #[serde(default)]
        success: bool,

        #[serde(default)]
        error: String,

        #[serde(default, alias = "errorType")]
        error_type: String,
    }

    impl Response {
        pub fn ts(&self) -> DateTime<Local> {
            self.ts
        }

        pub fn channel(&self) -> &String {
            &self.channel
        }

        pub fn message(&self) -> &ResponseMessage {
            &self.message
        }

        pub fn success(&self) -> bool {
            self.success
        }

        pub fn error_message(&self) -> &String {
            &self.error
        }

        pub fn error_type(&self) -> &String {
            &self.error_type
        }
    }

    impl Display for Response {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.success {
                f.write_str("ok")
            } else if self.error.is_empty() && !self.error_type.is_empty() {
                f.write_str(&self.error_type)
            } else if self.error.is_empty() && self.error_type.is_empty() {
                f.write_str("unknown error")
            } else {
                f.write_str(&self.error)
            }
        }
    }
}
