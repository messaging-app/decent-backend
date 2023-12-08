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

## Current Implementation
To obtain the UUID of a client, do a `GET` at the following url: `http://<ip-address>:8000/users`.

Logging in as a client is purely opening up a websocket with their UUID at the following URL: `ws://<ip-addr>:8000/ws/<UUID>`.

Once we are logged in we can send a message.
To send a message we must first know the other user's UUID, which  can also be received from the users endpoint.
After that, assuming the user is currently logged in, the request for the sender will look like:
```
<RECEIVER-UUID> This is a test
```

For the receiver, the result will be the following:
```
<SENDER-UUID> This is a test
```
