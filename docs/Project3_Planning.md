# Project 3 - Starting Out

Part of the problem with implementing Raft is knowing precisely where
to begin.  In this project, we're going to focus on a few goals--some
concrete and others more abstract.

## Multiple Application Instances

First and foremost, Raft is meant to serve an application.  In Project 1,
you created a simple Key-Value database.  It runs a single server.  With 
Raft, the application is going to be replicated on different servers.

Thus, your first task is to modify the KV Application to allow multiple
instances of the application to run on different network addresses.
Mostly, this is a configuration problem.   Modify your `kvserver.py` file
so that there is an association of logical server numbers to unique 
network addresses. For example:

```
kvserver.py

KVSERVERS = {
    1 : ('localhost', 20000),
    2 : ('localhost', 21000),
    3 : ('localhost', 22000),
    4 : ('localhost', 23000),
    5 : ('localhost', 24000)
}
...
```

Then, modify the KV server so that a specific server number is 
supplied on the command line when you run it.  For example,
to run server 2, you'd type this:

```
% python3 kvserver.py 2
KV Server 2 running on ('localhost', 21000)
```

After you have done this, modify your `kvclient.py` program
so that you also have to specify a server number on the command
line.  For example, to connect to server 2, type this:

```
% python3 kvclient.py 2
KV [2] > set x 42
ok
KV [2] > get x
42
KV [2] > incr x 10
52
KV [2] >
```

Other than a bit of configuration around networking, no other changes
should be made to the KV application at this time.  It should work
exactly the same way it did in Project 1 except that up to five
servers can now run at the same time (on different network addresses)
and the client can connect to any one of the servers.

In terms of the Raft project, this application interface roughly
represents steps (1) and (4)
as shown in Figure 1 of the [Raft Paper](https://raft.github.io/raft.pdf). 
Specifically, clients must be able to connect to the application and
issue commands (step 1).  When the commands execute, the response is
communicated back to the client (step 4).

## Internal Networking

The Raft algorithm involves a leader election process as well as log
replication where the leader replicates commands to followers.  To do
this, the algorithm requires an internal networking layer where
different Raft servers can send algorithm-related messages to each
other.  This networking is **DIFFERENT** than the application/client
interaction described in the prior section.   

Internal networking is critical if you want to have an actually
working version of Raft at the end of the course.  However, most
people find the code for it to be a bit fiddly and bug-prone.  By
starting with it now, you'll have a few days to work out various
issues with it.

Start by setting up some of basic configuration concerning the Raft
network.  Make a file `raftconfig.py` and hardcode some network
settings.  Again, these are **DIFFERENT** than the Key-Value
application.

```
# raftconfig.py

# Mapping of logical server numbers to network addresses.  All internal 
# network operations in Raft will use the logical server numbers 
# (e.g., send a message from node 2 to node 4).

RAFTSERVERS = {
    1: ('localhost', 15000),
    2: ('localhost', 16000),
    3: ('localhost', 17000),
    4: ('localhost', 18000),
    5: ('localhost', 19000),
    }
```

Next, make some kind of object that encapsulates the two essential 
network operations for servers--sending and receiving a message.

```
# raftnet.py

class RaftNet:
    def __init__(self, nodenum:int):
        # My own address
        self.nodenum = nodenum       
        
    # Send a message to a specific node number
    def send(self, destination: int, message:bytes):
        ... 

    # Receive and return any message that was sent to me
    def receive(self) -> bytes:
        ...
```

Try to implement these operations using actual network connections
(via sockets).  You already did something similar for the key-value
server and the traffic light.  This won't be much different.

## Writing a Debugging Console

Debugging Raft can be quite challenging.  To assist in that, try to
create a small debugging console that allows you to interactively 
type debugging commands.  Here's a small sample in Python:

```
def console(nodenum):
    while True:
        line = input(f"Raft {nodenum} > ")
        if not line:
            continue
        cmd, args = cmd.split()
        if cmd == 'submit':
            print("Submitting command to Raft")
            ...  # TODO
        elif cmd == 'log':
            print("Viewing Raft log")
            ...  # TODO
        elif cmd == 'quit':
            ...  # TODO
        else:
            print("Unknown command")
```

You'll need to flesh out a few details (and maybe provide a bit of
error handling), but this is a starting point. As the project evolves, you can add/remove 
various commands to the console to help you test and debug
your code.

## Server Ping

To test things out, implement a "ping" command where a server sends a
message to another server on the internal network and receives a
response back.  The purpose of this command is simply to test network
connectivity between Raft servers.  Here's what the ping command might
look like in the console:

```
Raft 1 > ping 2
2 is alive
Raft 1 >
```

In order for this to work, you would need to have programs running in
two different terminal windows.  One program would be running as Raft node 1
and the other would be running as Raft node 2.

## Sketching of Raft

Your final task is to spend some time thinking about the
overall "shape" of the Raft project. Figure 1 of the [Raft
Paper](https://raft.github.io/raft.pdf) shows the major components at
a high level.    For example, Raft involves sending messages between
servers.  There is some kind of consensus module that holds the
core Raft algorithm, a transaction log, and the application itself
(the state machine).   How are these parts going to be put together?

There's not necessarily one right answer, but think about what you
might need based on your current understanding of the paper.  Sketch
out a few files, the data structures, and other details.  We're not so
focused on actually making anything work yet--this is all just
planning and thinking.  Implementation details will be filled in later
and are likely to change.




