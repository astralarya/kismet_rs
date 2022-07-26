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

#[derive(Clone, Debug, PartialEq)]
pub enum OpRange {
    RANGE,
    RANGEI,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OpArith {
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    POW,
}

impl OpArith {
    pub fn space(&self) -> &'static str {
        match self {
            Self::POW | Self::MUL | Self::MOD => "",
            _ => " ",
        }
    }
}

impl fmt::Display for OpEqs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EQ => write!(f, "=="),
            Self::NE => write!(f, "!="),
            Self::LT => write!(f, "<"),
            Self::LE => write!(f, "<="),
            Self::GT => write!(f, ">"),
            Self::GE => write!(f, ">="),
        }
    }
}

impl fmt::Display for OpRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RANGE => write!(f, ".."),
            Self::RANGEI => write!(f, "..="),
        }
    }
}

impl fmt::Display for OpArith {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ADD => write!(f, "+"),
            Self::SUB => write!(f, "-"),
            Self::MUL => write!(f, "*"),
            Self::DIV => write!(f, "/"),
            Self::MOD => write!(f, "%"),
            Self::POW => write!(f, "^"),
        }
    }
}
