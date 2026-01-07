pub mod raftserver; // connects the pieces
pub mod raftconsole; // read from console, write to event queue
pub mod clienthandler; // read from client, write to event queue
pub mod raftnet; // handles raft internal network
pub mod raftlog; // handles adds/appends to log
pub mod raftconsensus; // TODO: adds client command to log, updates followers, updates leader
pub mod config; // static configuration

