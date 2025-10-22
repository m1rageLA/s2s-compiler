pub mod index;
pub mod length;
pub mod push;

pub use index::{index, index_number};
pub use length::{length, length_number};
pub use push::push;

use crate::value::Value;
use std::any::Any;

pub trait ArrayLike {
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> Option<Value>;
}

impl ArrayLike for Vec<Value> {
    fn len(&self) -> usize {
        self.len()
    }

    fn get(&self, index: usize) -> Option<Value> {
        self.as_slice().get(index).cloned()
    }
}

impl ArrayLike for Box<dyn Any> {
    fn len(&self) -> usize {
        cast_to_values(self.as_ref()).len()
    }

    fn get(&self, index: usize) -> Option<Value> {
        cast_to_values(self.as_ref()).as_slice().get(index).cloned()
    }
}

fn cast_to_values(any: &dyn Any) -> &Vec<Value> {
    any.downcast_ref::<Vec<Value>>()
        .expect("expected Vec<Value> for array operations")
}
