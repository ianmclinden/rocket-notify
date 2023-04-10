use csscolorparser::Color;
use rocket_notify::rocketchat::{Attachment, Client, Message};
use std::process;
use structopt::{lazy_static::lazy_static, StructOpt};

lazy_static! {
    static ref HOSTNAME: String = hostname::get()
        .unwrap_or("rocket-notify".into())
        .to_string_lossy()
        .replace(".local", "");
}

#[derive(Debug, StructOpt)]
#[structopt(about)]
struct Args {
    /// Rocket.Chat webhook URL
    #[structopt(short, long, default_value, env = "ROCKET_NOTIFY_URL", hidden = true)]
    url: String,

    /// Alias for the message sender
    #[structopt(short, long, default_value = &HOSTNAME, env = "ROCKET_NOTIFY_ALIAS")]
    alias: String,

    /// Set the sender's icon to an emoji
    #[structopt(
        short = "i",
        long = "icon",
        default_value = ":computer:",
        env = "ROCKET_NOTIFY_ICON"
    )]
    emoji: String,

    /// Set the sender's icon to the provided URL (replaces -i,--icon)
    #[structopt(short = "A", long = "avatar", env = "ROCKET_NOTIFY_AVATAR")]
    avatar: Option<String>,

    /// Title of the message
    #[structopt(short, long, default_value, env = "ROCKET_NOTIFY_TITLE")]
    title: String,

    /// Color of the message header
    #[structopt(short, long, default_value = "darkgrey", env = "ROCKET_NOTIFY_COLOR")]
    color: Color,

    /// Send with message block collapsed
    #[structopt(short, long, env = "ROCKET_NOTIFY_MINIMIZE")]
    minimize: bool,

    /// Channel to which the mesage will be sent, like '#general' or '@eric'
    #[structopt(env = "ROCKET_NOTIFY_CHANNEL")]
    channel: String,

    /// Message to send
    #[structopt(env = "ROCKET_NOTIFY_MESSAGE")]
    message: String,
}

/// Convenience macro for pretty-printing errors
fn print_err<S: std::fmt::Display>(msg: S) {
    eprintln!("\x1b[1;31merror:\x1b[m {msg}");
}

fn print_success<S: std::fmt::Display>(msg: S) {
    println!("\x1b[0;32msuccess:\x1b[m {msg}");
}

fn main() {
    let args = Args::from_args();

    if args.url.is_empty() {
        print_err("ROCKET_NOTIFY_URL not set.\nPlease acquire a webhook url from an admin, and then\n  `export ROCKET_NOTIFY_URL=https://{webhookURL}`");
        process::exit(1);
    }

    let mut message = Message::new()
        .channel(&args.channel)
        .text(&args.title)
        .alias(&args.alias)
        .attachment(
            Attachment::new()
                .title(if args.minimize { &args.title } else { "" })
                .text(&args.message)
                .color(args.color.to_hex_string())
                .collapsed(args.minimize),
        );

    message = match args.avatar.is_some() {
        true => message.avatar(args.avatar.unwrap()),
        false => message.emoji(args.emoji),
    };

    match Client::new(&args.url).post_message(&message) {
        Err(e) => {
            print_err(e);
            process::exit(1);
        }
        Ok(r) if !r.success() => {
            print_err(r);
            process::exit(1);
        }
        Ok(_) => print_success("message sent!"),
    };
}
