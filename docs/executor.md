# Executor Subsystem

The Execution engine in MiniDB follows an iterator model, often referred to as the **Volcano model** or **Pipeline model**.

## The `Executor` Trait
All execution operators implement the `Executor` trait, which defines a simple interface:
```rust
pub trait Executor {
    fn init(&mut self) -> Result<(), MiniDbError>;
    fn next(&mut self) -> Result<Option<Vec<Value>>, MiniDbError>;
}
```

- `init()`: Prepares the executor, resets internal state, or fetches the first page ID.
- `next()`: Returns the next tuple (row) that satisfies the operator's condition. Returns `None` when there are no more tuples.

## Composition
Because all operators implement the same trait, they can be composed hierarchically to form an execution plan.

For example, a query like `SELECT * FROM users WHERE age > 18 LIMIT 10` can be mapped to:
```text
LimitExecutor
 └── FilterExecutor
      └── SeqScanExecutor
```

When `LimitExecutor.next()` is called, it calls `FilterExecutor.next()`, which in turn calls `SeqScanExecutor.next()`. 

## Supported Executors
- **SeqScan**: Linearly scans all pages of a table using the Buffer Pool.
- **Insert**: Takes raw values (or a child executor) and writes them to the table's slotted pages.
- **Update**: Fetches tuples from a child executor, modifies them, and writes the updated tuple back.
- **Delete**: Fetches tuples from a child executor and marks them as deleted (tombstone) in the slotted page.
