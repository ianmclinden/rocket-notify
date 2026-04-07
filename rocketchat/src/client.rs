use thiserror::Error;

use crate::{request::Message, response::Response};

#[derive(Debug, Error)]
pub enum ClientError {
    /// The client was configured with an invalid url
    #[error("invalid url: {0}")]
    InvalidUrl(String),

    /// An error sending the request
    #[cfg(feature = "reqwest")]
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),

    /// An error sending the request
    #[cfg(feature = "ureq")]
    #[error(transparent)]
    RequestError(#[from] ureq::Error),

    /// The response from the server was an error
    #[error("server response was an error: {0}")]
    ServerError(String),
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
    /// use rocketchat::client::Client;
    /// let client = Client::new("https://my.rocket.com/hooks/db78d646/b072678678e8c74a");
    /// ```
    pub fn new<S: Into<String>>(url: S) -> Self {
        Client { url: url.into() }
    }

    /// POST a [`Message`] to the Rocket.Chat Message API
    ///
    /// ### Example
    /// ```no_run
    /// use rocketchat::{
    ///     client::Client,
    ///     request::{Attachment, Message},
    /// };
    ///
    /// let message = Message::new()
    ///     .channel("@myusername")
    ///     .text("Hello, world!");
    ///
    /// Client::new("https://my.rocket.com/hooks/db78d646/b072678678e8c74a")
    ///     .send(&message)
    ///     .unwrap();
    /// ```
    /// # Errors
    /// Returns a [`ClientError`] sending the message fails, or if posting works but a server-side error occurs.
    pub fn send(&self, message: &Message) -> Result<Response, ClientError> {
        self.send_internal(message)
    }

    #[cfg(feature = "reqwest")]
    fn send_internal(&self, message: &Message) -> Result<Response, ClientError> {
        let res = reqwest::blocking::Client::new()
            .post(&self.url)
            .json(message)
            .send()?
            .json::<Response>()?;

        if res.success() {
            Ok(res)
        } else {
            Err(ClientError::ServerError(res.error_message().clone()))
        }
    }

    #[cfg(feature = "ureq")]
    fn send_internal(&self, message: &Message) -> Result<Response, ClientError> {
        let res = ureq::post(&self.url)
            .send_json(message)?
            .body_mut()
            .read_json::<Response>()?;

        if res.success() {
            Ok(res)
        } else {
            Err(ClientError::ServerError(res.error_message().clone()))
        }
    }
}
