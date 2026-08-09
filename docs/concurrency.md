# Concurrency Subsystem

MiniDB supports concurrent transactions to allow multiple queries to run simultaneously without corrupting data or seeing inconsistent states.

## Strict Two-Phase Locking (2PL)
To maintain **Isolation**, MiniDB uses Strict Two-Phase Locking.

1. **Growing Phase**: A transaction acquires locks (Shared or Exclusive) as it needs them, but never releases any.
2. **Shrinking Phase**: A transaction releases all its locks simultaneously at the very end of the transaction (upon `COMMIT` or `ROLLBACK`). 

By holding all locks until the end, we prevent cascading aborts and ensure rigorous serializability.

## Lock Modes
- **Shared Lock (S)**: Required for reading a record. Multiple transactions can hold a Shared lock on the same record simultaneously (Multiple Readers).
- **Exclusive Lock (X)**: Required for writing/updating/deleting a record. Only one transaction can hold an Exclusive lock, and no Shared locks can exist while an Exclusive lock is held (Single Writer).

## The LockManager
The `LockManager` maintains an internal, thread-safe hash map (`Mutex<HashMap<RecordId, Vec<LockRequest>>>`) tracking which transaction holds which lock.
When a transaction attempts to acquire a lock that conflicts with an existing one, it must block (wait) until the lock is released.

## Deadlocks
(Future implementation notes)
Because transactions can wait on each other, deadlocks are possible (e.g., Txn A waits for Txn B, while Txn B waits for Txn A).
To handle this, MiniDB would need a **Deadlock Detection** background thread that periodically builds a waits-for graph, detects cycles, and aborts the youngest transaction.
