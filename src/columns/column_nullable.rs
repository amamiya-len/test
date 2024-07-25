use super::IColumn;
use super::{StringRef,Field};
use core::fmt::Debug;

#[derive(Debug, Clone)]
pub struct ColumnNullable {
    data: Box<dyn IColumn>,
    null_map: Vec<bool>,
}

impl ColumnNullable {
    pub fn new(data: Box<dyn IColumn>, null_map: Vec<bool>) -> Self {
        ColumnNullable { data, null_map }
    }
}

impl IColumn for ColumnNullable {
    fn clone_box(&self) -> Box<dyn IColumn> {
        Box::new(self.clone())
    }

    fn get_family_name(&self) -> &str {
        "Nullable"
    }

    fn get_data_type(&self) -> &str {
        "Nullable"
    }

    fn convert_to_full_column_if_const(&self) -> Box<dyn IColumn> {
        Box::new(self.clone())
    }

    fn size(&self) -> usize {
        self.data.size()
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
