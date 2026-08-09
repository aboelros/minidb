use crate::error::MiniDbError;
use super::ast::{Statement, Expression, Operator};
use super::lexer::Token;
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    fn consume_keyword(&mut self, expected: &str) -> Result<(), MiniDbError> {
        match self.tokens.next() {
            Some(Token::Keyword(kw)) if kw == expected => Ok(()),
            other => Err(MiniDbError::SyntaxError(format!("Expected keyword {}, got {:?}", expected, other))),
        }
    }

    fn consume_identifier(&mut self) -> Result<String, MiniDbError> {
        match self.tokens.next() {
            Some(Token::Identifier(id)) => Ok(id),
            Some(Token::Keyword(kw)) => Ok(kw.to_lowercase()), // Allow keywords as identifiers sometimes
            other => Err(MiniDbError::SyntaxError(format!("Expected identifier, got {:?}", other))),
        }
    }

    fn consume_symbol(&mut self, expected: char) -> Result<(), MiniDbError> {
        match self.tokens.next() {
            Some(Token::Symbol(sym)) if sym == expected => Ok(()),
            other => Err(MiniDbError::SyntaxError(format!("Expected symbol '{}', got {:?}", expected, other))),
        }
    }

    pub fn parse(&mut self) -> Result<Statement, MiniDbError> {
        match self.tokens.peek() {
            Some(Token::Keyword(kw)) => match kw.as_str() {
                "SELECT" => self.parse_select(),
                "INSERT" => self.parse_insert(),
                "UPDATE" => self.parse_update(),
                "DELETE" => self.parse_delete(),
                "CREATE" => self.parse_create(),
                "EXPLAIN" => {
                    self.tokens.next();
                    let stmt = self.parse()?;
                    Ok(Statement::Explain(Box::new(stmt)))
                },
                "BEGIN" => {
                    self.tokens.next();
                    self.consume_symbol(';').unwrap_or(());
                    Ok(Statement::Begin)
                },
                "COMMIT" => {
                    self.tokens.next();
                    self.consume_symbol(';').unwrap_or(());
                    Ok(Statement::Commit)
                },
                "ROLLBACK" => {
                    self.tokens.next();
                    self.consume_symbol(';').unwrap_or(());
                    Ok(Statement::Rollback)
                },
                _ => Err(MiniDbError::SyntaxError(format!("Unsupported keyword: {}", kw))),
            },
            _ => Err(MiniDbError::SyntaxError("Expected keyword at start of statement".into())),
        }
    }

    fn parse_create(&mut self) -> Result<Statement, MiniDbError> {
        self.tokens.next(); // Consume CREATE
        match self.tokens.next() {
            Some(Token::Keyword(kw)) if kw == "TABLE" => {
                let name = self.consume_identifier()?;
                self.consume_symbol('(')?;
                let mut columns = Vec::new();
                loop {
                    let col_name = self.consume_identifier()?;
                    let col_type_str = match self.tokens.next() {
                        Some(Token::Keyword(kw)) => kw,
                        other => return Err(MiniDbError::SyntaxError(format!("Expected data type, got {:?}", other))),
                    };
                    
                    let data_type = match col_type_str.as_str() {
                        "INTEGER" => crate::catalog::DataType::Integer,
                        "TEXT" => crate::catalog::DataType::Text,
                        "FLOAT" => crate::catalog::DataType::Float,
                        "BOOLEAN" => crate::catalog::DataType::Boolean,
                        _ => return Err(MiniDbError::SyntaxError(format!("Unsupported type: {}", col_type_str))),
                    };
                    
                    let mut is_primary_key = false;
                    let mut is_not_null = false;

                    // simple check for PRIMARY KEY
                    if let Some(Token::Keyword(kw)) = self.tokens.peek() {
                        if kw == "PRIMARY" {
                            self.tokens.next();
                            self.consume_keyword("KEY")?;
                            is_primary_key = true;
                            is_not_null = true;
                        }
                    }

                    columns.push(crate::catalog::Column {
                        name: col_name,
                        data_type,
                        is_primary_key,
                        is_not_null,
                    });

                    match self.tokens.peek() {
                        Some(Token::Symbol(',')) => {
                            self.tokens.next();
                        },
                        Some(Token::Symbol(')')) => {
                            self.tokens.next();
                            break;
                        },
                        other => return Err(MiniDbError::SyntaxError(format!("Expected ',' or ')', got {:?}", other))),
                    }
                }
                self.consume_symbol(';').unwrap_or(());
                Ok(Statement::CreateTable { name, columns })
            },
            Some(Token::Keyword(kw)) if kw == "INDEX" => {
                let index_name = self.consume_identifier()?;
                self.consume_keyword("ON")?;
                let table_name = self.consume_identifier()?;
                self.consume_symbol('(')?;
                let column_name = self.consume_identifier()?;
                self.consume_symbol(')')?;
                self.consume_symbol(';').unwrap_or(());
                Ok(Statement::CreateIndex { index_name, table_name, column_name })
            }
            other => Err(MiniDbError::SyntaxError(format!("Expected TABLE or INDEX, got {:?}", other)))
        }
    }

    fn parse_select(&mut self) -> Result<Statement, MiniDbError> {
        self.tokens.next(); // Consume SELECT
        
        let mut columns = Vec::new();
        match self.tokens.peek() {
            Some(Token::Symbol('*')) => {
                self.tokens.next();
                columns.push("*".into());
            },
            Some(Token::Operator(op)) if op == "*" => {
                self.tokens.next();
                columns.push("*".into());
            },
            _ => {
                columns.push(self.consume_identifier()?);
            }
        }
        
        self.consume_keyword("FROM")?;
        let table = self.consume_identifier()?;
        
        let mut where_clause = None;
        if let Some(Token::Keyword(kw)) = self.tokens.peek() {
            if kw == "WHERE" {
                self.tokens.next();
                where_clause = Some(self.parse_expression()?);
            }
        }
        self.consume_symbol(';').unwrap_or(());
        
        Ok(Statement::Select {
            columns,
            table,
            where_clause,
            order_by: None,
            limit: None,
        })
    }
    
    fn parse_insert(&mut self) -> Result<Statement, MiniDbError> {
        self.tokens.next(); // Consume INSERT
        self.consume_keyword("INTO")?;
        let table = self.consume_identifier()?;
        self.consume_keyword("VALUES")?;
        
        let mut values = Vec::new();
        loop {
            self.consume_symbol('(')?;
            let mut row = Vec::new();
            loop {
                row.push(self.parse_expression()?);
                match self.tokens.peek() {
                    Some(Token::Symbol(',')) => { self.tokens.next(); },
                    Some(Token::Symbol(')')) => { self.tokens.next(); break; },
                    other => return Err(MiniDbError::SyntaxError(format!("Expected ',' or ')', got {:?}", other))),
                }
            }
            values.push(row);
            
            match self.tokens.peek() {
                Some(Token::Symbol(',')) => { self.tokens.next(); },
                Some(Token::Symbol(';')) | Some(Token::EOF) => { self.tokens.next(); break; },
                other => return Err(MiniDbError::SyntaxError(format!("Expected ',' or ';', got {:?}", other))),
            }
        }
        
        Ok(Statement::Insert {
            table,
            columns: vec![],
            values,
        })
    }

    fn parse_update(&mut self) -> Result<Statement, MiniDbError> {
        self.tokens.next(); // Consume UPDATE
        let table = self.consume_identifier()?;
        self.consume_keyword("SET")?;
        
        let col = self.consume_identifier()?;
        self.consume_operator("=")?;
        let val = self.parse_expression()?;
        
        let mut where_clause = None;
        if let Some(Token::Keyword(kw)) = self.tokens.peek() {
            if kw == "WHERE" {
                self.tokens.next();
                where_clause = Some(self.parse_expression()?);
            }
        }
        self.consume_symbol(';').unwrap_or(());
        
        Ok(Statement::Update {
            table,
            assignments: vec![(col, val)],
            where_clause,
        })
    }

    fn parse_delete(&mut self) -> Result<Statement, MiniDbError> {
        self.tokens.next(); // Consume DELETE
        self.consume_keyword("FROM")?;
        let table = self.consume_identifier()?;
        
        let mut where_clause = None;
        if let Some(Token::Keyword(kw)) = self.tokens.peek() {
            if kw == "WHERE" {
                self.tokens.next();
                where_clause = Some(self.parse_expression()?);
            }
        }
        self.consume_symbol(';').unwrap_or(());
        
        Ok(Statement::Delete {
            table,
            where_clause,
        })
    }
    
    fn consume_operator(&mut self, expected: &str) -> Result<(), MiniDbError> {
        match self.tokens.next() {
            Some(Token::Operator(op)) if op == expected => Ok(()),
            other => Err(MiniDbError::SyntaxError(format!("Expected operator '{}', got {:?}", expected, other))),
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, MiniDbError> {
        let left = match self.tokens.next() {
            Some(Token::Number(num)) => Expression::LiteralInt(num.parse().unwrap()),
            Some(Token::StringLiteral(s)) => Expression::LiteralString(s),
            Some(Token::Identifier(id)) => Expression::ColumnRef(id),
            other => return Err(MiniDbError::SyntaxError(format!("Expected expression, got {:?}", other))),
        };
        
        if let Some(Token::Operator(op_str)) = self.tokens.peek() {
            let op = match op_str.as_str() {
                "=" => Operator::Eq,
                "!=" => Operator::NotEq,
                ">" => Operator::Gt,
                ">=" => Operator::GtEq,
                "<" => Operator::Lt,
                "<=" => Operator::LtEq,
                _ => return Ok(left), // Not a binary expression operator we handle here
            };
            self.tokens.next(); // consume operator
            let right = self.parse_expression()?;
            return Ok(Expression::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            });
        }
        
        Ok(left)
    }
}
