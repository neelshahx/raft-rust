# Project 4 - The Log

The most important part of Raft is the transaction log.  In fact, the
whole point of the algorithm is to replicate the transaction log!
Everything is ultimately about the log.  No log, no Raft.

In this project, you are going to implement the log as a stand-alone
function or object.  The goal is to have no dependencies on the
network, the system, or any other part of Raft.  The primary reason
for doing this is testing and understanding.  It is absolutely
essential that the log and its associated "append" operation get
implemented correctly.  If there are any bugs in it, you will be
chasing them through the 9th inner circle of debugging hell if you're
trying to figure out what's wrong when it's combined with all of the
networking and concurrency code later.

## Background Reading

The behavior of the log is described in section 5.3 and 5.4 of the
[Raft Paper](https://raft.github.io/raft.pdf). At first read, it's not
going to entirely make sense, but give it another read and proceed.

## The Project

In a nutshell, we're going to implement the two core append
operations related to Raft.   Here's a shell of the code:

```
# Each log entry consists of a term number and a command.
# A command is from the application such as 'set x 13'.
class LogEntry:
    def __init__(self, term:int, command:str):
        self.term = term
        self.command = command

class RaftLog:
    ...
    # Add a new command to the end of the log.  Performed by leaders.
    def add_new_command(self, leader_term: int, command: str):
        ...
        # Create a new LogEntry(leader_term, command) and put on
        # the end of the log.
        ...
        
    # Append new entries to the log. Used by the leader to update followers. 
    def append_entries(self, prev_index:int, prev_term:int , entries:list[LogEntry]) -> bool:
        ...
        return success     # A boolean
```

The `add_new_command()` method is used to create a new log entry and
append it to the end of the log.  This always works and is only
performed by the Raft leader. Log entries contain the leader term
number and a command.  The command is something from the application
(e.g., the KV application).

Log entries also have an associated index that indicate their log
position. However, the index may be implicit depending on
how you implement things.  For example, if you're using something
like a list or vector to store log entries, the index is simply the position
within the list (and doesn't need to be stored on the `LogEntry`
class itself).

The `append_entries()` method is used to replicate zero or more
already created log entries to a log.  It returns a True/False value
to indicate success.  This is the core operation that leaders use to
update followers.  The `prev_index` argument specifies the position in
the log *after which* the new entries go (e.g., specifying
`prev_index=8` means that entries are being appended starting at index
9). The `prev_term` argument specifies the expected `term` value of
the log entry at position `prev_index`. `entries` is a list of zero or
more `LogEntry` instances that are being added.

The `append_entries()` method may fail if certain conditions aren't
met.   This requires very careful reading of the Raft paper, but
here is a short summary of the requirements:

1. The log is never allowed to have holes in it.  For example, if
there are currently 5 entries in the log and `append_entries()` tries
to add a new entry at index 9, then the operation fails (return `False`).

2. There is a log-continuity condition where every append operation
must verify that the term number of the previous entry matches an
expected value. For example, if appending at `prev_index` 8, the
`prev_term` value must match the value of `log[prev_index].term`. If
there is a mismatch, the operation fails (return `False`).

3. Special case: Appending new log entries at the start of the log
always needs to work.  For this edge case, there is no "previous entry"
with which to obtain `log[prev_index].term`.   Sometimes people
have found it useful to insert an initial dummy value in the first
log position to avoid the empty case.

4. `append_entries()` is "idempotent."  That means that
`append_entries()` can be called repeatedly with the same arguments
and the final result is always the same.  For example, if you called
`append_entries()` twice in a row to add the same entry at index 10,
it just puts the entry at index 10 and does not result in any data
duplication or corruption.

5. Calling `append_entries()` with an empty list of entries is
allowed.  In this case, it should report `True` or `False` to indicate
if it would have been legal to add new entries at the specified
position.  Providing en empty list of new entries should never alter
the log regardless of the resulting success value.
 
6. If there are already existing entries at the specified log position,
but those entries are from an earlier term, the existing entries and
everything that follows are deleted.  The new entries are then
added in their place.  Ponder: What happens if there are existing
entries from the current term?  What happens if there are existing
entries from a later term?   Very careful reading of the paper is required.

## Testing

Of particular interest to this project is the problem of testing.
You should be able to write unit tests that test the various edge cases
of log behavior.

Figure 7 of the [Raft Paper](https://raft.github.io/raft.pdf) might
also be of interest.  This figure shows different possible log
configurations in relation to a new selected leader.  You might use
this figure as the basis for a test.  For example, you could check
what happens when the leader appends a new entry to the various logs
shown.

```
# Append an entry from term=8 at prev_index=10, prev_term=6
# Note: This assumes 1-based indexing like in the paper.
append_entries(log, 10, 6, [ LogEntry(8, "x") ])
```

The result of doing this for Figure 7 is as follows:

```
(a) False. Missing entry at index 10.
(b) False. Many missing entries.
(c) True. Entry already in position 11 is deleted.
(d) True. Entries at position 11,12 are deleted.
(e) False. Missing entries.
(f) False. Previous term mismatch.
```

## Other Methods

Later in the project, you will need to interact with Log in
other ways.  Because of this, you may want to implement a few
additional methods.  I often find the following methods to
be useful:

```python
class RaftLog:
    ...
    def last_index(self) -> int:
        # Return the index of the last log entry
        ...
        
    def last_term(self) -> int:
        # Return the term of the last log entry
        ...
        
    def get_entries(self, index: int, max_entries: int) -> list[LogEntry]:
        # Return at most max_entries log entries starting at the given index
        ...
```

## Other Details

Figure 2 of the Raft paper describes some additional details
concerning message term numbers and updates to the commit index.  In
this project, we're ignoring that and focusing solely on the problem
of appending to the log.  Those details won't matter until we start to
work on networking and replication.

## Persistence

Technically, the Raft log is supposed to be stored in a persistent
data structure that can survive server crashes.  One design problem
to think about is how persistence might get implemented and how this
implementation might be related to your stand-alone `append_entries()`
operation.   It's not necessary to solve this problem right away, but
tuck it away in the back of your mind to think about for later.

## Log Compaction

As Raft operates, the log will get longer and longer as
more entries are added.   However, the log can't grow infinitely.
Eventually, the log will have to be truncated to make space.  Section 7
of the Raft paper talks about a process of log compaction 
involving snapshots. 

We're not going to implement compaction. However, if you're forward
looking, you might think about how your Raft log implementation might
interact with log compaction.  For example, is there a lower bound on
the index of log entries?  Is it something that can be adjusted?  What
happens if access to an out-of-bounds log entry occurs?



