use clap::Parser;
use std::path::PathBuf;
use std::io::{self, Write};
use crate::error::MiniDbError;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    
    /// Path to the database directory (for REPL)
    pub db_path: Option<PathBuf>,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    Run {
        /// The path to the SQL script to execute
        script_path: PathBuf,
    },
}

pub fn start_repl(_db_path: &PathBuf) -> Result<(), MiniDbError> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("minidb> ");
        stdout.flush().map_err(MiniDbError::IoError)?;

        let mut input = String::new();
        stdin.read_line(&mut input).map_err(MiniDbError::IoError)?;

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        match input {
            ".quit" | ".exit" => {
                println!("Goodbye.");
                break;
            }
            ".help" => {
                println!("MiniDB Commands:");
                println!("  .help    - Show this message");
                println!("  .exit    - Exit the shell");
                println!("  .quit    - Exit the shell");
                println!("  <sql>    - Execute a SQL statement");
            }
            _ => {
                if input.starts_with('.') {
                    println!("Unknown command: {}", input);
                } else {
                    println!("Unrecognized statement: {}", input);
                    // TODO: Pass to SQL Parser
                }
            }
        }
    }

    Ok(())
}

pub fn run_script(script_path: &PathBuf) -> Result<(), MiniDbError> {
    let script_contents = std::fs::read_to_string(script_path)?;
    
    // Simulate memory-only execution to verify the engine parses and serializes correctly
    println!("Executing script: {}", script_path.display());
    println!("--------------------------------------------------");

    let mut in_memory_table: Vec<Vec<crate::types::Value>> = Vec::new();

    for statement in script_contents.split(';') {
        let stmt_trimmed = statement.trim();
        if stmt_trimmed.is_empty() {
            continue;
        }

        // 1. Lexing
        let mut lexer = crate::sql::lexer::Lexer::new(stmt_trimmed);
        let tokens = lexer.tokenize()?;
        
        if tokens.is_empty() || (tokens.len() == 1 && matches!(tokens[0], crate::sql::lexer::Token::EOF)) {
            continue;
        }

        // 2. Parsing
        let mut parser = crate::sql::parser::Parser::new(tokens);
        let ast = parser.parse()?;
        
        println!("SQL: {};", stmt_trimmed);
        println!("AST: {:?}", ast);

        // 3. Execution Simulation (Insert & Select)
        match ast {
            crate::sql::ast::Statement::Insert { values, .. } => {
                for row_ast in values {
                    let mut row_vals = Vec::new();
                    for expr in row_ast {
                        if let crate::sql::ast::Expression::LiteralInt(v) = expr {
                            row_vals.push(crate::types::Value::Integer(v));
                        } else if let crate::sql::ast::Expression::LiteralString(s) = expr {
                            row_vals.push(crate::types::Value::Text(s));
                        } else {
                            row_vals.push(crate::types::Value::Null);
                        }
                    }
                    let serialized = crate::types::Value::serialize_row(&row_vals);
                    println!("  => Inserting Tuple: {:?} (Serialized bytes: {:?})", row_vals, serialized);
                    in_memory_table.push(row_vals);
                }
            }
            crate::sql::ast::Statement::Select { .. } => {
                println!("  => Select Results:");
                for (i, row) in in_memory_table.iter().enumerate() {
                    let serialized = crate::types::Value::serialize_row(row);
                    let deserialized = crate::types::Value::deserialize_row(&serialized).unwrap();
                    println!("    Row {}: {:?}", i, deserialized);
                }
            }
            _ => {
                println!("  => Statement OK");
            }
        }
        println!("--------------------------------------------------");
    }
    
    println!("Script execution complete.");
    Ok(())
}
