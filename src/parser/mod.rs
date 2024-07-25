pub mod token;
use crate::ast::SqlStmt;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub(crate) sql,"/parser/sql.rs");


#[test]
fn parse_select() {
    let lexer = token::Lexer::new("
    select a from b ; 
    select c from d;"
);
    assert!(sql::SqlParser::new().parse(lexer).is_ok());

}