# Project 9 - Timing/Systems

At this point, you should be fairly far along in your Raft
implementation.  The goal is to work on loose-ends and other systems
implementation details.  This could include any of the following:

* Timers (heartbeats, elections)
* Networking with sockets
* Threads/concurrency
* Logging/monitoring

## Suggestions on the KV Application

If you are working to add Raft to your KV application, you will need to consider a
number of issues.  First, the KV application needs to be replicated just like Raft. It
need its own set of network ports:

```
KVSERVERS = {
    1: ('localhost', 15050),
    2: ('localhost', 16050),
    3: ('localhost', 17050),
    4: ('localhost', 18050),
    5: ('localhost', 19050),
    }
```

These ports are used by clients of the KV Application.  They are different
from the ports that the internal Raft algorithm uses.

Second, each server of the KV application will have an internal
structure roughly like this (see Figure 1 in the paper):

```
class KVServer:
    def __init__(self, nodenum):
        self.nodenum = nodenum
        self.app = KVApplication()
        self.raft = Raft(nodenum, self.apply_command)

    def apply_command(self, command):
        # Called by Raft to apply commands that have consensus.
        # Note: This is executed by ALL servers including leaders and followers
        ...

    def handle_client(self, sock):
        # Read commands from the client
        while True:
            command = sock.recv_message()
            # Submit to Raft
            self.raft.submit(command)
            # Anything else ???
            ...
```

Finally, you need to write a KV client program that allows you to connect to
any of the nodes.  For example:

```
shell % python3 raftkvclient.py 1
KV 1 > set x 23
ok
KV 1 > get x
23
KV 1 >
```

In this example, 1 is the node number.   You can change this number to
connect to the current leader.   If you connect to a non-leader, commands
should be rejected.   Everything that happens in Raft is supposed to happen
through the leader.

If you're feeling ambitious, you could modify your KV client to automatically
discover the leader and submit command to it.     However, for the purposes
of this project (and remaining time), it's fine to just make the server
number explicit as shown.

