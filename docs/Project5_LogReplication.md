# Project 5 - Log Replication

In Project 4, you implemented the basic `append_entries()` operation that's
at the core of Raft. In this project, you're going to extend this work to
implement log replication.   

Warning: This part of the project should not involve a huge amount of
code, but there are a lot of moving parts which make testing
and debugging more difficult. Take it slow.

## The Scenario

The ultimate goal for this project is to make one server (designated
in advance as the "leader") replicate its log to another server (a
follower).  It will do this by sending messages and processing the
replies.  You will be able to append new log entries onto the leader
log and those entries will appear on the follower. The leader will
also be able to bring the follower up to date if its log is missing
many entries.

## Server Logic

Each Raft server maintains its own copy of the log.  

```
class Raft:
    def __init__(self, nodenum:int):
        self.nodenum = nodenum
        self.log = RaftLog()
        self.current_term = 1
        ...
        # More attributes added as needed
        ...
```

Each server maintains its own copy of this state.

## Thinking in Messages

Log replication is driven by two separate network messages:

1. AppendEntries. A message sent by the Raft leader to a follower.
   This message contains log entries that should be added to the
   follower log.  When received by a follower, it uses
   `append_entries()` to carry out the operation and responds with an
   AppendEntriesResponse message to indicate success or failure.

2. AppendEntriesResponse. A message sent by a follower back to the
   Raft leader to indicate success/failure of an earlier AppendEntries
   message.  A failure tells the leader to retry the AppendEntries
   with earlier log entries.

Part of the server logic must be programmed to respond to these
messages when they are received.   Thus, you might have a dedicated
function or method dedicated to that purpose:

```
class Raft:
    ...
    def handle_append_entries(self, msg: AppendEntries):
        ...

    def handle_append_entries_response(self, msg: AppendEntriesResponse):
        ...
```

However, these methods are purely passive--containing statements for
processing messages when they arrive.   You're also going to need some way to
initiate the messages and have servers do something interesting.  For example,
how do new commands get put on the Raft log?  Also, what is actually responsible for
sending a follower an `AppendEntries` message?   When starting out, you
might make some methods specifically for these tasks:

```
class Raft:
    ...
    def add_new_command(self, command: str):
        ...
        # Add a new LogEntry to the end of the log.  Only on leaders.
        ...
    
    def update_follower(self, follower: int):
        ...
        # Send the specified follower an AppendEntries message. Only on leaders.
        ...
```

## Testing

To test your log replication, you might be able to write isolated unit
tests that test certain functionality within a carefully controlled
environment.   However, this is probably going to feel incomplete.  At
some point, you'll need to test everything in a more integrated environment.
This is a very rough sketch, but eventually you should be able to bring up
two servers and see data replicate from one server to the other:

```
>>> server1 = RaftServer(1)
>>> server2 = RaftServer(2)
>>> server1.add_new_command('set x 42')
>>> server1.log
RaftLog<[LogEntry(1, 'set x 42')]>
>>> server2.log
RaftLog<[]>
>>> server1.update_follower(2)
>>> server2.log
RaftLog<[LogEntry(1, 'set x 42')]>
>>>
```

Again, this is very rough--additional details will need to be fleshed out.

Later on, you might try to devise some system-level tests around
Figures 6 and 7 in the Raft paper.  For example, you could set up a
cluster of nodes in the configuration of Figure 6, manually designate
Server 1 as the leader, and have it update all of the other servers.
If it's working, the leader will eventually make the logs of all
followers match itself.

## Comments

Getting log replication to work might be one of the most difficult
parts of the entire Raft project.  It's not necessarily a lot of code,
but it integrates everything that you've been working on so far.
Testing and debugging is extremely challenging because you've suddenly
got multiple servers and it's hard to wrap your brain around
everything that's happening.

You will likely feel that you are at some kind of impasse where
everything is broken or hacked together in some horrible way that
should just be thrown out and rewritten.  This is normal.  Expect that
certain parts might need to be refactored or improved later.

## Debugging

Debugging this part of the project can be extremely challenging--especially
when you try to make it work with real networking.  In Project 3, it
was suggested that you write a Raft networking console where you could send
messages.   You might consider taking that code and extending it into
general purpose "Raft" console.  Have it understand some basic commands
that you could type interactively.  Here are some suggested commands:

```
log               # Show the state of the Raft log on this server
state             # Show other raft state on this server
command cmd       # Add a new command on the Raft leader
update            # Manually update followers (the heartbeat)
leader            # Become a leader
follower          # Become a follower
```

If you are using a language other than Python, having an interactive
console may extremely helpful to project success.  It doesn't have to
be fancy.



 

