use super::IColumn;
use super::{StringRef,Field};
use core::fmt::Debug;
use core::cmp::PartialEq;


#[derive(Debug)]
pub struct ColumnSparse<T> {
    data: Vec<T>,
    offsets: Vec<usize>,
    default_value: T,
}

impl<T> ColumnSparse<T>
where
    T: Copy + Debug + Default + 'static + PartialEq,
{
    pub fn new(default_value: T) -> Self {
        ColumnSparse {
            data: Vec::new(),
            offsets: Vec::new(),
            default_value,
        }
    }

    pub fn with_capacity(default_value: T, capacity: usize) -> Self {
        ColumnSparse {
            data: Vec::with_capacity(capacity),
            offsets: Vec::with_capacity(capacity),
            default_value,
        }
    }

    pub fn insert(&mut self, value: T, offset: usize) {
        if value != self.default_value {
            self.data.push(value);
            self.offsets.push(offset);
        }
    }
}

impl<T> IColumn for ColumnSparse<T>
where
    T: Copy + Debug + Default + 'static + PartialEq,
{
    fn clone_box(&self) -> Box<dyn IColumn> {
        Box::new(ColumnSparse {
            data: self.data.clone(),
            offsets: self.offsets.clone(),
            default_value: self.default_value,
        })
    }

    fn get_family_name(&self) -> &str {
        "ColumnSparse"
    }

    fn get_data_type(&self) -> &str {
        std::any::type_name::<T>()
    }

    fn convert_to_full_column_if_const(&self) -> Box<dyn IColumn> {
        Box::new(ColumnSparse {
            data: self.data.clone(),
            offsets: self.offsets.clone(),
            default_value: self.default_value,
        })
    }

    fn size(&self) -> usize {
        self.offsets.len()
    }

    fn get_field(&self, n: usize) -> Field {
        // Implementation for retrieving a field
        unimplemented!("get_field not implemented");
        Field::Int(0)
        //Field
    }

    fn get(&self, n: usize, res: &mut Field) {
        // Implementation for setting a field
    }

    fn get_data_at(&self, n: usize) -> StringRef {
        let start = if n == 0 { 0 } else { self.offsets[n - 1] };
        let end = self.offsets[n];
        let bytes = unsafe {
            std::slice::from_raw_parts(
                &self.data[n] as *const T as *const u8,
                std::mem::size_of::<T>(),
            )
        };
        StringRef::new(bytes)
    }
}


#[test]
fn test_sparse() {
    let mut col = ColumnSparse::new(0);
    col.insert(10, 0);
    col.insert(20, 5);
    col.insert(30, 10);

    let col_box: Box<dyn IColumn> = Box::new(col);

    println!("Column family name: {}", col_box.get_family_name());
    println!("Column size: {}", col_box.size());
    let data_ref = col_box.get_data_at(1);
    println!("Data at index 1: {:?}", data_ref.data);
}