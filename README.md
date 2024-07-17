# Temperature gRPC

This project demonstrates how to use gRPC to securely transmit system information in a type-safe manner. It consists of a server and client application built in Rust.

## Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) - The Rust programming language and Cargo package manager
- [Protoc](https://github.com/protocolbuffers/protobuf/releases) - Protocol Buffers compiler
- [Libre Hardware Monitor](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor) - Necessary for getting cpu temp on the client. 

> **Note:**
> Make sure to add the Protoc binary to your system's PATH environment variable.

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/hansaskov/tempurature-sender.git
   cd tempurature-sender
   ```

## Usage

### Start the Server

To start the tRPC server, run the following command:

```bash
cargo run --bin server
```

The server will start and listen for incoming connections.

### Start the Client

To start the tRPC client and connect to the server, run:

```bash
cargo run --bin client
```

The client will establish a connection with the server and begin exchanging system information.

## Project Structure

- `src/bin/server.rs`: Contains the server implementation
- `src/bin/client.rs`: Contains the client implementation

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
