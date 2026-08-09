use crate::storage::page::{Page, PageId};
use crate::storage::disk_manager::DiskManager;
use crate::error::MiniDbError;
use std::collections::HashMap;

pub type FrameId = usize;

pub struct BufferPoolManager {
    disk_manager: DiskManager,
    pool_size: usize,
    frames: Vec<Page>,
    page_table: HashMap<PageId, FrameId>,
    pin_counts: Vec<u32>,
    // For LRU eviction:
    usage_list: Vec<FrameId>,
}

impl BufferPoolManager {
    pub fn new(pool_size: usize, disk_manager: DiskManager) -> Self {
        let mut frames = Vec::with_capacity(pool_size);
        for _ in 0..pool_size {
            // Initialize with dummy pages
            frames.push(Page::new(0));
        }

        Self {
            disk_manager,
            pool_size,
            frames,
            page_table: HashMap::new(),
            pin_counts: vec![0; pool_size],
            usage_list: Vec::new(), // In a real implementation, a doubly-linked list or similar is better for O(1) LRU updates
        }
    }

    pub fn fetch_page(&mut self, page_id: PageId) -> Result<&mut Page, MiniDbError> {
        // Pseudo-logic:
        // 1. If page is in page_table, increment pin_count, update LRU usage_list, return frame.
        // 2. If page is NOT in page_table:
        //    a. Find a victim frame (pin_count == 0, using LRU).
        //    b. If victim is dirty, flush it to disk via disk_manager.
        //    c. Read the requested page_id from disk into the victim frame.
        //    d. Update page_table, set pin_count to 1, update LRU.
        //    e. Return frame.
        Err(MiniDbError::StorageError("BufferPoolManager::fetch_page not fully implemented".into()))
    }

    pub fn unpin_page(&mut self, page_id: PageId, is_dirty: bool) -> Result<(), MiniDbError> {
        // Pseudo-logic:
        // 1. Find frame in page_table.
        // 2. Decrement pin_count.
        // 3. If is_dirty is true, mark frame as dirty (do not overwrite true with false).
        Ok(())
    }

    pub fn flush_page(&mut self, page_id: PageId) -> Result<(), MiniDbError> {
        // Pseudo-logic:
        // 1. If page is in page_table, force write to disk via disk_manager.
        // 2. Unset dirty flag.
        Ok(())
    }

    pub fn new_page(&mut self) -> Result<&mut Page, MiniDbError> {
        // Pseudo-logic:
        // 1. Allocate new page_id via disk_manager.
        // 2. Find a victim frame (flush if dirty).
        // 3. Initialize new page in that frame.
        // 4. Return frame.
        Err(MiniDbError::StorageError("BufferPoolManager::new_page not fully implemented".into()))
    }
}
