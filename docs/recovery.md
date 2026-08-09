# WAL & Recovery Subsystem

To guarantee Durability and Atomicity in the event of a system crash or power failure, MiniDB employs a **Write-Ahead Log (WAL)**.

## The WAL Rule
The fundamental rule of WAL is:
> A modified data page cannot be written to disk until the log records describing those modifications have been flushed to disk.

This ensures that if the system crashes after writing the log but before writing the data page, the changes can be redone. If the system crashes after writing a data page for an uncommitted transaction, the log can be used to undo it.

## Log Sequence Numbers (LSN)
Every log record is assigned a monotonically increasing `LSN`. 
Data pages also store the `LSN` of the last operation that modified them (in the page header). This allows the recovery process to know if a specific log record has already been applied to a page or not.

## Log Records
The `LogRecord` structure contains:
- `lsn`: Unique identifier.
- `txn_id`: The transaction that made the change.
- `prev_lsn`: A linked list pointer to the transaction's previous log record (vital for fast rollbacks).
- `record_type`: What happened (`Insert`, `Update`, `Delete`, `Commit`, `Abort`). Including the raw before/after bytes for Undo/Redo logic.

## Recovery Protocol (ARIES Simplified)
On database startup, if MiniDB detects a previous unclean shutdown, it invokes `LogManager::recover()`:
1. **Analysis Pass**: Reads the log to determine which transactions were active at the time of the crash.
2. **Redo Pass**: Repeats history by applying all log records (including those of aborted transactions) to restore the database to the exact state at the moment of the crash.
3. **Undo Pass**: Scans backwards using the `prev_lsn` pointers to undo the operations of any transaction that did not commit before the crash.
