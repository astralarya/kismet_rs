use std::fmt;

use crate::{
    exec::{Context, Exec1, Primitive, Value},
    types::{Float, Integer, Node, UInteger},
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OpEqs {
    EQ,
    NE,
    LT,
    LE,
    GT,
    GE,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OpRange {
    RANGE,
    RANGEI,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OpArith {
    ADD,
    SUB,
    MUL,
    DIV,
    IDIV,
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
            Self::Unary(lhs, val) => write!(f, "{}{}", lhs, val),
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
            Self::IDIV => write!(f, "/%"),
            Self::MOD => write!(f, "%"),
            Self::POW => write!(f, "^"),
        }
    }
}

impl Exec1<Context> for Op {
    type Result = Value;

    fn exec(&self, c: Context) -> (Context, Self::Result) {
        fn arith_float(lhs: Float, op: &OpArith, rhs: Float) -> Value {
            Value::Primitive(Primitive::Float(match op {
                OpArith::ADD => lhs + rhs,
                OpArith::SUB => lhs - rhs,
                OpArith::MUL => lhs * rhs,
                OpArith::DIV => lhs / rhs,
                OpArith::IDIV => lhs.rem_euclid(rhs),
                OpArith::MOD => lhs % rhs,
                OpArith::POW => lhs.powf(rhs),
            }))
        }

        fn arith_int(lhs: Integer, op: &OpArith, rhs: Integer) -> Value {
            match match op {
                OpArith::ADD => lhs.checked_add(rhs),
                OpArith::SUB => lhs.checked_sub(rhs),
                OpArith::MUL => lhs.checked_mul(rhs),
                OpArith::DIV => return arith_float(lhs as Float, op, rhs as Float),
                OpArith::IDIV => lhs.checked_div(rhs),
                OpArith::MOD => lhs.checked_rem(rhs),
                OpArith::POW => match UInteger::try_from(rhs) {
                    Ok(rhs) => lhs.checked_pow(rhs),
                    Err(_) => None,
                },
            } {
                Some(x) => Value::Primitive(Primitive::Integer(x)),
                None => arith_float(lhs as Float, op, rhs as Float),
            }
        }

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
                    ) => (c, arith_int(lhs, op, rhs)),
                    (
                        Value::Primitive(Primitive::Float(lhs)),
                        Value::Primitive(Primitive::Float(rhs)),
                    ) => (c, arith_float(lhs, op, rhs)),
                    (
                        Value::Primitive(Primitive::Integer(lhs)),
                        Value::Primitive(Primitive::Float(rhs)),
                    ) => (
                        c,
                        match (**op, lhs) {
                            (OpArith::POW, 2) => Value::Primitive(Primitive::Float(rhs.exp2())),
                            _ => arith_float(lhs as Float, op, rhs),
                        },
                    ),
                    (
                        Value::Primitive(Primitive::Float(lhs)),
                        Value::Primitive(Primitive::Integer(rhs)),
                    ) => (
                        c,
                        match **op {
                            OpArith::POW => Value::Primitive(Primitive::Float(lhs.powi(rhs))),
                            _ => arith_float(lhs, op, rhs as Float),
                        },
                    ),
                    _ => (c, Value::Error),
                }
            }
            Op::Unary(op, rhs) => {
                let (c, rhs) = rhs.exec(c);
                match rhs {
                    Value::Primitive(Primitive::Integer(val)) => (
                        c,
                        match **op {
                            OpArith::ADD => rhs,
                            OpArith::SUB => match val.checked_neg() {
                                Some(val) => Value::Primitive(Primitive::Integer(val)),
                                None => Value::Primitive(Primitive::Float((val as Float) * -1.)),
                            },
                            _ => Value::Error,
                        },
                    ),
                    Value::Primitive(Primitive::Float(val)) => (
                        c,
                        match **op {
                            OpArith::ADD => rhs,
                            OpArith::SUB => Value::Primitive(Primitive::Float(val * -1.)),
                            _ => Value::Error,
                        },
                    ),
                    _ => (c, Value::Error),
                }
            }
            Op::Coefficient(_, _) => todo!(),
            Op::Die(_) => todo!(),
        }
    }
}
