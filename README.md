# ThorKV

ThorKV in an in-memory key-value database

## Design Decision

### Storage

TODO

### Concurrency: 2PL + Deadlock Detection

Since ThorKV stores every records in memory, storage space is an issue and 
keeping track of multiple versions of the record can be prohibitively 
expensive. Thus, we make a conscious decision to use 2PL + Deadlock Detection
to handle concurrency.

Not sure if we even actually need this since we may treat a transaction as a 
single key-value set/get.

### WAL

Write-ahead log is written asynchronously but it uses Raft consensus algorithm
to ensure that the log entries not flushed to disk survives a crash.

### Snapshotting & Recovery

We use CALC algorithm for snapshotting that keeps track of live and stable 
version of each record.

For the description of the algorithm, please read this paper.
Low-Overhead Asynchronous Checkpointing in Main-Memory Database Systems
Kun Ren, Thaddeus Diamond, Daniel J. Abadi, Alexander Thomson
https://15721.courses.cs.cmu.edu/spring2020/papers/10-recovery/p1539-ren.pdf
