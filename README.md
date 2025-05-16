# BakaPanel

BakaPanel is a lightweight admin panel for Minecraft networks

## Features

- A query service which notifies of changes in the state of your servers
- Frontend to view the state of your host and servers

## TODO

- Add RCON command support
- Service to manage the uptime of the servers
- Page to test the event commands
- Automate the build of the frontend when building the backend

## Build

To build the program, first you need to build the frontend to an HTML file which will be served by the Rust program.

This can be done by running the build command in the `/frontend` directory.

```bash
npm run build # Or with your manager of preference
```

Then you can simply build the Rust program.

```bash
cargo build --release
```

## Config File

When the program is executed, it searches for a config file named `config.toml` in the current directory.

This config file is provided with a template to send discord messages to a channel on events via curl.

`./config.toml`

```toml
# Name of your network
name = "Network"

# Port for the API to listen to
port = 3000

# How often should the query service be executed (in seconds)
refresh_time = 30

# List of your servers
[servers]

[servers.localhost]
    host = "localhost"
    port = 25565
    rcon = { port = 25569, password = "password" }

[servers.kimu]
    host = "play.kimu.moe"
    port = 25565

# Commands to be ran on different events
[actions]
# Placeholders: {ERROR}
app_error = """
            curl --request POST \
                --url https://discord.com/api/v10/channels/<CHANNEL_ID>/messages \
                --header 'Authorization: Bot <BOT_TOKEN>' \
                --header 'Content-Type: application/json' \
                --data "{ \\"content\\": \\"An internal error ocurred: {ERROR}\\" }"
            """

# Placeholders: {SERVER_NAME} {SERVER_HOST} {SERVER_PORT}
server_off = """
            curl --request POST \
                --url https://discord.com/api/v10/channels/<CHANNEL_ID>/messages \
                --header 'Authorization: Bot <BOT_TOKEN>' \
                --header 'Content-Type: application/json' \
                --data "{ \\"content\\": \\"Error querying server: {SERVER_NAME} at {SERVER_HOST}:{SERVER_PORT}\\" }"
            """

# Placeholders: {SERVER_NAME} {SERVER_HOST} {SERVER_PORT}
server_on = """
            curl --request POST \
                --url https://discord.com/api/v10/channels/<CHANNEL_ID>/messages \
                --header 'Authorization: Bot <BOT_TOKEN>' \
                --header 'Content-Type: application/json' \
                --data "{ \\"content\\": \\"Server is back is online: {SERVER_NAME} at {SERVER_HOST}:{SERVER_PORT}\\" }"
            """

```
