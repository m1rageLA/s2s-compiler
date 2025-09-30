pub mod console;
pub mod math;
pub mod time;
pub mod value;

pub mod prelude {
    pub use crate::console::*;
    pub use crate::math::*;
    pub use crate::time::*;
    pub use crate::value::*;
}