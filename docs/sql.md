# SQL Subsystem

The SQL subsystem is responsible for taking raw string queries from the user and translating them into structured Abstract Syntax Trees (ASTs) that the Query Planner can understand.

## 1. Lexer (`lexer.rs`)
The `Lexer` breaks down a raw SQL string into a series of `Token`s. It handles:
- Keywords (e.g., `SELECT`, `FROM`)
- Identifiers (e.g., `users`, `age`)
- Literals (Numbers, Strings, Booleans)
- Operators (e.g., `=`, `<`, `AND`)
- Symbols (e.g., `,`, `;`, `(`, `)`)

## 2. Parser (`parser.rs`)
The `Parser` implements a recursive-descent parsing strategy. It consumes a stream of tokens provided by the Lexer and constructs an AST based on SQL grammar rules.

It currently supports basic structures for:
- `SELECT`
- `INSERT`
- `UPDATE`
- `DELETE`
- `CREATE TABLE`
- `CREATE INDEX`
- Transaction commands (`BEGIN`, `COMMIT`, `ROLLBACK`)

## 3. Abstract Syntax Tree (`ast.rs`)
The AST defines the internal representation of SQL statements and expressions.

- **Statement**: Represents a complete SQL query or command.
- **Expression**: Represents a value, column reference, or a mathematical/logical operation (e.g., `age > 18`).

The resulting `Statement` from the Parser is then passed to the Query Planner for further translation into logical and physical execution plans.
