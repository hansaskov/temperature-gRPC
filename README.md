# Temperature gRPC

This project demonstrates how to use gRPC to securely transmit system information in a type-safe manner. It consists of a server and client application built in Rust.

## Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) - The Rust programming language and Cargo package manager
- [Protoc](https://github.com/protocolbuffers/protobuf/releases) - Protocol Buffers compiler
- [Libre Hardware Monitor](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor) - Necessary for getting cpu temp on the client. 

> [!WARNING]
> Libre Hardware Monitor must be running on the client for the program to work

> **Note:**
> Make sure to add the Protoc binary to your system's PATH environment variable.

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/hansaskov/temperature-gRPC.git
   cd tempurature-sender/ingestion
   ```

## Usage

### Start the Server

Easiest way to start the tRPC server i with docker, by running the following command:

``` bash
docker compose up --build
```

or you can run it locally with an external database url.

> **Note:**
> Only Postgres is supported

```bash
cargo run --bin server -- --db-url postgres://username:password@timescaledb:5432/mydatabase
```

The server will start and listen for incoming connections.

### Start the Client

To start the tRPC client and connect to the server, run:

```bash
cargo run --bin client
```

The client will establish a connection with the server and begin exchanging system information.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
