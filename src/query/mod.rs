pub mod logical_plan;
pub mod physical_plan;

use crate::sql::ast::Statement;
use crate::error::MiniDbError;
use self::logical_plan::LogicalPlan;
use self::physical_plan::PhysicalPlan;

pub struct Planner;

impl Planner {
    pub fn new() -> Self {
        Self
    }

    pub fn create_logical_plan(&self, stmt: Statement) -> Result<LogicalPlan, MiniDbError> {
        match stmt {
            Statement::Select { columns, table, where_clause, order_by, limit } => {
                let mut plan = LogicalPlan::SeqScan { table_name: table };

                if let Some(expr) = where_clause {
                    plan = LogicalPlan::Filter { predicate: expr, child: Box::new(plan) };
                }
                
                if let Some((col, is_desc)) = order_by {
                    plan = LogicalPlan::Sort { order_by: col, is_desc, child: Box::new(plan) };
                }

                if let Some(l) = limit {
                    plan = LogicalPlan::Limit { limit: l, child: Box::new(plan) };
                }

                plan = LogicalPlan::Projection { columns, child: Box::new(plan) };

                Ok(plan)
            },
            Statement::Insert { table, columns, values } => {
                Ok(LogicalPlan::Insert { table_name: table, columns, values })
            },
            // ... Implement other statements ...
            _ => Err(MiniDbError::SyntaxError("Statement not supported for logical planning yet".into()))
        }
    }

    pub fn create_physical_plan(&self, logical_plan: LogicalPlan) -> Result<PhysicalPlan, MiniDbError> {
        // Pseudo-logic:
        // Here we would consult the catalog to see if an index exists for the Filter condition.
        // For now, we do a basic mapping to SeqScan.
        
        match logical_plan {
            LogicalPlan::SeqScan { table_name } => Ok(PhysicalPlan::SeqScan { table_name }),
            LogicalPlan::Filter { predicate, child } => {
                let physical_child = self.create_physical_plan(*child)?;
                Ok(PhysicalPlan::Filter { predicate, child: Box::new(physical_child) })
            },
            // Map the rest of the nodes directly...
            LogicalPlan::Projection { columns, child } => {
                Ok(PhysicalPlan::Projection { columns, child: Box::new(self.create_physical_plan(*child)?) })
            }
            LogicalPlan::Limit { limit, child } => {
                Ok(PhysicalPlan::Limit { limit, child: Box::new(self.create_physical_plan(*child)?) })
            }
            LogicalPlan::Insert { table_name, .. } => {
                Ok(PhysicalPlan::Insert { table_name })
            }
            _ => Err(MiniDbError::SyntaxError("Logical plan node not supported for physical planning yet".into()))
        }
    }
}
