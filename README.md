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


### Techniques

- multiple producer single queue (mpsc) and event routing
- interrupts and timers
- networking (TCP listeners and streams, connection caching, retries)
- serialisation/deserialisation
- concurrency (threads, reference counters, atomics, mutexes)
- designing functional core/imperative shell
- testing distributed systems

### How to use
To compile
```cargo
cargo build
cargo test
```

To run a 3 node raft cluster with 1 client, execute these in different shells
```cargo
cargo run --bin server 1 3
cargo run --bin server 2 3
cargo run --bin server 3 3
cargo run --bin client 1
```
Within the console
- Servers take commands such as "log", "state", "leader", "flood", etc.
- Clients take commands such as "set k v", "get k", "delete k", and "incr(ement) k".
