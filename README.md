<h3 align="center">Starship Plugin POC</h3>
<p align="center">A proof-of-concept for a dynamically loaded plugin-based module system.</p>

---

> 👉 Comments and feedback are appreciated in Issues.

Here is a proof-of-concept for how we could use some form of IPC or child process
piping to create a dynamically loaded plugin system. In its current state, we
spawn an instance of a specific "plugin", but this could potentially work like
how `cargo` does; any binary starting with `starship-plugin-*` would be treated
like a plugin, and would be executed as an RPC client.

## Goals

These are the main goals of this POC:

- [x] Create a clear API for IPC between Starship and its plugins
- [ ] Compare performance between pipe and socket-based IPC
