pub mod console;
pub mod math;
pub mod time;
pub mod value;
pub mod array;

pub mod prelude {
    pub use crate::console::*;
    pub use crate::value::*;
}
