# Project 1 - Key-Value Server

In the [Warmup](Warmup.md) exercise, you wrote code for sending and
receiving messages over a socket.  In this project, you are going to
use that code to build a networked key-value store.  A key value store
is essentially a dictionary that lives on the network.  For example,
it would provide networked version of the following operations:

```
data = { }

def get(key:str) -> str:
    return data.get(key)

def set(key:str, value:str) -> str:
    data[key] = value
    return 'ok'

def delete(key:str) -> str:
    if key in data:
        del data[key]
    return 'ok'
```

Write a program `kvserver.py` that contains the key-value store data
and responds to client messages such as `"set key value"`, `"get key"`,
and `"delete key"`.

You'll run the server as a standalone program like this:

```
bash % python kvserver.py
KV Server listening on port 12345
```

Next, write a `kvclient.py` program that connects to the remote server and
allows commands to be typed at the terminal like this:

```
bash % python kvclient.py
KV > set foo hello
ok
KV > get foo
hello
KV > delete foo
ok
KV >
```

If you're using a programming language other than Python, try to
follow the same model.   

Don't overthink things.  Read a command on the input as a string, send
the command string to the server, and receive the server's response
string.

## Supporting Concurrency

Your server should allow multiple clients to be connected at the same
time.  To do this, use threads or a similar concurrency primitive.

## A Design Dilemma

Ben tells everyone that he intends to use the key-value server to
implement a distributed counter.  For example, implementing the
following client operation (shown in pseudocode):

```
# Increment a counter value
def increment_counter(name: str, change: int):
    value = int(server_execute(f'get {name}'))
	value += change
	server_execute(f'set {name} {value})
```

In this code, the `server_execute()` indicates a command that is
to be executed by the remote KV server.

Eva argues that the `increment_counter()` feature would be better
implemented as a feature of the KV server itself and not as an
operation carried out in clients.  For example, she proposes a server
command such as `incr name change` that both increments and returns
the new value like this:

```
KV > set x 42
ok
KV > incr x 3
45
KV > 
```

Your task:  Implement `incr` as a server feature so that
you can experiment with it.  Note: `incr` only works on 
integer values.  If applied to a non-integer, have it
return an error of some kind.

```
KV > set name Guido
ok
KV > incr name 1
error: not a number
KV >
```

## How to Proceed

To make this work, you need to send messages back and forth between
the client and the server.  These messages need to contain the method
(i.e., "get", "set", "delete", "incr") as well as additional information about
the keys and values.  In addition, the server needs to send a response
message back. You can probably build the application as an extension of
the echo server written in the warmup.

## Big Picture

This key-value client/server is eventually going to serve as the
"application" or "state machine" for Raft.  It doesn't have to be
super-fancy, but spend some time to make sure that it works correctly.
Think about a strategy for testing it.

The difference between the `set name value` and `incr name change` commands 
present some subtle problems that will be discussed at some length in later
stages of the project.

The main thing is that you have some kind of client-server application
to work with.  Raft is going to be added to this application.  The
server portion will eventually become replicated and the client portion
will be modified to contact the Raft leader. 







