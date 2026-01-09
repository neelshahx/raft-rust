pub mod raftserver; // imperative shell of {consensus, kvstore, net}
pub mod kvapp; // state machine, validates and applies commands
pub mod kvstore; // data structure
pub mod raftconsensus; // adds client command to log, updates followers, updates leader
pub mod raftlog; // handles adds/appends to log
pub mod clientnet; // handles client app network
pub mod raftnet; // handles raft internal network
pub mod raftconsole; // read from console, write to event queue
pub mod shared;
// connects the pieces // static configuration
