# Project 7 - Raft Leader Election

In this project, we work to implement the Raft leader election and
logic for servers switching between follower/candidate/leader roles.

## Role Transitions

Raft servers run in different roles such as LEADER, FOLLOWER, and CANDIDATE.
You'll need to extend your code to include this information.

```
class Raft:
    def __init__(self):
        ...
        self.role = 'FOLLOWER'
```

Servers often change roles.  Write functions or methods that handle role changes.

```
def become_leader(self):
    self.role = 'LEADER'
    ...
    # Additional logic
    ...

def become_candidate(self):
    self.role = 'CANDIDATE'
    ...
    # Additional logic
    ...

def become_follower(self):
    self.role = 'FOLLOWER'
    ...
    # Additional logic
    ...
```

These role transitions might carry out actions and send messages. Roughly speaking,
the following actions need to take place:

1. A server that becomes a "follower" becomes passive. It 
sets its role to follower and initiates no actions at all.

2. A server that becomes a "candidate" increments its term number
and sends out a RequestVote message to all of its peers. Later
incoming RequestVoteResponse messages will let it know if it
won the election or not.  If it wins, it becomes a leader.

3. A server that becomes a "leader" resets some internal tracking
information related to the followers and then immediately sends
an AppendEntries message to all of the peers to assert its leadership.

## Elections

To implement leader election, servers need to handle two messages:

1. RequestVote.  A message sent by a candidate to all of the other
   servers.  This message will contain information about the candidate's
   log (length and last term number).

2. RequestVoteResponse.  A message sent back to a candidate in response
   to a RequestVote message.  Indicates whether or not the vote
   was granted.

In addition to these messages, there must be some mechanism for
calling an election.   

## Testing

Testing continues to be a challenge.  You should be able to write isolated
tests that exercise the different messages and events.   However, to
test it in full, you will need to simulate the effect of carrying out
an election on an entire cluster of servers.

A server may or may not be able to become a leader depending
on the state of its log.   You might try to create testing
scenarios based on figures in the paper.   For example, if working
off of Figures 6 and 7, you could write tests that make
different servers a candidate and then assert whether or not they
win the resulting election.

## Edge Cases

Leader election involves a number of subtle edge cases you'll
need to be aware of.

1. Time is subdivided into "terms" which are represented by
monotonically increasing integers.  There is only one leader per term.

2. A server may only vote for a single candidate in a given term.

3. A server only grants a vote to a candidate if the candidate's log
is at least as up-to-date as itself. Read section 5.4.1 carefully.
Then read it again.

4. A newly elected leader may NEVER commit entries from a previous
entry before it has committed new entries from its own term.  See
Figure 8 and section 5.4.2.

5. All messages in Raft embed the current term number of the sender.
If a message with a newer term is received, the receiver immediately
updates its own term and becomes a follower.  If a message with an
older term is received, a server can ignore the message (or respond
with a fail/false status response).  A server must never carry out
an operation from a earlier term.

6. A server that is a candidate should switch to a follower
if it ever receives an AppendEntries message from a server with the same
term number as its own.  This can happen if two servers decided to
become a candidate at the same time, but only one of the servers was
successful in getting a majority of votes.

7. Whenever a server grants a vote, it should immediately
reset its election timeout just as if it has heard from a leader.
This is extremely subtle, but described in Figure 2 of the Raft paper
under the "Followers" section of "Rules for Servers."  If you
forget to do this, you might find your implementation to have a
lot of "churn" over establishing a new leader.

8. A newly elected leader should immediately send `AppendEntries`
messages to its followers to assert its leadership.  This
can be an empty `AppendEntries` although the paper also suggests
the idea of putting a "dummy" entry on the log when a leader starts.




