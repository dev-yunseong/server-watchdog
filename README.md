# Server Watchdog

A server monitoring and remote control tool via messenger.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Configuration](#configuration)
  - [Server Management](#server-management)
  - [Client Management](#client-management)
  - [Event Management](#event-management)
  - [Password Management](#password-management)
- [Usage](#usage)
  - [Available Commands](#available-commands)
  - [Running the Watchdog](#running-the-watchdog)


## Features

- Monitor server health and logs.
- Remote control servers via messenger bots (e.g., Telegram).
- Get notified when specific keywords are found in server logs or health checks.
- Secure access with password-based authentication.

## Installation

To build the project, you need Rust and Cargo installed. If you don't have them, you can install them from [rust-lang.org](https://www.rust-lang.org/tools/install).

Once Rust and Cargo are installed, clone the repository and build the project:

```bash
cargo install server-watchdog 
```

If you encounter issues during installation, please refer to the [Troubleshooting Guide](TROUBLESHOOTING.md).

## Configuration

The `server-watchdog` CLI is used to configure the application.

### Server Management

- **Add a server:**
  ```bash
  server-watchdog server add
  ```
- **List servers:**
  ```bash
  server-watchdog server list
  ```

### Client Management

- **Add a client:**
  ```bash
  server-watchdog client add
  ```
- **List clients:**
  ```bash
  server-watchdog client list
  ```

### Event Management

- **Add an event:**
  ```bash
  server-watchdog event add
  ```
  You will be prompted to enter the event's details (name, type, target server, keyword).
- **List events:**
  ```bash
  server-watchdog event list
  ```
- **Remove an event:**
  ```bash
  server-watchdog event remove <event_name>
  ```

### Password Management

- **Set the password:**
  ```bash
  server-watchdog password set <password>
  ```

## Usage

### Available Commands

Once the watchdog is running, you can interact with it through the configured messenger client (e.g., Telegram).

- **/register `<password>`**: Registers you to use the bot.
    - `password`: The password you set for the bot.

- **/alarm `add` `<event_name>`**: Adds an alarm for a pre-configured event.
- **/alarm `remove` `<event_name>`**: Removes an alarm for a pre-configured event.
- **/alarm `list`**: Lists all active alarms.

- **/event `[list]`**: Lists all configured events.
    - `list` (optional): Displays a list of all configured events. If omitted, acts the same as `/event list`.

- **/logs `<server_name>` `<lines>`**: Fetches the last `<lines>` of logs from the specified server.
    - `server_name`: The name you assigned to the server.
    - `lines`: The number of log lines to retrieve.

- **/health `[server_name]`**: Checks the health of the specified server. If no server name is provided, it will check all registered servers.
    - `server_name` (optional): The name you assigned to the server.


### Running the Watchdog

To start the server watchdog application:

```bash
server-watchdog run
```