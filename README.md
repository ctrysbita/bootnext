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
cargo build --release --features windows
```

### Run

On Linux:
```sh
sudo ./target/release/bootnext
```

On Windows (run as Administrator):
```sh
.\target\release\bootnext.exe
```

## Configuration
- The target OS is selected at compile time using features:
  - `ubuntu` for Ubuntu
  - `windows` for Windows

## License
MIT

## Disclaimer
- Modifying UEFI variables can be risky. Use at your own risk.
- This tool is intended for advanced users familiar with UEFI systems.
