use crate::transaction::TransactionId;
use crate::storage::page::RecordId;
use crate::error::MiniDbError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockMode {
    Shared,
    Exclusive,
}

#[allow(dead_code)]
struct LockRequest {
    txn_id: TransactionId,
    mode: LockMode,
    granted: bool,
}

#[allow(dead_code)]
pub struct LockManager {
    // Maps a record to the queue of lock requests for it
    record_locks: Arc<Mutex<HashMap<RecordId, Vec<LockRequest>>>>,
}

impl Default for LockManager {
    fn default() -> Self {
        Self::new()
    }
}

impl LockManager {
    pub fn new() -> Self {
        Self {
            record_locks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn lock_shared(&self, _txn_id: TransactionId, _record_id: RecordId) -> Result<(), MiniDbError> {
        // Pseudo-logic (Two-Phase Locking - 2PL):
        // 1. Check if txn is in shrinking phase (if strict 2PL).
        // 2. Acquire mutex on record_locks.
        // 3. If another txn holds Exclusive lock, wait/block.
        // 4. Otherwise, grant Shared lock and add to queue.
        Ok(())
    }

    pub fn lock_exclusive(&self, _txn_id: TransactionId, _record_id: RecordId) -> Result<(), MiniDbError> {
        // Pseudo-logic:
        // 1. Check if txn is in shrinking phase.
        // 2. Acquire mutex.
        // 3. If any other txn holds Shared or Exclusive lock, wait/block.
        // 4. If this txn already holds Shared, upgrade to Exclusive.
        // 5. Grant Exclusive lock.
        Ok(())
    }

    pub fn unlock(&self, _txn_id: TransactionId, _record_id: RecordId) -> Result<(), MiniDbError> {
        // Pseudo-logic:
        // 1. Remove txn_id from the queue for record_id.
        // 2. Wake up any waiting transactions.
        // 3. Mark txn as entering shrinking phase (for 2PL).
        Ok(())
    }
}
