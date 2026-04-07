use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

/// A Rocket.Chat Message payload.
///
/// ### Example:
/// ```
/// use rocketchat::request::Message;
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
#[serde(rename_all = "camelCase")]
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
    emoji: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    avatar: String,

    attachments: Vec<Attachment>,
}

impl Message {
    /// Create a new [`Message`].
    #[must_use]
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Set the room id of where the message is to be sent. Resets
    #[must_use]
    pub fn room_id<S: Into<String>>(mut self, room_id: S) -> Self {
        self.room_id = room_id.into();
        if !self.channel.is_empty() && !self.room_id.is_empty() {
            self.channel = String::default();
        }
        self
    }

    /// Set the channel where the message is to be sent. Set the channel where the message is to be sent, with the prefix in front of it. `#` refers to channel, `@` refers to a username
    #[must_use]
    pub fn channel<S: Into<String>>(mut self, channel: S) -> Self {
        self.channel = channel.into();
        if !self.room_id.is_empty() && !self.channel.is_empty() {
            self.room_id = String::default();
        }
        self
    }

    /// Set the text of the message. Text is optional because of attachments.
    #[must_use]
    pub fn text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = text.into();
        self
    }

    /// Set the message's name to appear as the given alias. The sending your username will still display.
    #[must_use]
    pub fn alias<S: Into<String>>(mut self, alias: S) -> Self {
        self.alias = alias.into();
        self
    }

    /// Set the avatar on the message to be an emoji.
    #[must_use]
    pub fn emoji<S: Into<String>>(mut self, emoji: S) -> Self {
        self.emoji = emoji.into();
        self
    }

    /// Set the avatar of the message to the provided image url.
    #[must_use]
    pub fn avatar<S: Into<String>>(mut self, avatar: S) -> Self {
        self.avatar = avatar.into();
        self
    }

    /// Add an [`Attachment`] to the message
    #[must_use]
    pub fn attachment(mut self, attachment: Attachment) -> Self {
        self.attachments.push(attachment);
        self
    }

    /// Add multiple [`Attachment`] to the message
    #[must_use]
    pub fn attachments(mut self, attachments: Vec<Attachment>) -> Self {
        self.attachments = attachments;
        self
    }
}

/// Attachment fields that annotate an [`Attachment`]. Allows for "tables" or "columns" to be displayed on messages.
///
/// ### Example:
/// ```
/// use rocketchat::request::AttachmentField;
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
    #[must_use]
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Set whether the field should be a short field.
    #[must_use]
    pub fn short(mut self, short: bool) -> Self {
        self.short = short;
        self
    }

    /// Set the title of the field.
    #[must_use]
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = title.into();
        self
    }

    /// Set the value of the field, displayed underneath the title value.
    #[must_use]
    pub fn value<S: Into<String>>(mut self, value: S) -> Self {
        self.value = value.into();
        self
    }
}

/// Message attachment, containing additional data to be rendered in the message.
///
/// ### Example:
/// ```
/// use rocketchat::request::Attachment;
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
    #[must_use]
    pub fn new() -> Self {
        Self {
            ts: Local::now(),
            ..Default::default()
        }
    }

    /// Set the attachment left border color. Can be any [background-color](https://developer.mozilla.org/en-US/docs/Web/CSS/background-color) value.
    #[must_use]
    pub fn color<S: Into<String>>(mut self, color: S) -> Self {
        self.color = color.into();
        self.update_ts()
    }

    /// Set the text to display for the attachment, it is different than the message's text.
    #[must_use]
    pub fn text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = text.into();
        self.update_ts()
    }

    fn update_ts(mut self) -> Self {
        self.ts = Local::now();
        self
    }

    /// An image that displays to the left of the text, looks better when this is relatively small.
    #[must_use]
    pub fn thumb_url<S: Into<String>>(mut self, thumb_url: S) -> Self {
        self.thumb_url = thumb_url.into();
        self
    }

    /// Set the attachment time to a clickable link.
    #[must_use]
    pub fn message_link<S: Into<String>>(mut self, message_link: S) -> Self {
        self.message_link = message_link.into();
        self
    }

    /// Set the image, audio, and video sections to be collapsed.
    #[must_use]
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    /// Set the name of the message author.
    #[must_use]
    pub fn author_name<S: Into<String>>(mut self, author_name: S) -> Self {
        self.author_name = author_name.into();
        self
    }

    /// Set the author's name to a clickable link.
    #[must_use]
    pub fn author_link<S: Into<String>>(mut self, author_link: S) -> Self {
        self.author_link = author_link.into();
        self
    }

    /// Set a tiny icon to the left of the Author's name.
    #[must_use]
    pub fn author_icon<S: Into<String>>(mut self, author_icon: S) -> Self {
        self.author_icon = author_icon.into();
        self
    }

    /// Set the title for the attachment. Displays under the author.
    #[must_use]
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = title.into();
        self.update_ts()
    }

    /// Set the this attachment title to a clickable link.
    #[must_use]
    pub fn title_link<S: Into<String>>(mut self, title_link: S) -> Self {
        self.title_link = title_link.into();
        self
    }

    /// Enable a download icon that appears on the attachment. Clicking this saves the link to file.
    #[must_use]
    pub fn title_link_download(mut self, title_link_download: bool) -> Self {
        self.title_link_download = title_link_download;
        self
    }

    /// Set an image to display, will be "big" and easy to see.
    #[must_use]
    pub fn image_url<S: Into<String>>(mut self, image_url: S) -> Self {
        self.image_url = image_url.into();
        self
    }

    /// Audio file to play, supports any [html audio](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio).
    #[must_use]
    pub fn audio_url<S: Into<String>>(mut self, audio_url: S) -> Self {
        self.audio_url = audio_url.into();
        self
    }

    /// Video file to play, supports any [html video](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/video).
    #[must_use]
    pub fn video_url<S: Into<String>>(mut self, video_url: S) -> Self {
        self.video_url = video_url.into();
        self
    }

    /// Add an [`AttachmentField`] to the attachment.
    #[must_use]
    pub fn field(mut self, field: AttachmentField) -> Self {
        self.fields.push(field);
        self
    }

    /// Add multiple [`AttachmentField`] to the attachment.
    #[must_use]
    pub fn fields(mut self, fields: Vec<AttachmentField>) -> Self {
        self.fields = fields;
        self
    }
}
