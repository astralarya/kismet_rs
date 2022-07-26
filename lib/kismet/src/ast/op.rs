use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum OpEqs {
    EQ,
    NE,
    LT,
    LE,
    GT,
    GE,
}

impl fmt::Display for OpEqs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpEqs::EQ => write!(f, "=="),
            OpEqs::NE => write!(f, "!="),
            OpEqs::LT => write!(f, "<"),
            OpEqs::LE => write!(f, "<="),
            OpEqs::GT => write!(f, ">"),
            OpEqs::GE => write!(f, ">="),
        }
    }
}
