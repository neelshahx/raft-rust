# Introduction to Messages and Concurrency

To implement Raft, you minimally need to be able to write programs
that exchange network messages.  In addition, you will need to write
programs that involve concurrency (i.e., being able to do more than
one thing at once).  This exercise guides you through a few primitive
elements that may be useful in the project.

## Part 1:  Network Programming with Sockets

The most low-level way to communiate on the network is to write code
that uses sockets.  A socket represents an end-point for communicating
on the network.   

To create a socket, use the `socket` library.  For example:

```
from socket import socket, AF_INET, SOCK_STREAM
sock = socket(AF_INET, SOCK_STREAM)
```

The `AF_INET` option specifies that you are using the internet
(version 4) and the `SOCK_STREAM` specifies that you want a reliable
streaming connection.  Technically, this is a TCP/IPv4 connection.
Change the `AF_INET` to `AF_INET6` if you want to use TCP/IPv6.

### Making an Outgoing Connection

If a program makes an outgoing connection to a remote machine, it is
usually known as a "client." Here's an example of using a socket to
make an outgoing HTTP request to a web server and reading the response:

```
# client.py
from socket import socket, AF_INET, SOCK_STREAM

sock = socket(AF_INET, SOCK_STREAM)
sock.connect(('www.python.org', 80))
sock.send(b'GET /index.html HTTP/1.0\r\n\r\n')
parts = []
while True:
    part = sock.recv(1000)     # Receive up to 1000 bytes (might be less)
    if part == b'':
        break                  # Connection closed
    parts.append(part)
# Form the complete response
response = b''.join(parts)
print("Response:", response.decode('ascii'))
```

Try running the above program.  You'll probably get a response
indicating some kind of error.  That is fine. Our goal is not to
implement HTTP, but simply to see some communication in action.

Now, a few important details.

1. Network addresses are specified as a tuple `(hostname, port)` where
`hostname` is a name like `'www.python.org'` `port` is a number in the range 0-65535.

2. The port number must be known in advance.  This is usually dictated
by a standard. For example, port 25 is used for email, port 80 is used
for HTTP, and port 443 is used for HTTPS.  See
https://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.xhtml

3. Data is sent using `sock.send()`.  Data is received `sock.recv()`.
Both of these operations only work with byte-strings.  If you are
working with text (Unicode), you will need to make sure it's properly
encoded/decoded from bytes.

4. Data is received and transmitted in fragments.  The `sock.recv()`
accepts a maximum number of bytes, but that it is only a maximum.  The
actual number of bytes returned might be much less than this. It is your
responsibility to reassemble data from fragments into a complete
response.  Thus, you might have to collect parts and put them back
together as shown.  A closed connection or "end of file" is indicated
by `sock.recv()` returning an empty byte string.

Try the above code in Python.  Then, try to convert it to your favorite
programming language.  Although the exact details might vary, all 
programming languages provide some mechanism for accessing the low-level
network.

### Receiving Incoming Connections

A program that receives incoming connections is usually known as a
"server."  Recall that clients (above) need to know the address and
port number in order to make a connection.  To receive a connection, a
program first needs to bind a socket to a port.  Here's how you do
that:

```
sock = socket(AF_INET, SOCK_STREAM)
sock.bind(('', 12345))         # Bind to port 12345 on this machine
sock.listen()                  # Enable incoming connections
```

To accept a connection, use the `sock.accept()` method:

```
client, addr = sock.accept()     # Wait for a connection
```

`accept()` returns two values.  The first is a new socket
object that represents the connection back to the client.  The second
is the remote address `(host, port)` of the client.  You use the
`client` socket for further communication.  Use the address `addr` for
diagnostics and to do things like reject connections from unknown
locations.

One confusion with servers concerns the initial socket that you create
(`sock`).  The initial socket only serves as a connection point
for clients. No actual communication takes place using this socket.
All further communication actually uses the `client` socket
returned by `sock.accept()`.

Here is an example of a server that reads a short message sent by the client and
echoes it back.

```
# echoserver.py
import time
from socket import socket, AF_INET, SOCK_STREAM

sock = socket(AF_INET, SOCK_STREAM)
sock.bind(('',12345))
sock.listen()
while True:
    client, addr = sock.accept()
    print('Connection from', addr)
	msg = client.recv(1000)
    client.sendall(msg)
    client.close()
```

Try running this program on your machine.  While it is running, try
connecting to it from a separate program.

```
# echoclient.py
from socket import socket, AF_INET, SOCK_STREAM

sock = socket(AF_INET, SOCK_STREAM)
sock.connect(('localhost', 12345))
sock.sendall(b'Hello World')
response = sock.recv(1000)      # Get the response
print('Server said:', response.decode('utf-8'))
```

If using a language other than Python, port the above programs
to your language.  As an experiment, you can mix the above
Python programs with your programs. For example, you can have
a server written in Rust receive connections from a client
written in Python. 

