use super::IColumn;
use super::{StringRef,Field};
use core::fmt::Debug;

#[derive(Debug)]
pub struct ColumnAggregateFunction {
    // Container for data
    data: Vec<Option<Box<dyn IColumn>>>,
    // Whether to keep the state or not
    keep_state: bool,
}

impl ColumnAggregateFunction {
    pub fn new() -> Self {
        ColumnAggregateFunction {
            data: Vec::new(),
            keep_state: false,
        }
    }

    pub fn add_data(&mut self, value: Option<Box<dyn IColumn>>) {
        self.data.push(value);
    }

    pub fn set_keep_state(&mut self, keep_state: bool) {
        self.keep_state = keep_state;
    }
}

impl IColumn for ColumnAggregateFunction {
    fn clone_box(&self) -> Box<dyn IColumn> {
        Box::new(self.clone())
    }

    fn get_family_name(&self) -> &str {
        "AggregateFunction"
    }

    fn get_data_type(&self) -> &str {
        "AggregateFunction"
    }

    fn convert_to_full_column_if_const(&self) -> Box<dyn IColumn> {
        Box::new(self.clone())
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn get_field(&self, _n: usize) -> Field {
        panic!("Cannot get value from {}", self.get_name());
    }

    fn get(&self, _n: usize, _res: &mut Field) {
        panic!("Cannot get value from {}", self.get_name());
    }

    fn get_data_at(&self, _n: usize) -> StringRef {
        panic!("Cannot get data from {}", self.get_name());
    }
}

impl Clone for ColumnAggregateFunction {
    fn clone(&self) -> ColumnAggregateFunction {
        ColumnAggregateFunction {
            data: self.data.clone(),
            keep_state: self.keep_state,
        }
    }
}
