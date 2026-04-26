# Portico

A Rust-based tool for managing SSH port forwarding with real-time monitoring and a terminal user interface.

[中文版本](./README_CN.md)

## Features

- Manage multiple SSH servers and port forwardings via YAML configuration
- Support both password and private key authentication
- Real-time monitoring of port forwarding status
- Terminal User Interface (TUI) for visualizing connections and traffic
- Automatic cleanup on exit

## Installation

```bash
git clone https://github.com/yourusername/portico.git
cd portico
cargo build --release
```

## Usage

1. Create a configuration file (see [examples/config.yaml](./examples/config.yaml) for reference)
2. Run Portico:

```bash
./target/release/portico -f your-config.yaml
```

## Configuration

Portico uses YAML configuration files. Here's a basic example:

```yaml
timeout:
  connect_timeout_secs: 5
  read_timeout_secs: 10
  write_timeout_secs: 10

servers:
  - name: "server1"
    host: "192.168.1.100"
    port: 22
    username: "user"
    password: "password"
    forwardings:
      - local_port: 8080
        remote_host: "localhost"
        remote_port: 80
```

For more examples, see the [examples](./examples) directory.

## License

This project is licensed under the MIT License.
