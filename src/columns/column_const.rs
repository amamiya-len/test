use super::IColumn;
use super::{StringRef,Field};
use core::fmt::Debug;

#[derive(Debug, Clone)]
pub struct ColumnConst {
    data: Box<dyn IColumn>,
    size: usize,
}

impl ColumnConst {
    pub fn new(data: Box<dyn IColumn>, size: usize) -> Self {
        ColumnConst { data, size }
    }
}

impl IColumn for ColumnConst {
    fn clone_box(&self) -> Box<dyn IColumn> {
        Box::new(self.clone())
    }

    fn get_family_name(&self) -> &str {
        "Const"
    }

    fn get_data_type(&self) -> &str {
        "Const"
    }

    fn convert_to_full_column_if_const(&self) -> Box<dyn IColumn> {
        Box::new(self.clone())
    }

    fn size(&self) -> usize {
        self.size
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
