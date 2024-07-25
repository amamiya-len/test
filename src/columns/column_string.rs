use super::IColumn;
use super::{StringRef,Field};
use core::fmt::Debug;

#[derive(Debug)]
pub struct ColumnString {
    data: Vec<u8>,
    offsets: Vec<usize>,
}

impl ColumnString {
    pub fn new() -> Self {
        ColumnString {
            data: Vec::new(),
            offsets: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        ColumnString {
            data: Vec::with_capacity(capacity),
            offsets: Vec::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, s: &str) {
        self.data.extend_from_slice(s.as_bytes());
        self.offsets.push(self.data.len());
    }
}

impl IColumn for ColumnString {
    fn clone_box(&self) -> Box<dyn IColumn> {
        Box::new(ColumnString {
            data: self.data.clone(),
            offsets: self.offsets.clone(),
        })
    }

    fn get_family_name(&self) -> &str {
        "ColumnString"
    }

    fn get_data_type(&self) -> &str {
        "String"
    }

    fn convert_to_full_column_if_const(&self) -> Box<dyn IColumn> {
        Box::new(ColumnString {
            data: self.data.clone(),
            offsets: self.offsets.clone(),
        })
    }

    fn size(&self) -> usize {
        self.offsets.len()
    }

    fn get_field(&self, n: usize) -> Field {
        // Implementation for retrieving a field
        unimplemented!("Not implemented");
        Field::Int(0)
    }

    fn get(&self, n: usize, res: &mut Field) {
        // Implementation for setting a field
    }

    fn get_data_at(&self, n: usize) -> StringRef {
        let start = if n == 0 { 0 } else { self.offsets[n - 1] };
        let end = self.offsets[n];
        StringRef::new(&self.data[start..end])
    }
}

#[test]
fn test_column_string() {
    let mut col = ColumnString::new();
    col.insert("hello");
    col.insert("world");

    let col_box: Box<dyn IColumn> = Box::new(col);

    println!("Column family name: {}", col_box.get_family_name());
    println!("Column size: {}", col_box.size());
    let data_ref = col_box.get_data_at(1);
    println!("Data at index 1: {:?}", std::str::from_utf8(data_ref.data));
}