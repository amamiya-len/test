
pub(crate) mod parser;
pub(crate) mod logger;
pub(crate) mod columns;
pub(crate) mod core;
pub(crate) mod ast;

use logos::Logos;
use parser::sql;
use parser::token::*;
use parser::token::TokenKind;
use logger::*;
use std::sync::Arc;
use columns::IColumn;
use columns::column_vector::ColumnVector;
use columns::column_string::ColumnString;

fn main() {
	initialize_logging();
	
	
	log_info("hello from proton!");
	
	let log_store = InMemoryLogStore::new();
    let log_entry = Arc::new(LogEntry::new(1, "Log entry data".to_string()));

    log_store.append(1, log_entry.clone());
    log_info(&format!("Log size: {}", log_store.size()));

    if let Some(entry) = log_store.get(1) {
        log_info(&format!("Log entry term: {}", entry.get_term()));
        log_info(&format!("Log entry data: {}", entry.get_data()));
    }

    log_store.remove(1);
    log_info(&format!("Log size after removal: {}", log_store.size()));
	
	let col = ColumnVector::new(vec![1, 2, 3, 4]);
    let col_box: Box<dyn IColumn> = Box::new(col);

    log_info(&format!("Column family name: {}", col_box.get_family_name()));
    log_info(&format!("Column size: {}", col_box.size()));
    let data_ref = col_box.get_data_at(1);
    log_info(&format!("Data at index 1: {:?}", data_ref.get_data()));
	
	let mut col = ColumnString::new();
    col.insert("hello");
    col.insert("world");

    let col_box: Box<dyn IColumn> = Box::new(col);

    log_info(&format!("Column family name: {}", col_box.get_family_name()));
    log_info(&format!("Column size: {}", col_box.size()));
    let data_ref = col_box.get_data_at(1);
    log_info(&format!("Data at index 1: {:?}", std::str::from_utf8(data_ref.get_data())));

    let sql = "SELECT distinct a from b 
     select c from d
     create table e (f int, g text)
     ";
    let lexer = Lexer::new(sql);

    let result = sql::SqlParser::new().parse(lexer);

    match result {
        Ok(ast) => log_debug(&format!("AST: {:#?}", ast)),
        Err(e) => log_error(&format!("{:?}", e)),
    }
}
