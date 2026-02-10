# Server Watchdog

A server monitoring and remote control tool via messenger.

## Installation

To build the project, you need Rust and Cargo installed. If you don't have them, you can install them from [rust-lang.org](https://www.rust-lang.org/tools/install).

Once Rust and Cargo are installed, clone the repository and build the project:

```bash
cargo install server-watchdog 
```

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

## Usage

Once the watchdog is running, you can interact with it through the configured messenger client (e.g., Telegram).

### Available Commands

- `/logs <server_name> <lines>`: Fetches the last `<lines>` of logs from the specified server.

  - `server_name`: The name you assigned to the server during configuration.
  - `lines`: The number of log lines to retrieve.

  Example:
  ```
  /logs my-web-server 100
  ```

### Running the Watchdog

To start the server watchdog application:

```bash
server-watchdog run
```