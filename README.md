<h3 align="center">Starship Plugin POC</h3>
<p align="center">A proof-of-concept for a dynamically loaded plugin-based module system.</p>

---

> 👉 Comments and feedback are appreciated in Issues.

## Goals

These are the main goals of this POC:

- [x] Create an API for IPC between Starship and its plugins
- [ ] Compare performance between pipe and socket-based IPC

## Try it out

You can run the proof-of-concept locally with the following:

```sh
# Build all executables and run the server
cargo build && cargo run --bin server
```
