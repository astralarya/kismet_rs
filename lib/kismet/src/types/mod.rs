use num_complex::Complex;

mod node;
mod span;

pub use node::*;
pub use span::*;

pub type Integer = i32;
pub type Float = f32;
pub type Imaginary = Complex<f32>;
