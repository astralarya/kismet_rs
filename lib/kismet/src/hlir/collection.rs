use std::{collections::HashMap, fmt};

use crate::ast::Id;

use super::Value;

#[derive(Clone, Debug, PartialEq)]
pub enum Collection {
    Tuple(Vec<Value>),
    List(Vec<Value>),
    Dict(HashMap<Id, Value>),
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
