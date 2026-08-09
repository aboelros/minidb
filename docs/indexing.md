# Indexing Subsystem

MiniDB uses a **B+ Tree** for all indexing.

## B+ Tree Characteristics
A B+ Tree is a self-balancing tree data structure that keeps data sorted and allows searches, sequential access, insertions, and deletions in logarithmic time (`O(log n)`).

Key features in our implementation:
1. **Nodes are Pages**: Every node (Internal or Leaf) maps exactly to a 4KB `Page` in the storage engine.
2. **Internal Nodes**: Contain only search keys and child pointers (PageIds). They do not contain actual records.
3. **Leaf Nodes**: Contain keys and pointers to the actual data (`RecordId`). 
4. **Leaf Linked List**: Leaf nodes contain a `next_leaf` pointer. This allows rapid sequential `range_scan` queries (e.g. `WHERE age > 18 AND age < 30`) without needing to traverse back up the tree.

## Structure

```text
       [Internal Node]
        keys: [20, 50]
       /      |       \
      /       |        \
[Leaf 1]   [Leaf 2]    [Leaf 3]
 < 20       20-49       >= 50
  |---------->|---------->| (next pointers)
```

## Operations
- **Insert**: Traverses to the correct leaf. If the leaf is full (`> MAX_KEYS`), it splits the leaf in half and promotes the middle key to the parent internal node. This split can cascade up to the root.
- **Search**: Performs a binary search within internal nodes to find the correct path, and then a binary search within the leaf node to find the exact `RecordId`.
- **Delete**: Removes the key from the leaf. Complex implementations will merge under-full nodes (underflow handling), though simplified implementations may leave under-full nodes and rely on occasional vacuuming.
