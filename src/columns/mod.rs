pub(crate) mod column_aggregate_function;
pub(crate) mod column_array;
pub(crate) mod column_const;
pub(crate) mod column_decimal;
pub(crate) mod column_dummy;
pub(crate) mod column_fixed_string;
pub(crate) mod column_nullable;
pub(crate) mod column_sparse;
pub(crate) mod column_string;
pub(crate) mod column_vector;

use std::any::Any;
use std::fmt::Debug;

use crate::core::field::Field;

pub struct StringRef<'a> {
    data: &'a [u8],
}

impl<'a> StringRef<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        StringRef { data }
    }
	pub fn get_data(&'a self) -> &'a [u8]{
		self.data
	}
}

pub trait IColumn: Debug + Any {
    /// Clones the column. This is an internal method.
    fn clone_box(&self) -> Box<dyn IColumn>;

    /// Returns the name of the column.
    fn get_name(&self) -> String {
        String::from(self.get_family_name())
    }

    /// Returns the family name of the column kind.
    fn get_family_name(&self) -> &str;

    /// Returns the underlying data type of the column.
    fn get_data_type(&self) -> &str;

    /// Converts a constant column to a full column if applicable.
    fn convert_to_full_column_if_const(&self) -> Box<dyn IColumn>;

    /// Returns the number of values in the column.
    fn size(&self) -> usize;

    /// Checks if the column is empty.
    fn empty(&self) -> bool {
        self.size() == 0
    }

    /// Returns the value of the n-th element in the universal Field representation.
    fn get_field(&self, n: usize) -> Field;

    /// Retrieves the n-th element and stores it in the provided Field reference.
    fn get(&self, n: usize, res: &mut Field);

    /// Returns a reference to the memory chunk containing the n-th element.
    fn get_data_at(&self, n: usize) -> StringRef;

    /// Returns the n-th element cast to u64 for columns storing integers or floating-point numbers.
    fn get_u64(&self, n: usize) -> u64 {
        panic!("Method get_u64 is not supported for {}", self.get_name());
    }

    /// Returns the n-th element cast to f64.
    fn get_f64(&self, n: usize) -> f64 {
        panic!("Method get_f64 is not supported for {}", self.get_name());
    }

    /// Returns the n-th element cast to f32.
    fn get_f32(&self, n: usize) -> f32 {
        panic!("Method get_f32 is not supported for {}", self.get_name());
    }
}

impl dyn IColumn {
    fn downcast_ref<T: IColumn>(&self) -> Option<&T> {
        if !self.get_family_name().is_empty(){
            unsafe { Some(&*(self as *const dyn IColumn as *const T)) }
        } else {
            None
        }
    }
}


impl Clone for Box<dyn IColumn> {
    fn clone(&self) -> Box<dyn IColumn> {
        self.clone_box()
    }
}


