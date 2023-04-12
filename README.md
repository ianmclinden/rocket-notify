<h1 align="center"><code>🚀 rocket-notify</code></h1>

<p align="center">
    <a href="https://github.com/ianmclinden/rocket-notify/actions/workflows/rust.yml?query=branch%3Amain+" title="Build status">
        <img src="https://github.com/ianmclinden/rocket-notify/actions/workflows/rust.yml/badge.svg?branch=main">
    </a>
    <a href="https://github.com/ianmclinden/rocket-notify/releases/latest" title="GitHub release">
        <img src="https://img.shields.io/github/release/ianmclinden/rocket-notify.svg">
    </a>
    <a href="https://hub.docker.com/repository/docker/ijmclinden/rocket-notify/tags" title="Docker image">
        <img src="https://img.shields.io/badge/rocket--notify-%40latest-blue?&logo=docker">
    </a>
    <a href="https://opensource.org/licenses/MIT" title="License: MIT">
        <img src="https://img.shields.io/badge/License-MIT-blue.svg">
    </a>
</p>

## About
🚀 Simple command-line client to send messages to Rocket.Chat

## Installation

### Homebrew
`rocket-notify` can be installed via the [homebrew](https://brew.sh/) package manager on macOS and linux
```sh
brew tap ianmclinden/extras
brew install rocket-notify
```

### Linux
```sh
curl -L https://github.com/ianmclinden/rocket-notify/releases/latest/download/rocket-notify-x86_64-unknown-linux-gnu.tar.gz | tar zx
sudo install -t /usr/local/bin rocket-notify
```

Binaries for [`linux/arm64`](https://github.com/ianmclinden/rocket-notify/releases/latest/download/rocket-notify-aarch64-unknown-linux-gnu.tar.gz) and [`linux/arm`](https://github.com/ianmclinden/rocket-notify/releases/latest/download/rocket-notify-arm-unknown-linux-gnueabihf.tar.gz) are also available from the [releases page](https://github.com/ianmclinden/rocket-notify/releases/latest).

### macOS
```sh
curl -L https://github.com/ianmclinden/rocket-notify/releases/latest/download/rocket-notify-x86_64-apple-darwin.tar.gz | tar zx
sudo cp rocket-notify /usr/local/bin/
```

Binaries for [`darwin/arm64` (Apple Silicon)](
https://github.com/ianmclinden/rocket-notify/releases/latest/download/rocket-notify-aarch64-apple-darwin.tar.gz) are also available from the [releases page](https://github.com/ianmclinden/rocket-notify/releases/latest).

### Windows
A `rocket-notify.exe` for [`windows/amd64`](https://github.com/ianmclinden/rocket-notify/releases/latest/download/rocket-notify-x86_64-pc-windows-gnu.tar.gz) is available as a tarball from the [releases page](https://github.com/ianmclinden/rocket-notify/releases/latest). 

## Usage

### Webhook Creation
In order for Rocket.Chat to receive webhooks, a valid webhook URL must be provisioned by a Rocket.Chat admin. The URL is associated with permissions that restricts what users can send and recieve messages.

Once a webhook URL is acquired, it can be added to the environment as `ROCKET_NOTIFY_URL`. To do this automatically (in bash), add the following to `~/.bashrc`, or `~/.bash_profile`, or `/etc/environment`.
```sh
export ROCKET_NOTIFY_URL=https://{webhookURL}
```

### Cli Usage

```sh
rocket-notify [options] <channel> <message>
```

For example
```sh
rocket-notify --icon ":tada:" --alias "Project Builder" @myusername "Build Finished!"
```

For a full list of options
```sh
rocket-notify --help
```

### Docker
The cli can also be invoked via a Docker container. In this case, parameters can be passed via the environment, for example:
```sh
docker run \
    -e ROCKET_NOTIFY_URL="https://{webhookURL}" \
    -e ROCKET_NOTIFY_ALIAS="Project Builder" \
    -e ROCKET_NOTIFY_ICON=":tada:" \
    -e ROCKET_NOTIFY_CHANNEL="@myusername" \
    -e ROCKET_NOTIFY_MESSAGE="Build Finished!" \
    --rm ijmclinden/rocket-notify:latest
```

## Reference


<details>
  <summary>Expand</summary>

### Environment Variables

#### `ROCKET_NOTIFY_URL` *(Required)*
Rocket.Chat webhook URL

#### `ROCKET_NOTIFY_CHANNEL` *(Required)*
Channel to which the mesage will be sent, like '#general' or '@eric'

#### `ROCKET_NOTIFY_MESSAGE` *(Required)*
Message to send

#### `ROCKET_NOTIFY_ALIAS`
Alias for the message sender

**Default** `<hostname>`

#### `ROCKET_NOTIFY_AVATAR`
Set the sender's icon to the provided URL. Supercedes [`ROCKET_NOTIFY_ICON`](#rocket_notify_icon)

#### `ROCKET_NOTIFY_COLOR`
Color of the message header

**Default** `darkgrey`

#### `ROCKET_NOTIFY_ICON`
Set the sender's icon to an emoji

**Default** `:computer:`

#### `ROCKET_NOTIFY_MINIMIZE`
Send with message block collapsed

#### `ROCKET_NOTIFY_TITLE`
Title of the message
</details>
