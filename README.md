# Server Watchdog

A server monitoring and remote control tool via messenger.

## Usage

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

### Running the Watchdog

To start the server watchdog application:

```bash
server-watchdog run
```