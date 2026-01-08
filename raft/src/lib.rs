pub mod clienthandler; // read from client, write to event queue TODO: goes in raftapp
mod raftapp;
pub mod raftconsensus; // TODO: adds client command to log, updates followers, updates leader
pub mod raftconsole; // read from console, write to event queue
pub mod raftlog; // handles adds/appends to log
pub mod raftnet; // handles raft internal network
pub mod raftserver;
pub mod shared;
// connects the pieces // static configuration
