use std::sync::atomic::{AtomicU32, Ordering};

pub type TransactionId = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionState {
    Active,
    Committed,
    Aborted,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: TransactionId,
    pub state: TransactionState,
    // Future: write_set to keep track of written tuples for rollback/MVCC
}

impl Transaction {
    pub fn new(id: TransactionId) -> Self {
        Self {
            id,
            state: TransactionState::Active,
        }
    }
}

pub struct TransactionManager {
    next_txn_id: AtomicU32,
    // Typically holds a reference to a LockManager or LogManager
}

impl TransactionManager {
    pub fn new() -> Self {
        Self {
            next_txn_id: AtomicU32::new(1),
        }
    }

    pub fn begin(&self) -> Transaction {
        let id = self.next_txn_id.fetch_add(1, Ordering::SeqCst);
        Transaction::new(id)
    }

    pub fn commit(&self, txn: &mut Transaction) {
        // Pseudo-logic:
        // 1. Flush WAL buffers to disk for durability.
        // 2. Release locks held by this transaction.
        txn.state = TransactionState::Committed;
    }

    pub fn rollback(&self, txn: &mut Transaction) {
        // Pseudo-logic:
        // 1. Iterate backwards through txn's write_set.
        // 2. Apply inverse operations to restore previous state.
        // 3. Release locks.
        txn.state = TransactionState::Aborted;
    }
}
