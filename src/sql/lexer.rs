use crate::error::MiniDbError;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    StringLiteral(String),
    Number(String),
    Symbol(char),
    Operator(String),
    EOF,
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, MiniDbError> {
        let mut tokens = Vec::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else if c.is_alphabetic() || c == '_' {
                tokens.push(self.consume_identifier_or_keyword());
            } else if c.is_numeric() {
                tokens.push(self.consume_number());
            } else if c == '\'' {
                tokens.push(self.consume_string()?);
            } else if "!=<>+-*/".contains(c) {
                tokens.push(self.consume_operator());
            } else if "(),;*".contains(c) {
                self.chars.next();
                tokens.push(Token::Symbol(c));
            } else {
                return Err(MiniDbError::SyntaxError(format!("Unexpected character: {}", c)));
            }
        }
        tokens.push(Token::EOF);
        Ok(tokens)
    }

    fn consume_identifier_or_keyword(&mut self) -> Token {
        let mut result = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_alphanumeric() || c == '_' {
                result.push(c);
                self.chars.next();
            } else {
                break;
            }
        }
        let upper = result.to_uppercase();
        match upper.as_str() {
            "SELECT" | "FROM" | "WHERE" | "INSERT" | "INTO" | "VALUES" | "UPDATE" | "SET" | 
            "DELETE" | "CREATE" | "TABLE" | "INDEX" | "ON" | "PRIMARY" | "KEY" | "ORDER" | 
            "BY" | "ASC" | "DESC" | "LIMIT" | "AND" | "OR" | "NULL" | "BEGIN" | "COMMIT" | 
            "ROLLBACK" | "EXPLAIN" | "INTEGER" | "TEXT" | "FLOAT" | "BOOLEAN" => Token::Keyword(upper),
            _ => Token::Identifier(result),
        }
    }

    fn consume_number(&mut self) -> Token {
        let mut result = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_numeric() || c == '.' {
                result.push(c);
                self.chars.next();
            } else {
                break;
            }
        }
        Token::Number(result)
    }

    fn consume_string(&mut self) -> Result<Token, MiniDbError> {
        self.chars.next(); // consume opening quote
        let mut result = String::new();
        while let Some(&c) = self.chars.peek() {
            if c == '\'' {
                self.chars.next(); // consume closing quote
                return Ok(Token::StringLiteral(result));
            } else {
                result.push(c);
                self.chars.next();
            }
        }
        Err(MiniDbError::SyntaxError("Unterminated string literal".into()))
    }

    fn consume_operator(&mut self) -> Token {
        let mut result = String::new();
        while let Some(&c) = self.chars.peek() {
            if "!=<>+-*/".contains(c) {
                result.push(c);
                self.chars.next();
            } else {
                break;
            }
        }
        Token::Operator(result)
    }
}
