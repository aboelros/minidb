#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    And,
    Or,
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    LiteralInt(i64),
    LiteralFloat(f64),
    LiteralString(String),
    LiteralBool(bool),
    LiteralNull,
    ColumnRef(String),
    BinaryOp {
        left: Box<Expression>,
        op: Operator,
        right: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Select {
        columns: Vec<String>,
        table: String,
        where_clause: Option<Expression>,
        order_by: Option<(String, bool)>, // bool = is_desc
        limit: Option<usize>,
    },
    Insert {
        table: String,
        columns: Vec<String>,
        values: Vec<Vec<Expression>>,
    },
    Update {
        table: String,
        assignments: Vec<(String, Expression)>,
        where_clause: Option<Expression>,
    },
    Delete {
        table: String,
        where_clause: Option<Expression>,
    },
    CreateTable {
        name: String,
        columns: Vec<crate::catalog::Column>,
    },
    CreateIndex {
        index_name: String,
        table_name: String,
        column_name: String,
    },
    Begin,
    Commit,
    Rollback,
    Explain(Box<Statement>),
}
