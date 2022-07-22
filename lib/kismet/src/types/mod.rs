use num_complex::Complex;

mod span;
mod node;
mod node_iter;

pub use span::*;
pub use node::*;
pub use node_iter::*;

pub type Integer = i32;
pub type Float = f32;
pub type Imaginary = Complex<f32>;