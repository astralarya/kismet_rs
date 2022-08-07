use std::fmt;

use super::{Collection, Primitive};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Primitive(Primitive),
    Collection(Collection),
    Error,
}

impl Default for Value {
    fn default() -> Self {
        Self::Primitive(Primitive::default())
    }
}

impl From<Primitive> for Value {
    fn from(val: Primitive) -> Self {
        Value::Primitive(val)
    }
}

impl TryFrom<Value> for Primitive {
    type Error = ();

    fn try_from(val: Value) -> Result<Self, Self::Error> {
        match val {
            Value::Primitive(x) => Ok(x),
            Value::Collection(_) => Err(()),
            Value::Error => Err(()),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Self::Primitive(x) => write!(f, "{}", x),
            Self::Collection(x) => write!(f, "{}", x),
            Self::Error => write!(f, "error"),
        }
    }
}
