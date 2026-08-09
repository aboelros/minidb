use crate::sql::ast::Expression;

#[derive(Debug, Clone)]
pub enum LogicalPlan {
    SeqScan {
        table_name: String,
    },
    Filter {
        predicate: Expression,
        child: Box<LogicalPlan>,
    },
    Projection {
        columns: Vec<String>,
        child: Box<LogicalPlan>,
    },
    Sort {
        order_by: String,
        is_desc: bool,
        child: Box<LogicalPlan>,
    },
    Limit {
        limit: usize,
        child: Box<LogicalPlan>,
    },
    Insert {
        table_name: String,
        columns: Vec<String>,
        values: Vec<Vec<Expression>>,
    },
    Update {
        table_name: String,
        assignments: Vec<(String, Expression)>,
        child: Box<LogicalPlan>,
    },
    Delete {
        table_name: String,
        child: Box<LogicalPlan>,
    }
}