## Part 2: From Sockets to Messages

A problem with sockets is that they are too low-level.  Data is often
streamed in fragments in a manner that's hard to predict. As such,
the code is often buggy.  In the above code, consider the `sock.recv(1000)`
operation.  The 1000 is a maximum byte code.  In reality, the `recv()` 
call might return less data than that.   However, as a maximum, this
also limits the maximum message size.  You could consider increasing
the amount to some very large number, but this also won't work
(as an experiment, try increasing the number to 10000000 and then sending
a very large message in `echoclient.py`).

To make communication over a socket more sane, it is common
to build a message-passing abstraction layer where messages are
packaged into discreet units that are delivered and received in their
entirety.  One way to do this is to use size-prefixed messages.  This
is a technique where every message is prepended with a byte-count to
indicate how large the message is.  Here is some sample code that
sends a size-prefixed message:

```
def send_message(sock, msg):
    size = b'%10d' % len(msg)    # Make a 10-byte length field
    sock.sendall(size)           # Send the size prefix
    sock.sendall(msg)            # Send the message payload
```

Here is some sample code that receives a size-prefixed message:

```
# Receive exactly nbytes of data, using multiple recv()
# operations if necessary.
def recv_exactly(sock, nbytes):
    chunks = []
    while nbytes > 0:
        chunk = sock.recv(nbytes)
        if chunk == b'':
            raise IOError("Incomplete message")
        chunks.append(chunk)
        nbytes -= len(chunk)
    return b''.join(chunks)

def recv_message(sock):
    size = int(recv_exactly(sock, 10))    # The size prefix
    return recv_exactly(sock, size)       # Message payload
```

Put these functions into a file called `message.py`.

### Echo Server

To test your `send_message()` and `recv_message()` functions, modify your
echo server and client.  For example, here's a client:

```
# echoclient.py
from socket import socket, AF_INET, SOCK_STREAM
from message import send_message, recv_message

def main(addr):
    sock = socket(AF_INET, SOCK_STREAM)
    sock.connect(addr)
    while True:
        msg = input("Say > ")
        if not msg:
            break
        send_message(sock, msg.encode('utf-8'))       
        response = recv_message(sock)
        print("Received > ", response.decode('utf-8'))
    sock.close()
     
main(('localhost', 12345))
```

Here's the modified server

```
# echoserver.py
from socket import socket, AF_INET, SOCK_STREAM, SOL_SOCKET, SO_REUSEADDR
from message import send_message, recv_message

def echo_messages(sock):
    try:
        while True:
            msg = recv_message(sock)
            send_message(sock, msg)
    except IOError:
        sock.close()
        
def main(addr):
    sock = socket(AF_INET, SOCK_STREAM)
	sock.setsockopt(SOL_SOCKET, SO_REUSEADDR)
    sock.bind(addr)
    sock.listen()
    while True:
        client, addr = sock.accept()
        print('Connection from:', addr)
        echo_messages(client)

main(('localhost', 12345))
```

Make sure you know how to run the above two programs.  You need to run
the `echoserver.py` program first and leave it running.  The
`echoclient.py` programs needs to run separately.  You should be able
to type messages into the client and see them echoed back.

The addition of the `sock.setsockopt()` call relates to a common
error encountered when writing network code.  If a server crashes
and needs to restart, you will sometimes get a lingering "address
already in use" error.   This error will go away by itself after about two
minutes, but the `sock.setsockopt()` call in the code makes it go
away right away.

Again, port the above code to the language you intend to use
for the Raft project.

## Part 3 - Concurrency with Threads

Run the `echoserver.py` and `echoclient.py` programs from the last part.
While they are running, start a second `echoclient.py` program in a
separate terminal window.   Does this second program work?  That is, if
you type messages, are they echoed back?   The answer should be no.

What happens if you kill the first `echoclient.py` program?  (Hint:
you should sending see the second program start working).

A common problem in network programming is that of handling concurrent
connections.  One way to to address this is to use thread programming.
A thread a function that runs concurrently and independently within
your program.  Here is a simple Python example:

```
import time
import threading

def countdown(n):
    while n > 0:
        print('T-minus', n)
        time.sleep(1)
        n -= 1

def countup(stop):
    x = 0
    while x < stop:
        print('Up we go', x)
        time.sleep(1)
        x += 1

def main():
    t1 = threading.Thread(target=countdown, args=[10])
    t2 = threading.Thread(target=countup, args=[5])
    t1.start()
    t2.start()
    print('Waiting')
    t1.join()
    t2.join()
    print('Goodbye')

main()
```

Run this program and watch what it does.  You should see the `countdown()`
and `countup()` functions running at the same time.

There's not much you can do with threads once created.  The `join()`
method is used if you want to wait for a thread to terminate.  There is
no way to kill a thread manually.

