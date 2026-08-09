use crate::transaction::TransactionId;
use crate::storage::page::{PageId, RecordId};
use crate::error::MiniDbError;

pub type LSN = u32; // Log Sequence Number

#[derive(Debug, Clone)]
pub enum LogRecordType {
    TxnBegin,
    TxnCommit,
    TxnAbort,
    Insert {
        page_id: PageId,
        record_id: RecordId,
        tuple_data: Vec<u8>,
    },
    Update {
        page_id: PageId,
        record_id: RecordId,
        old_data: Vec<u8>,
        new_data: Vec<u8>,
    },
    Delete {
        page_id: PageId,
        record_id: RecordId,
        old_data: Vec<u8>,
    }
}

#[derive(Debug, Clone)]
pub struct LogRecord {
    pub lsn: LSN,
    pub txn_id: TransactionId,
    pub prev_lsn: LSN,
    pub record_type: LogRecordType,
}

pub struct LogManager {
    next_lsn: LSN,
    // Typically holds a buffer to write logs before flushing to disk
    // file: File,
}

impl Default for LogManager {
    fn default() -> Self {
        Self::new()
    }
}

impl LogManager {
    pub fn new() -> Self {
        Self {
            next_lsn: 1,
        }
    }

    pub fn append_log_record(&mut self, _record: LogRecord) -> Result<LSN, MiniDbError> {
        // Pseudo-logic:
        // 1. Assign next_lsn to the record.
        // 2. Serialize the record into the log buffer.
        // 3. If log buffer is full, flush to disk.
        
        let assigned_lsn = self.next_lsn;
        self.next_lsn += 1;
        Ok(assigned_lsn)
    }

    pub fn flush(&mut self) -> Result<(), MiniDbError> {
        // Pseudo-logic:
        // Write the contents of the log buffer to the WAL file on disk.
        // Called by TransactionManager on commit.
        Ok(())
    }

    pub fn recover(&mut self) -> Result<(), MiniDbError> {
        // Pseudo-logic:
        // 1. Analysis pass: Scan WAL from beginning to find active vs committed txns.
        // 2. Redo pass: Re-apply all operations (ARIES style) to get pages to crash state.
        // 3. Undo pass: Rollback operations of uncommitted transactions using prev_lsn links.
        Ok(())
    }
}
