use clap::Parser;
use csscolorparser::Color;
use rocket_notify::rocketchat::{Attachment, Client, Message};
use std::{process, sync::LazyLock};

static HOSTNAME: LazyLock<String> = LazyLock::new(|| {
    hostname::get()
        .unwrap_or("rocket-notify".into())
        .to_string_lossy()
        .replace(".local", "")
});

#[derive(Debug, Parser)]
#[command(version, about, long_about, author)]
struct Args {
    /// Rocket.Chat webhook URL
    #[arg(short, long, env = "ROCKET_NOTIFY_URL", hide = true)]
    url: Option<String>,

    /// Alias for the message sender
    #[arg(short, long, default_value_t = HOSTNAME.clone(), env = "ROCKET_NOTIFY_ALIAS")]
    alias: String,

    /// Set the sender's icon to an emoji
    #[arg(
        short = 'i',
        long = "icon",
        default_value = ":computer:",
        env = "ROCKET_NOTIFY_ICON"
    )]
    emoji: String,

    /// Set the sender's icon to the provided URL (replaces -i,--icon)
    #[arg(short = 'A', long = "avatar", env = "ROCKET_NOTIFY_AVATAR")]
    avatar: Option<String>,

    /// Title of the message
    #[arg(short, long, default_value = "", env = "ROCKET_NOTIFY_TITLE")]
    title: String,

    /// Color of the message header
    #[arg(short, long, default_value = "darkgrey", env = "ROCKET_NOTIFY_COLOR")]
    color: Color,

    /// Send with message block collapsed
    #[arg(short, long, env = "ROCKET_NOTIFY_MINIMIZE")]
    minimize: bool,

    /// Channel to which the mesage will be sent, like '#general' or '@eric'
    #[arg(env = "ROCKET_NOTIFY_CHANNEL")]
    channel: String,

    /// Message to send
    #[arg(env = "ROCKET_NOTIFY_MESSAGE")]
    message: String,
}

// Convenience fns for pretty-printing errors
fn print_err<S: std::fmt::Display>(msg: S) {
    eprintln!("\x1b[1;31merror:\x1b[m {msg}");
}
fn print_success<S: std::fmt::Display>(msg: S) {
    println!("\x1b[0;32msuccess:\x1b[m {msg}");
}

fn main() {
    let args = Args::parse();

    let Some(url) = args.url else {
        print_err(
            "ROCKET_NOTIFY_URL not set.\nPlease acquire a webhook url from an admin, and then\n  `export ROCKET_NOTIFY_URL=https://{webhookURL}`",
        );
        process::exit(1);
    };

    let mut message = Message::new()
        .channel(&args.channel)
        .text(&args.title)
        .alias(&args.alias)
        .attachment(
            Attachment::new()
                .title(if args.minimize { &args.title } else { "" })
                .text(&args.message)
                .color(args.color.to_css_hex())
                .collapsed(args.minimize),
        );

    message = if args.avatar.is_some() {
        message.avatar(args.avatar.unwrap())
    } else {
        message.emoji(args.emoji)
    };

    match Client::new(&url).post_message(&message) {
        Err(e) => {
            print_err(e);
            process::exit(1);
        }
        Ok(_) => print_success("message sent!"),
    }
}
