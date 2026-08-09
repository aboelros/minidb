# Buffer Pool Subsystem

Disk I/O is the slowest part of a database. MiniDB uses a **Buffer Pool** to cache active pages in memory and minimize expensive reads and writes to the physical disk.

## Architecture

The Buffer Pool sits directly between the Execution engine and the Storage engine (`DiskManager`).

```text
Executor -> BufferPoolManager -> DiskManager -> OS File
```

When an Executor wants to read `Page 4`, it asks the BufferPoolManager via `fetch_page(4)`. 
1. If `Page 4` is already in memory, it is returned immediately (a **cache hit**).
2. If `Page 4` is not in memory (a **cache miss**), the BufferPoolManager evicts an old page to make room, reads `Page 4` from disk, places it in a memory "frame", and returns it.

## Key Concepts
- **Frame**: A slot in memory that can hold exactly one 4KB `Page`. The `BufferPoolManager` is initialized with a fixed number of frames (e.g., 1024 frames = 4MB RAM usage).
- **Page Table**: A `HashMap` mapping a `PageId` to a `FrameId` to quickly determine if a page is currently cached.
- **Pinning**: When a system component is actively using a page, it is "pinned" (`pin_count > 0`). A pinned page cannot be evicted. Once the component is done, it must call `unpin_page()`.
- **Dirty Flag**: If a component modifies a page, it unpins it with `is_dirty = true`. When the BufferPoolManager evicts a dirty page, it knows it must first write the contents back to disk (`flush_page()`).

## Eviction Policy (LRU)
MiniDB implements a **Least Recently Used (LRU)** eviction policy. When the pool is full and a new page needs to be loaded, the pool searches for a frame with `pin_count == 0` that was accessed furthest in the past, and evicts it.
