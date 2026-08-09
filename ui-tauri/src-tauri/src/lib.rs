use std::sync::Mutex;
use minidb::sql::lexer::Lexer;
use minidb::sql::parser::Parser;
use minidb::sql::ast::{Statement, Expression};
use minidb::types::Value;

pub struct DbState {
    pub in_memory_table: Mutex<Vec<Vec<Value>>>,
}

#[tauri::command]
fn execute_sql(state: tauri::State<'_, DbState>, sql: String) -> String {
    let mut output = String::new();
    let mut table = state.in_memory_table.lock().unwrap();

    for statement in sql.split(';') {
        let stmt_trimmed = statement.trim();
        if stmt_trimmed.is_empty() {
            continue;
        }

        let mut lexer = Lexer::new(stmt_trimmed);
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                output.push_str(&format!("Lexer Error: {:?}\n", e));
                continue;
            }
        };
        
        if tokens.is_empty() || (tokens.len() == 1 && matches!(tokens[0], minidb::sql::lexer::Token::EOF)) {
            continue;
        }

        let mut parser = Parser::new(tokens);
        let ast = match parser.parse() {
            Ok(a) => a,
            Err(e) => {
                output.push_str(&format!("Parser Error: {:?}\n", e));
                continue;
            }
        };
        
        output.push_str(&format!("SQL: {};\n", stmt_trimmed));
        output.push_str(&format!("AST: {:?}\n", ast));

        match ast {
            Statement::Insert { values, .. } => {
                let mut inserted = 0;
                for row_ast in values {
                    let mut row_vals = Vec::new();
                    for expr in row_ast {
                        if let Expression::LiteralInt(v) = expr {
                            row_vals.push(Value::Integer(v));
                        } else if let Expression::LiteralString(s) = expr {
                            row_vals.push(Value::Text(s));
                        } else {
                            row_vals.push(Value::Null);
                        }
                    }
                    let serialized = Value::serialize_row(&row_vals);
                    output.push_str(&format!("  => Inserting Tuple: {:?} (Serialized bytes: {:?})\n", row_vals, serialized));
                    table.push(row_vals);
                    inserted += 1;
                }
            }
            Statement::Select { .. } => {
                output.push_str("  => Select Results:\n");
                for (i, row) in table.iter().enumerate() {
                    let serialized = Value::serialize_row(row);
                    let deserialized = Value::deserialize_row(&serialized).unwrap();
                    output.push_str(&format!("    Row {}: {:?}\n", i, deserialized));
                }
            }
            _ => {
                output.push_str("  => Statement OK\n");
            }
        }
        output.push_str("--------------------------------------------------\n");
    }
    
    output
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(DbState {
            in_memory_table: Mutex::new(Vec::new()),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![execute_sql])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
