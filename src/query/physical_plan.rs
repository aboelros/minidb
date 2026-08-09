use crate::sql::ast::Expression;

#[derive(Debug, Clone)]
pub enum PhysicalPlan {
    SeqScan {
        table_name: String,
    },
    IndexScan {
        index_name: String,
        predicate: Expression,
    },
    Filter {
        predicate: Expression,
        child: Box<PhysicalPlan>,
    },
    Projection {
        columns: Vec<String>,
        child: Box<PhysicalPlan>,
    },
    Sort {
        order_by: String,
        is_desc: bool,
        child: Box<PhysicalPlan>,
    },
    Limit {
        limit: usize,
        child: Box<PhysicalPlan>,
    },
    Insert {
        table_name: String,
        // Expressions would be evaluated to Values at execution time
    },
    Update {
        table_name: String,
        child: Box<PhysicalPlan>,
    },
    Delete {
        table_name: String,
        child: Box<PhysicalPlan>,
    }
}
