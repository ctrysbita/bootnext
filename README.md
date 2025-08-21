# bootnext

bootnext is a simple Rust command-line tool to set the next UEFI boot entry to a specific operating system by manipulating UEFI variables.

## Usage

### Prerequisites

- Rust toolchain (https://rustup.rs/)
- Administrator/root privileges (required to modify UEFI variables)
- UEFI firmware (not supported on legacy BIOS)

### Build

To build for booting windows:

```sh
TARGET_OS=windows cargo build --release
```

### Run

On Linux:

```sh
sudo ./target/release/bootnext
```

On Windows:

```sh
.\target\release\bootnext.exe
```

## Configuration

- The target OS is set using the `TARGET_OS` environment variable when building the project.

## License

MIT

## Disclaimer

- Modifying UEFI variables can be risky. Use at your own risk.
- This tool is intended for advanced users familiar with UEFI systems.
