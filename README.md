# Server Watchdog

A server monitoring and remote control tool via messenger.

## Installation

To build the project, you need Rust and Cargo installed. If you don't have them, you can install them from [rust-lang.org](https://www.rust-lang.org/tools/install).

Once Rust and Cargo are installed, clone the repository and build the project:

```bash
cargo install server-watchdog 
```

If you encounter issues during installation, please refer to the [Troubleshooting Guide](TROUBLESHOOTING.md).

## Configuration

### Server Management

To add a new server to be monitored:

```bash
server-watchdog server add
```

You will be prompted to enter the server's details.

To list the currently configured servers:

```bash
server-watchdog server list
```

### Client Management

To add a new client (e.g., for Telegram notifications):

```bash
server-watchdog client add
```

You will be prompted for the client's details.

To list the currently configured clients:

```bash
server-watchdog client list
```

### Password Management

To set the password for the bot:

```bash
server-watchdog password set <password>
```

You will be prompted to enter the password.


## Usage

Once the watchdog is running, you can interact with it through the configured messenger client (e.g., Telegram).

### Available Commands

- `/register <password>`: Registers you to use the bot.

  - `password`: The password you set for the bot.
  
- `/logs <server_name> <lines>`: Fetches the last `<lines>` of logs from the specified server.

  - `server_name`: The name you assigned to the server during configuration.
  - `lines`: The number of log lines to retrieve.

  Example:
  ```
  /logs my-web-server 100
  ```

- `/health [server_name]`: Checks the health of the specified server. If no server name is provided, it will check all registered servers.

  - `server_name` (optional): The name you assigned to the server during configuration.

  Examples:
  ```
  /health my-web-server
  ```
  ```
  /health
  ```

### Running the Watchdog

To start the server watchdog application:

```bash
server-watchdog run
```