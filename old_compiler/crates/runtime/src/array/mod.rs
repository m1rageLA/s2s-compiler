pub mod filter;
pub mod index;
pub mod join;
pub mod length;
pub mod map;
pub mod push;

pub use filter::filter;
pub use index::{index, index_number};
pub use join::join;
pub use length::{length, length_number};
pub use map::map;
pub use push::{push, push_number};

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

impl ArrayLike for Vec<f64> {
    fn len(&self) -> usize {
        self.len()
    }

    fn get(&self, index: usize) -> Option<Value> {
        self.as_slice().get(index).copied().map(Value::Number)
    }
}

impl ArrayLike for Vec<String> {
    fn len(&self) -> usize {
        self.len()
    }

    fn get(&self, index: usize) -> Option<Value> {
        self.as_slice().get(index).cloned().map(Value::String)
    }
}

impl ArrayLike for Vec<bool> {
    fn len(&self) -> usize {
        self.len()
    }

    fn get(&self, index: usize) -> Option<Value> {
        self.as_slice().get(index).copied().map(Value::Bool)
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
