# Decent Backend
This is the backend for our the decent messaging app. This is expected to be run on either ARM or x64 servers such as Raspberry Pi.

## Build
If you develop on windows, idk good luck

__Install dependencies for libsignal:__
### Ubuntu/Debian
`sudo apt install clang libclang-dev cmake make protobuf-compiler`
### Fedora
`sudo dnf install clang smake make protobuf-devel`
### Rust
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Building:
`cargo build`

Running:
`cargo run`