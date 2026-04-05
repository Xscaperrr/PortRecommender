# Port Recommender

`port-recommender` is a cross-platform Rust CLI that turns a service name into a deterministic, usable port number.

It is designed for local development, scripts, and lightweight service discovery where you want a stable port recommendation without manually choosing one.

## Features

- Maps a name to a port by hashing the input with `SHA-256`.
- Scans only the safe user-port range: `1024-65535`.
- Excludes built-in common ports such as `22`, `80`, `443`, `3306`, `5432`, `6379`, `8080`, `8443`, `27017`, and more.
- Excludes ports already occupied on the current machine.
- Supports `tcp`, `udp`, or `both` occupancy checks.
- Works on Windows, Linux, and macOS.
- Produces stable results for the same name when the exclusion set is unchanged.

## How It Works

1. The input name is hashed with `SHA-256`.
2. The first 8 bytes of the hash are converted into a starting port inside `1024-65535`.
3. The tool excludes:
   - common built-in ports,
   - ports already in use on the current machine for the selected protocol scope.
4. If the starting port is unavailable, it probes forward one port at a time and wraps at the end of the range.
5. The first available port is printed to stdout.

The recommended port can vary across machines or at different times on the same machine because local listening ports are part of the exclusion set.

## Usage

### Run with Cargo

```bash
cargo run -- <name>
```

Example:

```bash
cargo run -- example-service
```

### Use the compiled binary

```bash
port-recommender <name>
```

### Protocol options

By default, the tool checks both TCP and UDP.

```bash
port-recommender <name> --protocol both
port-recommender <name> --protocol tcp
port-recommender <name> --protocol udp
```

Examples:

```bash
port-recommender api-gateway
port-recommender redis-shadow --protocol tcp
port-recommender telemetry-agent --protocol udp
```

### Output

On success, the CLI prints only the port number:

```text
15251
```

On failure, it prints an error message to stderr and exits with a non-zero status.

## Build and Package

### Development build

```bash
cargo build
```

### Release build

```bash
cargo build --release
```

Generated binaries:

- Windows: `target/release/port-recommender.exe`
- Linux: `target/release/port-recommender`
- macOS: `target/release/port-recommender`

### Install locally

This installs the binary into Cargo's local bin directory so it can be used from your shell.

```bash
cargo install --path .
```

### Package source for distribution

This creates a Cargo package archive:

```bash
cargo package
```

### Build for a specific target

Examples:

```bash
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target aarch64-apple-darwin
```

The produced binary will be placed under:

```text
target/<target-triple>/release/
```

## Test

Run all tests:

```bash
cargo test
```

CI is configured to run tests on Windows, Linux, and macOS.

## Project Structure

- `src/main.rs`: CLI entrypoint
- `src/cli.rs`: command-line argument parsing
- `src/recommender.rs`: hash-to-port mapping and probing logic
- `src/ports.rs`: local occupied-port detection
- `src/common_ports.rs`: built-in common-port denylist
- `tests/cli.rs`: CLI integration tests

## License

This project is licensed under the MIT License.

See [LICENSE](LICENSE) for details.
