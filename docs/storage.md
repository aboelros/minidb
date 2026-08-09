# Storage Subsystem

The storage subsystem in MiniDB is responsible for persisting data to disk in a structured, durable manner.

## Page Layout
MiniDB uses fixed-size **4096-byte (4KB)** pages. 
This aligns with typical OS and SSD block sizes, ensuring efficient read/write operations without partial page torn writes under normal operation.

### Slotted Page Structure
Each page containing records uses a **slotted page layout**. This approach allows records (tuples) to vary in size (e.g. TEXT types) and makes deletion and reorganization within a page much simpler.

```text
---------------------------------------------------------
| PageId (4) | LSN (4) | FreeSpacePtr (2) | NumSlots (2) | -> Header (12 bytes)
---------------------------------------------------------
| Tuple0_Offset (2) | Tuple0_Size (2) | -> Slot 0
---------------------------------------------------------
| Tuple1_Offset (2) | Tuple1_Size (2) | -> Slot 1
---------------------------------------------------------
| ... Free Space ...                                    |
---------------------------------------------------------
| Tuple 1 Data | Tuple 0 Data                           |
---------------------------------------------------------
```

- **Header**: Stores metadata (PageId, Log Sequence Number for WAL, FreeSpace pointer, and number of slots).
- **Slot Directory**: Grows downwards from the header. Stores the offset and size of each tuple.
- **Tuples**: Inserted starting from the end of the page and growing upwards.

When a tuple is deleted, we set its size in the slot directory to `0`, creating a tombstone. During a compaction phase (vacuuming), we can reclaim this space.

## Disk Manager
The `DiskManager` abstracts file I/O. It maps a `PageId` to an offset in the underlying file (`PageId * 4096`).
When asked to allocate a page, it increases the internal counter and expands the file size implicitly by writing at the new offset.

## Record IDs (RID)
A `RecordId` uniquely identifies a tuple globally in the database.
It is composed of:
1. `PageId` (32-bit unsigned integer)
2. `Slot Number` (16-bit unsigned integer)

Indexes store these RIDs to point back to the actual data.
