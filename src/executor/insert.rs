use super::Executor;
use crate::error::MiniDbError;
use crate::types::Value;

pub struct InsertExecutor {
    table_name: String,
    values: Vec<Vec<Value>>,
    index: usize,
}

impl InsertExecutor {
    pub fn new(table_name: String, values: Vec<Vec<Value>>) -> Self {
        Self {
            table_name,
            values,
            index: 0,
        }
    }
}

impl Executor for InsertExecutor {
    fn init(&mut self) -> Result<(), MiniDbError> {
        self.index = 0;
        Ok(())
    }

    fn next(&mut self) -> Result<Option<Vec<Value>>, MiniDbError> {
        if self.index < self.values.len() {
            let row = &self.values[self.index];
            self.index += 1;
            
            // Pseudo-logic:
            // 1. Find table in catalog
            // 2. Serialize row
            // 3. Fetch last page from buffer pool
            // 4. Try insert. If full, allocate new page.
            // 5. Return inserted row data
            
            Ok(Some(row.clone()))
        } else {
            Ok(None)
        }
    }
}
