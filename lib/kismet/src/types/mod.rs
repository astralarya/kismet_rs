use num_complex::Complex;

mod span;
mod node;

pub use span::*;
pub use node::*;

pub type Integer = i32;
pub type Float = f32;
pub type Imaginary = Complex<f32>;