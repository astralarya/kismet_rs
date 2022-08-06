use std::fmt;

use crate::types::{Float, Integer};

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
            Self::Float(x) => {
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
            Self::String(x) => write!(f, "{}", x),
            Self::Null => write!(f, "null"),
            Self::Undefined => write!(f, "undefined"),
        }
    }
}
