use std::cmp::Ordering;
use std::collections::HashMap;

// Define a custom error type
#[derive(Debug)]
enum FieldError {
    BadTypeOfField,
    BadGet,
    NotImplemented,
    LogicalError,
    IllegalTypeOfArgument,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Null {
    Value(i32), // Representing positive and negative infinity
}

const NEGATIVE_INFINITY: Null = Null::Value(-1);
const POSITIVE_INFINITY: Null = Null::Value(1);

#[derive(Debug, PartialEq)]
pub enum Field {
    Int(i32),
    Float(f64),
    String(String),
    Array(Vec<Field>),
    Tuple(Vec<Field>),
    Map(HashMap<String, Field>),
    Object(HashMap<String, Field>),
    AggregateFunctionStateData(AggregateFunctionStateData),
}

#[derive(Debug, PartialEq,Eq)]
struct AggregateFunctionStateData {
    name: String,
    data: String,
}

impl AggregateFunctionStateData {
    fn new(name: String, data: String) -> Self {
        AggregateFunctionStateData { name, data }
    }
}

impl PartialOrd for AggregateFunctionStateData {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        None
    }
}

impl Ord for AggregateFunctionStateData {
    fn cmp(&self, _other: &Self) -> Ordering {
        unimplemented!("Operator < is not implemented for AggregateFunctionStateData.")
    }
}

#[derive(Debug, PartialEq,Eq)]
struct DecimalField<T> {
    value: T,
    scale: u32,
}
