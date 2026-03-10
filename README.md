# rget

A minimal `wget` clone written in Rust. Downloads files from the web with a real-time progress bar.

## Features

- Downloads files over HTTP/HTTPS
- Live progress bar with speed and ETA
- Spinner fallback when content length is unknown
- Saves file to disk using the filename from the URL

## Installation

Make sure you have [Rust](https://rustup.rs/) installed, then:

```bash
git clone https://github.com/turnbulll/rget
cd rget
cargo build --release
```

The compiled binary will be at `./target/release/rget`.

Optionally, install it to your PATH:

```bash
cargo install --path .
```

## Usage

```bash
rget <URL>
```

### Examples

```bash
# Download a small test file
rget https://httpbin.org/bytes/1024

# Download a large file with progress bar
rget https://speed.hetzner.de/100MB.bin

# Download a CSV
rget https://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.csv
```

The file is saved in the current directory using the filename from the URL.

## Dependencies

| Crate       | Purpose                        |
|-------------|--------------------------------|
| `clap`      | CLI argument parsing           |
| `reqwest`   | HTTP client                    |
| `indicatif` | Progress bar and spinner       |
| `console`   | Terminal colours               |

## Building from Source

Requires Rust 2021 edition or later.

```bash
cargo build          # debug build
cargo build --release  # optimised release build
cargo clippy         # run linter
cargo test           # run tests
```

## Project Structure

```
rget/
├── Cargo.toml       # dependencies and project metadata
└── src/
    └── main.rs      # all source code
```

## License

MIT
