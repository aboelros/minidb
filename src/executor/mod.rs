use crate::error::MiniDbError;
use crate::types::Value;

pub mod seq_scan;
pub mod insert;
pub mod update;
pub mod delete;

pub trait Executor {
    fn init(&mut self) -> Result<(), MiniDbError>;
    fn next(&mut self) -> Result<Option<Vec<Value>>, MiniDbError>;
}
