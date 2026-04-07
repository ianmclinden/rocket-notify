//! [Rocket.Chat]() Message API Client
//!
//! - [Message API Documentation](https://developer.rocket.chat/reference/api/rest-api/endpoints/core-endpoints/chat-endpoints/postmessage)
//! - [Full API Documentation](https://developer.rocket.chat/reference/api)
//!
//! ### Example:
//! _Send a message and return the result_
//! ```no_run
//! use rocketchat::{
//!     client::Client,
//!     request::{Attachment, Message},
//! };
//!
//! let message = Message::new()
//!     .channel("@myuser")
//!     .alias("My.Bot")
//!     .emoji(":rocket:")
//!     .attachment(
//!         Attachment::new()
//!             .title("Hello, World")
//!             .text("A sample message body")
//!             .color("green")
//!             .collapsed(false),
//!     );
//!
//! let webhook_url = "https://my.rocket.com/hooks/db78d646/b072678678e8c74a";
//! let response = Client::new(webhook_url)
//!     .send(&message)
//!     .expect("Failed to send message");
//!
//! println!("OK: Posted to {}", response.channel());
//! ```

#[cfg(all(feature = "reqwest", feature = "ureq"))]
compile_error!("Both client features `reqwest` and `ureq` cannot both be enabled");
#[cfg(not(any(feature = "reqwest", feature = "ureq")))]
compile_error!("You must enable either the `reqwest` or `ureq` transport features");

#[cfg(all(feature = "rustls", feature = "native-tls"))]
compile_error!("Both client features `rustls` and `native-tls` cannot both be enabled");
#[cfg(not(any(feature = "rustls", feature = "native-tls")))]
compile_error!("You must enable either the `rustls` or `native-tls` transport features");

pub mod client;
pub mod request;
pub mod response;
