use super::IColumn;
use super::{StringRef,Field};
use core::fmt::Debug;

#[derive(Debug, Clone)]
pub struct ColumnDecimal {
    data: Vec<i128>, // Assuming 128-bit decimal representation
    precision: u8,
    scale: u8,
}

impl ColumnDecimal {
    pub fn new(data: Vec<i128>, precision: u8, scale: u8) -> Self {
        ColumnDecimal { data, precision, scale }
    }
}

impl IColumn for ColumnDecimal {
    fn clone_box(&self) -> Box<dyn IColumn> {
        Box::new(self.clone())
    }

    fn get_family_name(&self) -> &str {
        "Decimal"
    }

    fn get_data_type(&self) -> &str {
        "Decimal"
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
