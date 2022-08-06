use std::fmt;

use crate::types::{fmt_float, Float, Integer};

#[derive(Clone, Default, Debug, PartialEq)]
pub enum Primitive {
    Boolean(bool),
    Integer(Integer),
    Float(Float),
    String(String),
    Null,
    #[default]
    Undefined,
}

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Self::Boolean(x) => write!(f, "{}", x),
            Self::Integer(x) => write!(f, "{}", x),
            Self::Float(x) => fmt_float(f, x),
            Self::String(x) => write!(f, "{}", x),
            Self::Null => write!(f, "null"),
            Self::Undefined => write!(f, "undefined"),
        }
    }
}
