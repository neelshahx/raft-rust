This project is an implementation of Raft (https://raft.github.io) in Rust

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




