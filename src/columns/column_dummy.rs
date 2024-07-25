use super::IColumn;
use super::{StringRef,Field};
use core::fmt::Debug;

#[derive(Debug)]
pub struct ColumnDummy {
    s: usize,
}

impl ColumnDummy {
    pub fn new(size: usize) -> Self {
        ColumnDummy { s: size }
    }

    pub fn add_size(&mut self, delta: usize) {
        self.s += delta;
    }

    pub fn is_dummy(&self) -> bool {
        true
    }
}

impl IColumn for ColumnDummy {
    fn clone_box(&self) -> Box<dyn IColumn> {
        Box::new(self.clone())
    }

    fn get_family_name(&self) -> &str {
        "Dummy"
    }

    fn get_data_type(&self) -> &str {
        "Dummy"
    }

    fn convert_to_full_column_if_const(&self) -> Box<dyn IColumn> {
        Box::new(self.clone())
    }

    fn size(&self) -> usize {
        self.s
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

    fn get_u64(&self, _n: usize) -> u64 {
        panic!("Method get_u64 is not supported for {}", self.get_name());
    }

    fn get_f64(&self, _n: usize) -> f64 {
        panic!("Method get_f64 is not supported for {}", self.get_name());
    }

    fn get_f32(&self, _n: usize) -> f32 {
        panic!("Method get_f32 is not supported for {}", self.get_name());
    }
}

impl Clone for ColumnDummy {
    fn clone(&self) -> ColumnDummy {
        ColumnDummy { s: self.s }
    }
}