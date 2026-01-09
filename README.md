This project is an implementation of Raft (https://raft.github.io) in Rust.

### Status

Implemented, but not robust 
- replicating the log between KV servers
- applying commits to the state machine, a key-value (KV) store
- communication between clients, raft nodes and KV app

Partially implemented
- leader elections

Not implemented
- responding to the client
- log persistence and compaction
- ...

To compile

```cargo
cargo build
cargo test
```

Example: to run a 3 node raft cluster with 1 client, execute these in different shells
```cargo
cargo run --bin server 1 3
cargo run --bin server 2 3
cargo run --bin server 3 3
cargo run --bin client 1
```
Each command will launch a console
- Server console takes commands like log, state, leader, follower, etc.
- Client console takes commands such as "set k v", "get k", "delete k", and "incr k" which incr(ement)s integral values.
