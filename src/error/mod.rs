use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum MiniDbError {
    #[error("I/O Error: {0}")]
    IoError(#[from] io::Error),
    
    #[error("Syntax Error: {0}")]
    SyntaxError(String),
    
    #[error("Table not found: {0}")]
    TableNotFound(String),
    
    #[error("Column not found: {0}")]
    ColumnNotFound(String),
    
    #[error("Type mismatch: expected {expected}, got {found}")]
    TypeMismatch {
        expected: String,
        found: String,
    },
    
    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),
    
    #[error("Corrupt page: {0}")]
    CorruptPage(String),
    
    #[error("Internal storage error: {0}")]
    StorageError(String),
}
