use super::IColumn;
use super::{StringRef,Field};
use core::fmt::Debug;

#[derive(Debug, Clone)]
pub struct ColumnArray {
    data: Vec<Box<dyn IColumn>>,
    offsets: Vec<usize>,
}

impl ColumnArray {
    pub fn new(data: Vec<Box<dyn IColumn>>, offsets: Vec<usize>) -> Self {
        ColumnArray { data, offsets }
    }
}

impl IColumn for ColumnArray {
    fn clone_box(&self) -> Box<dyn IColumn> {
        Box::new(self.clone())
    }

    fn get_family_name(&self) -> &str {
        "Array"
    }

    fn get_data_type(&self) -> &str {
        "Array"
    }

    fn convert_to_full_column_if_const(&self) -> Box<dyn IColumn> {
        Box::new(self.clone())
    }

    fn size(&self) -> usize {
        self.offsets.len()
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