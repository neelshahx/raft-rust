pub mod clienthandler; // read from client, write to event queue TODO: goes in raftapp
pub mod shared;
pub mod raftconsensus; // TODO: adds client command to log, updates followers, updates leader
pub mod raftconsole; // read from console, write to event queue
pub mod raftlog; // handles adds/appends to log
pub mod raftnet; // handles raft internal network
pub mod raftserver; // connects the pieces // static configuration
