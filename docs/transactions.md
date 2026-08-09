# Transaction Subsystem

The Transaction subsystem ensures that MiniDB maintains the ACID properties (Atomicity, Consistency, Isolation, Durability) for database operations.

## Transaction States
A transaction goes through the following states:
1. **Active**: The initial state when `BEGIN` is called.
2. **Committed**: When `COMMIT` is called, changes are made durable and locks are released.
3. **Aborted (Rolled Back)**: If `ROLLBACK` is called or an error occurs, all changes made by the transaction are undone.

## Transaction Manager
The `TransactionManager` issues unique Transaction IDs (using an `AtomicU32`) and handles the lifecycle of the `Transaction` object.

When `commit()` is invoked, it is responsible for ensuring the Write-Ahead Log (WAL) has been fully flushed to disk (ensuring Durability).

When `rollback()` is invoked, it uses a reverse log of operations (often tracked in an in-memory `write_set` for the transaction) to undo changes made to the buffer pool pages.

## Integration with Executors
Executors interact with the Transaction subsystem to acquire locks (Isolation) and log their changes before writing to the actual data pages (Atomicity and Durability). If a transaction fails mid-execution, the Executor yields a `TransactionError`, and the upper layers trigger a rollback.
