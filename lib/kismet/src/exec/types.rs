use std::{collections::HashMap, fmt};

use crate::{
    ast::Id,
    types::{Float, Integer},
};

#[derive(Clone, Debug, PartialEq)]
pub enum Primitive {
    Boolean(bool),
    Integer(Integer),
    Float(Float),
    String(String),
    Null,
    Undefined,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Collection {
    Tuple(Vec<Value>),
    List(Vec<Value>),
    Dict(HashMap<Id, Value>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Primitive(Primitive),
    Collection(Collection),
    Error,
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

impl From<Primitive> for Value {
    fn from(val: Primitive) -> Self {
        Value::Primitive(val)
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

impl fmt::Display for Collection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Self::Tuple(x) => write!(
                f,
                "({}{})",
                x.into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
                if x.len() == 1 { "," } else { "" },
            ),
            Self::List(x) => write!(
                f,
                "[{}]",
                x.into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::Dict(x) => write!(
                f,
                "{{{}}}",
                x.iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Self::Boolean(x) => write!(f, "{}", x),
            Self::Integer(x) => write!(f, "{}", x),
            Self::Float(x) => write!(f, "{}", x),
            Self::String(x) => write!(f, "{}", x),
            Self::Null => write!(f, "null"),
            Self::Undefined => write!(f, "undefined"),
        }
    }
}
