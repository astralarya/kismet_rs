use std::fmt;

use crate::{ast::TargetKind, types::Node};

use super::{
    ArgsDef, Atom, Branch, DictItem, ExprBlock, ListItem, Loop, Op, Primary, Target,
    TargetDictItem, TargetListItem,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Assign(Node<Target>, Node<Expr>),
    Function {
        args: Node<ArgsDef>,
        block: Node<ExprBlock>,
    },
    Branch(Branch),
    Loop(Loop),
    Op(Op),
    Primary(Primary),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assign(lhs, rhs) => write!(f, "{} := {}", lhs, rhs),
            Self::Branch(val) => write!(f, "{}", val),
            Self::Loop(val) => write!(f, "{}", val),
            Self::Function { args, block } => {
                write!(f, "({}) => {}", args, block)
            }
            Self::Op(val) => write!(f, "{}", val),
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
        fn list_item(val: Node<ListItem>) -> Result<Node<TargetListItem<Target>>, ()> {
            let (val, node): (
                Node<Expr>,
                &dyn Fn(Node<Target>) -> Node<TargetListItem<Target>>,
            ) = match *val.data {
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
            Atom::Id(x) => Ok(Node::new(val.span, Target(TargetKind::Id(x.clone())))),
            Atom::Paren(x) => {
                let x = list_item(x)?;
                Ok(Node::new(
                    val.span,
                    Target(TargetKind::TargetTuple(vec![x])),
                ))
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
                Ok(Node::new(val.span, Target(TargetKind::TargetTuple(y))))
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
                Ok(Node::new(val.span, Target(TargetKind::TargetList(y))))
            }
            Atom::DictDisplay(x) => {
                let x_len = x.len();
                let y = x
                    .into_iter()
                    .filter_map(
                        |x: Node<DictItem>| -> Option<Node<TargetDictItem<Target>>> {
                            match *x.data {
                                DictItem::Shorthand(y) => Some(Node::new(
                                    x.span,
                                    TargetDictItem::Target(Target(TargetKind::Id(y))),
                                )),
                                DictItem::Spread(x) => match Node::<Target>::try_from(x) {
                                    Ok(x) => Some(Node::new(x.span, TargetDictItem::Spread(x))),
                                    Err(_) => None,
                                },
                                DictItem::KeyVal { key, val } => {
                                    match Node::<Target>::try_from(val) {
                                        Ok(val) => Some(Node::new(
                                            x.span,
                                            TargetDictItem::Pair {
                                                key: key.clone(),
                                                val,
                                            },
                                        )),
                                        Err(_) => None,
                                    }
                                }
                                _ => None,
                            }
                        },
                    )
                    .collect::<Vec<_>>();
                if x_len != y.len() {
                    return Err(());
                }
                Ok(Node::new(val.span, Target(TargetKind::TargetDict(y))))
            }
            _ => Err(()),
        }
    }
}
