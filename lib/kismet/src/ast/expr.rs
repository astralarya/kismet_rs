use std::fmt;

use crate::types::Node;

use super::{
    Atom, Branch, DictItem, ListItem, OpArith, OpEqs, Primary, Range, Target, TargetDictItem,
    TargetListItem,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Stmts(Vec<Node<Expr>>),
    Assign(Node<Target>, Node<Expr>),
    Branch(Branch),
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
    Primary(Primary),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stmts(val) => write!(f, "{}", Node::join(&val, "\n")),
            Self::Assign(lhs, rhs) => write!(f, "{} := {}", lhs, rhs),
            Self::Branch(val) => write!(f, "{}", val),
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
            Self::Primary(val) => write!(f, "{}", val),
        }
    }
}

impl TryFrom<&Node<Expr>> for Node<String> {
    type Error = ();

    fn try_from(val: &Node<Expr>) -> Result<Self, Self::Error> {
        match &*val.data {
            Expr::Primary(Primary::Atom(Atom::Id(x))) => Ok(Node::new(val.span, x.clone())),
            _ => Err(()),
        }
    }
}

impl TryFrom<Node<Expr>> for Node<Target> {
    type Error = ();

    fn try_from(val: Node<Expr>) -> Result<Self, Self::Error> {
        match *val.data {
            Expr::Primary(Primary::Atom(x)) => Node::<Target>::try_from(Node::new(val.span, x)),
            _ => Err(()),
        }
    }
}

impl TryFrom<Node<Atom>> for Node<Target> {
    type Error = ();

    fn try_from(val: Node<Atom>) -> Result<Self, Self::Error> {
        fn list_item(val: Node<ListItem>) -> Result<Node<TargetListItem>, ()> {
            let (val, node): (Node<Expr>, &dyn Fn(Node<Target>) -> Node<TargetListItem>) =
                match *val.data {
                    ListItem::Expr(y) => (Node::new(val.span, y), &|x: Node<Target>| {
                        Node::new(x.span, TargetListItem::Target(*x.data))
                    }),
                    ListItem::Spread(x) => (x.clone(), &|x: Node<Target>| {
                        Node::new(x.span, TargetListItem::Spread(x))
                    }),
                };
            let val = Node::<Target>::try_from(val)?;
            Ok(node(val))
        }

        match *val.data {
            Atom::Id(x) => Ok(Node::new(val.span, Target::Id(x.clone()))),
            Atom::Paren(x) => {
                let x = list_item(x)?;
                Ok(Node::new(val.span, Target::TargetTuple(vec![x])))
            }
            Atom::Tuple(x) => {
                let x_len = x.len();
                let y = x
                    .into_iter()
                    .filter_map(|x| list_item(x).ok())
                    .collect::<Vec<_>>();
                if x_len != y.len() {
                    return Err(());
                }
                Ok(Node::new(val.span, Target::TargetTuple(y)))
            }
            Atom::ListDisplay(x) => {
                let x_len = x.len();
                let y = x
                    .into_iter()
                    .filter_map(|x| list_item(x).ok())
                    .collect::<Vec<_>>();
                if x_len != y.len() {
                    return Err(());
                }
                Ok(Node::new(val.span, Target::TargetList(y)))
            }
            Atom::DictDisplay(x) => {
                let x_len = x.len();
                let y = x
                    .into_iter()
                    .filter_map(|x| match *x.data {
                        DictItem::Shorthand(y) => {
                            Some(Node::new(x.span, TargetDictItem::Target(y.clone())))
                        }
                        DictItem::Spread(x) => match Node::<Target>::try_from(x) {
                            Ok(x) => Some(Node::new(x.span, TargetDictItem::Spread(x))),
                            Err(_) => None,
                        },
                        DictItem::KeyVal { key, val } => match Node::<Target>::try_from(val) {
                            Ok(val) => Some(Node::new(
                                x.span,
                                TargetDictItem::Pair {
                                    key: key.clone(),
                                    val,
                                },
                            )),
                            Err(_) => None,
                        },
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                if x_len != y.len() {
                    return Err(());
                }
                Ok(Node::new(val.span, Target::TargetDict(y)))
            }
            _ => Err(()),
        }
    }
}
