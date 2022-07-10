use std::fmt;

pub enum Expr {
    Number(i32),
    Paren(Box<Expr>),
    Op(Box<Expr>, Op, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::Paren(e) => write!(f, "({})", e),
            Expr::Op(l, o, r) => match o {
                Op::Die => write!(f, "{}{}{}", l, o, r),
                o => write!(f, "{} {} {}", l, o, r),
            },
        }
    }
}

pub enum Op {
    Die,
    Mod,
    Mul,
    Div,
    Add,
    Sub,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::Die => write!(f, "d"),
            Op::Mod => write!(f, "%"),
            Op::Mul => write!(f, "*"),
            Op::Div => write!(f, "/"),
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
        }
    }
}
