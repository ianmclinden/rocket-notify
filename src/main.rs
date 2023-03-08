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
    #[structopt(short, long, default_value = &HOSTNAME)]
    alias: String,

    /// Set the sender's icon to an emoji
    #[structopt(short = "i", long = "icon", default_value = ":computer:")]
    emoji: String,

    /// Title of the message
    #[structopt(short, long, default_value)]
    title: String,

    /// Color of the message header
    #[structopt(short, long, default_value = "darkgrey")]
    color: Color,

    /// Send with message block collapsed
    #[structopt(short, long)]
    minimize: bool,

    /// Channel to which the mesage will be sent, like '#general' or '@eric'
    channel: String,

    /// Message to send
    message: String,
}

/// Convenience macro for pretty-printing errors
fn print_err(msg: &str) {
    eprintln!("\x1b[1;31merror:\x1b[m {msg}");
}

fn print_success(msg: &str) {
    println!("\x1b[0;32msuccess:\x1b[m {msg}");
}

fn main() {
    let args = Args::from_args();

    if args.url.is_empty() {
        print_err("ROCKET_NOTIFY_URL not set.\nPlease acquire a webhook url from an admin, and then\n  `export ROCKET_NOTIFY_URL=https://{webhookURL}`");
        process::exit(1);
    }

    let message = Message::new()
        .channel(&args.channel)
        .text(&args.title)
        .emoji(&args.emoji)
        .alias(&args.alias)
        .attachment(
            Attachment::new()
                .title(if args.minimize { &args.title } else { "" })
                .text(&args.message)
                .color(args.color.to_hex_string())
                .collapsed(args.minimize),
        );

    match Client::new(&args.url).post_message(&message) {
        Err(e) => {
            print_err(&e.to_string());
            process::exit(1);
        }
        Ok(r) if !r.is_success() => {
            print_err(&r.to_string());
            process::exit(1);
        }
        Ok(_) => print_success("message sent!"),
    };
}
