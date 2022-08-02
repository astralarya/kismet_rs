use std::fmt;

use crate::{
    exec::{Context, Exec, Primitive, Value},
    types::{Integer, Node, UInteger},
};

use super::{Atom, Expr, Range};

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    And(Node<Expr>, Node<Expr>),
    Or(Node<Expr>, Node<Expr>),
    Not(Node<Expr>),
    CompareBound {
        l_val: Node<Expr>,
        l_op: Node<OpEqs>,
        val: Node<Expr>,
        r_op: Node<OpEqs>,
        r_val: Node<Expr>,
    },
    Compare(Node<Expr>, Node<OpEqs>, Node<Expr>),
    Range(Range),
    Arith(Node<Expr>, Node<OpArith>, Node<Expr>),
    Unary(Node<OpArith>, Node<Expr>),
    Coefficient(Node<Atom>, Node<Expr>),
    Die(Node<Atom>),
}

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

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::And(lhs, rhs) => write!(f, "{} and {}", lhs, rhs),
            Self::Or(lhs, rhs) => write!(f, "{} or {}", lhs, rhs),
            Self::Not(val) => write!(f, "not {}", val),
            Self::CompareBound {
                l_val,
                l_op,
                val,
                r_op,
                r_val,
            } => write!(f, "{} {} {} {} {}", l_val, l_op, val, r_op, r_val),
            Self::Compare(lhs, op, rhs) => write!(f, "{} {} {}", lhs, op, rhs),
            Self::Range(val) => write!(f, "{}", val),
            Self::Arith(lhs, op, rhs) => {
                write!(
                    f,
                    "{}{}{}{}{}",
                    lhs,
                    op.data.space(),
                    op,
                    op.data.space(),
                    rhs
                )
            }
            Self::Unary(lhs, val) => write!(f, "{}{}{}", lhs, lhs.data.space(), val),
            Self::Coefficient(lhs, rhs) => write!(f, "{}{}", lhs, rhs),
            Self::Die(val) => match *val.data {
                Atom::Id(_) => write!(f, "d({})", val),
                _ => write!(f, "d{}", val),
            },
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

impl Exec<Context> for Op {
    type Result = Value;

    fn exec(&self, c: Context) -> (Context, Self::Result) {
        match self {
            Op::And(_, _) => todo!(),
            Op::Or(_, _) => todo!(),
            Op::Not(_) => todo!(),
            Op::CompareBound {
                l_val,
                l_op,
                val,
                r_op,
                r_val,
            } => todo!(),
            Op::Compare(_, _, _) => todo!(),
            Op::Range(_) => todo!(),
            Op::Arith(lhs, op, rhs) => {
                let (c, lhs) = lhs.exec(c);
                let (c, rhs) = rhs.exec(c);
                match (lhs, rhs) {
                    (
                        Value::Primitive(Primitive::Integer(lhs)),
                        Value::Primitive(Primitive::Integer(rhs)),
                    ) => match *op.data {
                        OpArith::ADD => (
                            c,
                            match Integer::checked_add(lhs, rhs) {
                                Some(x) => Value::Primitive(Primitive::Integer(x)),
                                None => Value::Error,
                            },
                        ),
                        OpArith::SUB => (
                            c,
                            match Integer::checked_sub(lhs, rhs) {
                                Some(x) => Value::Primitive(Primitive::Integer(x)),
                                None => Value::Error,
                            },
                        ),
                        OpArith::MUL => (
                            c,
                            match Integer::checked_mul(lhs, rhs) {
                                Some(x) => Value::Primitive(Primitive::Integer(x)),
                                None => Value::Error,
                            },
                        ),
                        OpArith::DIV => (
                            c,
                            match Integer::checked_div(lhs, rhs) {
                                Some(x) => Value::Primitive(Primitive::Integer(x)),
                                None => Value::Error,
                            },
                        ),
                        OpArith::MOD => (c, Value::Primitive(Primitive::Integer(lhs % rhs))),
                        OpArith::POW => {
                            let rhs = match UInteger::try_from(rhs) {
                                Ok(rhs) => rhs,
                                Err(_) => return (c, Value::Error),
                            };
                            (
                                c,
                                match Integer::checked_pow(lhs, rhs) {
                                    Some(x) => Value::Primitive(Primitive::Integer(x)),
                                    None => Value::Error,
                                },
                            )
                        }
                    },
                    _ => (c, Value::Error),
                }
            }
            Op::Unary(_, _) => todo!(),
            Op::Coefficient(_, _) => todo!(),
            Op::Die(_) => todo!(),
        }
    }
}
