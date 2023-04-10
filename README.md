<h1 align="center"><code>rocket-notify</code></h1>

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
Simple command line client to send messages to Rocket.Chat

## Installation

### Homebrew
`rocket-notify` can be installed via the [homebrew](https://brew.sh/) package manager on macOS and linux
```sh
brew tap ianmclinden/homebrew-extras
brew install rocket-notify
```

### Linux
```sh
curl -L https://github.com/ianmclinden/rocket-notify/releases/latest/download/rocket-notify-x86_64-unknown-linux-gnu.tar.gz | tar zx
sudo install -t /usr/local/bin rocket-notify
```

Binaries for `linux/arm64` and `linux/arm` are also available from the [releases page](https://github.com/ianmclinden/rocket-notify/releases/latest).

### macOS
```sh
curl -L https://github.com/ianmclinden/rocket-notify/releases/latest/download/rocket-notify-x86_64-apple-darwin.tar.gz | tar zx
sudo cp rocket-notify /usr/local/bin/
```

Binaries for `darwin/arm64` are also available from the [releases page](https://github.com/ianmclinden/rocket-notify/releases/latest).

### Windows
A `rocket-notify.exe` for `windows/amd64` is available as a tarball from the [releases page](https://github.com/ianmclinden/rocket-notify/releases/latest). 

## Usage

### Webhook Creation
In order for Rocket.Chat to receive webhooks, a valid webhook URL must be provisioned by a Rocket.Chat admin. The URL includes permissions that restricts what users can send and recieve messages.

Once a webhook URL is acquired, it can be added to the environment as `ROCKET_NOTIFY_URL`. To do this automatically (in bash), add the following to `~/.bashrc` or `~/.bash_profile`
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

For a full list of commands
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