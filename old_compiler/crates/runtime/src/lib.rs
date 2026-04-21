pub mod array;
pub mod console;
pub mod math;
pub mod string;
pub mod time;
pub mod value;

pub mod prelude {
    pub use crate::console::*;
    pub use crate::string::*;
    pub use crate::value::*;
}
