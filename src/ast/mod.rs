use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum SqlValue {
    Null,
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SqlType {
    Integer,
    Float,
    Text,
    Boolean,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColumnDef {
    pub name: String,
    pub col_type: SqlType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SqlExpr {
    Column(String),
    Value(SqlValue),
    BinaryOp {
        left: Box<SqlExpr>,
        op: SqlBinaryOperator,
        right: Box<SqlExpr>,
    },
    UnaryOp{
        op: SqlUnaryOperator,
        expr: Box<SqlExpr>,
    }
}




#[derive(Debug, Clone, PartialEq)]
pub enum SqlUnaryOperator {
    Neg,
}


#[derive(Debug, Clone, PartialEq)]
pub enum SqlBinaryOperator {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Mul,
    Div,
    Add,
    Sub,
    Mod,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectStmt {
    pub columns: Vec<SelectedColumnDef>,
    pub table: String,
    pub where_clause: Option<SqlExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectedColumnDef {
    pub name: String,
    pub alias: Option<String>,
    pub distinct: bool,
}


#[derive(Debug, Clone, PartialEq)]
pub struct InsertStmt {
    pub table: String,
    pub values: HashMap<String, SqlValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateTableStmt {
    pub table: String,
    pub columns: Vec<ColumnDef>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SqlStmt {
    Select(SelectStmt),
    Insert(InsertStmt),
    CreateTable(CreateTableStmt),
    DropStmt,
}


