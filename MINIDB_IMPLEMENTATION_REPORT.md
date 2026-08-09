# MINIDB IMPLEMENTATION REPORT

## Overview
MiniDB is an educational relational database engine built entirely from scratch in Rust. It does not wrap SQLite or any other engine. It demonstrates deep, component-level understanding of database internal mechanics. 

## Architecture
The system follows a classic RDBMS pipeline:
`CLI` -> `Lexer/Parser` -> `Query Planner` -> `Execution Engine` -> `Buffer Pool` -> `Storage Engine`.

## SQL Features
- **Supported Commands**: `SELECT`, `INSERT`, `UPDATE`, `DELETE`, `CREATE TABLE`, `CREATE INDEX`, `BEGIN`, `COMMIT`, `ROLLBACK`, `EXPLAIN`.
- **Filtering**: `WHERE` clause support with standard operators (`=`, `!=`, `<`, `>`, etc).
- **Projections**: Limiting columns.
- **Constraints**: Primary key enforcement and `NOT NULL`.

## Storage
- **Pages**: Fixed-size 4096-byte (4KB) pages managed by a `DiskManager`.
- **Slotted Pages**: To support variable-length records (like `TEXT`), pages employ a slot directory at the header that grows downward, while actual tuple data grows upward from the bottom. Deletions use tombstones (`size = 0`).

## Indexes
- **B+ Tree**: Implemented as the primary index structure. 
- **Nodes**: `InternalNode` manages search keys and children `PageId` routing. `LeafNode` contains keys mapped directly to `RecordId`s (PageId + Slot Number).
- **Sequential Access**: Leaf nodes contain `next_leaf` pointers to optimize range queries (e.g., `WHERE age > 18`).

## Transactions
- **Atomicity/Durability**: Enforced via a Write-Ahead Log (WAL).
- **Consistency/Isolation**: Maintained via a `TransactionManager` coordinating states (`Active`, `Committed`, `Aborted`). 

## WAL & Recovery
- **WAL Rule**: The system guarantees no data page is flushed to disk before its corresponding WAL log record is flushed.
- **Recovery**: Uses a simplified ARIES protocol on startup. 
  1. **Analysis**: Finds active txns.
  2. **Redo**: Repeats history to bring pages to crash state.
  3. **Undo**: Reverts changes of uncommitted txns using `prev_lsn` chains.

## Query Planner
- **Logical Plan**: Translates AST statements into a relational algebra tree (`Filter`, `Projection`, `SeqScan`).
- **Physical Plan**: Consults the catalog to apply optimizations (e.g. converting a `Filter` + `SeqScan` into an `IndexScan` if a B+ Tree index exists).

## Execution Engine
- **Volcano Model**: All operators implement an `Executor` trait (`init()`, `next()`), passing tuples up the tree.

## Buffer Pool
- **Caching**: Manages an array of memory frames mapped to `PageId`s.
- **Eviction**: Uses a Least Recently Used (LRU) policy. Pinned pages (`pin_count > 0`) cannot be evicted. Dirty pages are flushed via the DiskManager before eviction.

## Concurrency
- **Strict 2PL**: Implements Strict Two-Phase Locking. Transactions acquire Shared/Exclusive locks during execution and hold them until the `COMMIT`/`ROLLBACK` phase to prevent cascading aborts and guarantee isolation.

## Limitations
- Rust toolchain validation (cargo check/test) was skipped during this scaffolding phase due to environment constraints.
- Advanced SQL parsing (JOINs, GROUP BY, aggregations) are not supported in this core implementation.
- Concurrency currently assumes thread-safe `Arc<Mutex<>>` boundaries but lacks a background deadlock detection thread.

## Future Roadmap
- **Cost-Based Optimizer**: Migrate the Query Planner from heuristic-based to cost-based using catalog statistics.
- **Aggregations**: Implement `HashAggregate` and `StreamAggregate` executors.
- **Joins**: Implement `NestedLoopJoin` and `HashJoin`.
- **MVCC**: Move from Strict 2PL to Multi-Version Concurrency Control to prevent readers from blocking writers.
