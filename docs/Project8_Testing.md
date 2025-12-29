# Project 8 - Testing and Debugging

Testing remains one of the greatest challengs of the Raft project.  Although
it is possible to write isolated unit tests for many parts of the system,
it is hard to write tests for more system-wide effects.

Figure 8 of the [Raft Paper](https://raft.github.io/raft.pdf) is a
good example of something subtle that can go wrong.  In a nutshell,
Figure 8 is about a particular aspect of leader election.
Specifically, after a new leader is elected in Raft, is it allowed to
commit log entries from a prior leader if they happen to be on the
log?  The answer is NO unless the new leader also has committed an
entry from its own term.

The requirement of first committing an entry from its own term is an
extremely subtle aspect of the Raft algorithm. Suppose that you didn't
implement that.  How hard would it be to discover that your Raft
implementation has a problem?  Would you be able to detect this
problem in some kind of test?

Consider this project to be a challenge project consisting of two parts.

## Part 1 : Invariants

Figure 8 describes a scenario in which committed log entries are lost!
How would modify your Raft implementation to include a run-time check
or assertion that detects this problem if occurred?  Specifically, an
assertion that causes your Raft code to crash with an error if it ever
loses a committed log entry?

What other invariants are described in the Raft paper?  What can be
"easily" checked in your code without having to refactor the entire
implementation?

## Part 2 : Testing

Suppose you know nothing about potential bugs in your implementation
of the algorithm--in fact, you think that it's correct. Can you devise
some kind of general test or simulation that causes the assertion from
Part 1 to fail.  Moreover, can you produce the exact sequence of
events that need to take place to make it happen?

Just to be clear, I'm not asking you to write a test that's hard-wired
to Figure 8. Instead, I want to know if it's possible to write a test
that would discover the scenario in Figure 8 all by itself with no
outside guidance.  For example, if you didn't know that the problem in
Figure 8 existed in the first place.

## Comments

This is an open-ended project.  If you are busy with other parts of
the problem, you can skip it. Also, be aware that part 2 is
extraordinarily difficult and open-ended.  I've only been successful
in writing such a test once--and it required both writing Raft and the
test in a very specific non-obvious way after spending more than 3
years of fiddling around with it in past courses.






