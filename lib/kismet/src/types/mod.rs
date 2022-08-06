use std::fmt;

use num_complex::Complex;

mod list;
mod node;
mod span;

pub use list::*;
pub use node::*;
pub use span::*;

pub type Integer = i32;
pub type UInteger = u32;
pub type Float = f32;
pub type Imaginary = Complex<f32>;

pub fn fmt_float(f: &mut fmt::Formatter<'_>, x: &Float) -> fmt::Result {
    if x.abs() >= 1e16 || x.abs() <= 1e-4 {
        write!(f, "{:e}", x)
    } else {
        let s = x.to_string();
        let mut s = s.split(".");
        let fract = s.nth(1);
        match fract {
            Some(fract) => write!(f, "{}.{}", x.trunc(), fract),
            None => write!(f, "{}.", x),
        }
    }
}
