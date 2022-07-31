use num_complex::Complex;

mod list;
mod node;
mod span;

pub use list::*;
pub use node::*;
pub use span::*;

pub type Integer = i32;
pub type Float = f32;
pub type Imaginary = Complex<f32>;