Try porting the above thread code to the language you intend to use
for the project.   It should be fairly straightforward.

### A Threaded Echo Server

Modify the `echoserver.py` program to handle each client connection
in a thread.  This is a minor change:

```
# echoserver.py
from socket import socket, AF_INET, SOCK_STREAM, SOL_SOCKET, SO_REUSEADDR
from message import send_message, recv_message
from threading import Thread

def echo_messages(sock):
    try:
        while True:
            msg = recv_message(sock)
            send_message(sock, msg)
    except IOError:
        sock.close()
        
def main(addr):
    sock = socket(AF_INET, SOCK_STREAM)
	sock.setsockopt(SOL_SOCKET, SO_REUSEADDR)
    sock.bind(addr)
    sock.listen()
    while True:
        client, addr = sock.accept()
        print('Connection from:', addr)
        Thread(target=echo_messages, args=[client]).start()  # <<<

main(('localhost', 12345))
```

Run this new version of `echoserver.py`.  Make sure that it can talk
to multiple clients at once.

Again, if using a language other than Python for the project,
figure out how to launch threads or concurrent tasks and modify
your code accordingly.

## Part 4 - Concurrency Recipes

Once concurrency enters the picture, you're likely to face a
variety of problems related to thread coordination.   In this
section, we briefly look at a number of common thread programming
techniques that may be useful in the project.  Although these
are given in Python, the techniques are pretty standard and you
are likely to find similarity functionality in other languages.

### Communicating Tasks

Sometimes, it's useful for threads to communicate with each other.
One approach for this problem is to create a channel or queue.
Here is an example of creating a producer-consumer queue in
Python.  This would be similar to a channel in Go or a multi-producer,
single consumer (MPSC) channel in Rust.

```
import threading
import queue
import time

def producer(q):
    for i in range(10):
        print('Producing', i)
        q.put(i)
        time.sleep(1)
    q.put(None)
    print('Producer done')

def consumer(q):
    while True:
        item = q.get()
        if item is None:
            break
        print('Consuming', item)
    print('Consumer goodbye')

def main():
    q = queue.Queue()
    t1 = threading.Thread(target=producer, args=[q])
    t2 = threading.Thread(target=consumer, args=[q])
    t1.start()
    t2.start()
    print('Waiting')
    t1.join()
    t2.join()
    print('Done')

main()
```

In this program, there are two different threads, `producer()` and
`consumer()`.   The producer puts items onto a queue which the
consumer reads them.   Try running this program and observe its
behavior.  Make sure you understand what's happening.

Again, try porting this example to the programming language you
intend to use for the project.

### Making a Thread Wait for an Event

Sometimes you want a thread to pause its execution until it's signalled
to wake up.  One way to do this is to use an event.  Try this code:

```
import threading
import time

def waiter(evt):
    print("Yawn. I'm waiting")
    evt.wait()
    print("I'm awake")

def main():
    evt = threading.Event()
    # Launch many threads
    for i in range(5):
        threading.Thread(target=waiter, args=[evt]).start()
    time.sleep(10)
    evt.set()       # Make the thread wake up

main()
```

When you run this code, you should see 5 threads print a message
about waiting and then go to sleep.  After 10 seconds, when the event is
set, they'll all wake up.

### Protecting Mutable Data

Great care needs to be taken with concurrent access to mutable data.
Try the following example:

```
import threading
import time

data = {
    'x' : 1,
    'y' : 2,
    'z' : 3
}

def process_data():
    for key, value in data.items():
        print(f'{key}={value}')
        time.sleep(1)

def main():
    threading.Thread(target=process_data).start()
    time.sleep(1)
    data['a'] = 100
    print(data)

main()
```

To fix this, you may need to protect mutable data with a lock.  Try this
updated version.

```
import threading
import time

data_lock = threading.Lock()
data = {
    'x' : 1,
    'y' : 2,
    'z' : 3
}

def process_data():
    data_lock.acquire()
    for key, value in data.items():
        print(f'{key}={value}')
        time.sleep(1)
    data_lock.release()

def main():
    threading.Thread(target=process_data).start()
    time.sleep(1)
    data_lock.acquire()
    data['a'] = 100
    data_lock.release()
    print(data)

main()
```

A lock guarantees that at most, only one thread can execute code
between the `lock.acquire()` `lock.release()` operations.  Locks
should always be used if multiple threads are accessing and
modifying shared state.

## Summary

The general programming techniques used in this warmup are the main
components you will need to implement Raft.  Specifically:

1. You need to send network messages.
2. You need to have concurrent tasks (threads, etc.)
3. You need to manage communication between tasks (queues, channels, etc.).
4. You may need to coordinate tasks in other ways (events and locks).

If you are planning to implement the project in a different language
than Python, it is critically important that you try to work these
examples in that language.  Doing so *in advance* will make the
project more manageable and it will save you a lot of time early on.








 
