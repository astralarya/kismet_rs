use std::fmt;

use crate::types::Node;

use super::{Atom, DictItem, Expr, Id, ListItem, Primary};

#[derive(Clone, Debug, PartialEq)]
pub struct Target(pub TargetKind<Target>);

#[derive(Clone, Debug, PartialEq)]
pub enum TargetExpr {
    Target(TargetKind<TargetExpr>),
    TargetExpr(Node<TargetKind<TargetExpr>>, Node<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Match {
    Target(TargetKind<Match>),
    Literal(Atom),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TargetKind<T> {
    Id(Id),
    TargetTuple(Vec<Node<TargetListItem<T>>>),
    TargetList(Vec<Node<TargetListItem<T>>>),
    TargetDict(Vec<Node<TargetDictItem<T>>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TargetListItem<T> {
    Spread(Node<T>),
    Target(T),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TargetDictItem<T> {
    KeyVal { key: Node<Id>, val: Node<T> },
    Spread(Node<T>),
    Target(T),
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for TargetExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Target(tar) => write!(f, "{}", tar),
            Self::TargetExpr(tar, val) => write!(f, "{} = {}", tar, val),
        }
    }
}

impl fmt::Display for Match {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Target(tar) => write!(f, "{}", tar),
            Self::Literal(val) => write!(f, "{}", val),
        }
    }
}

impl From<TargetKind<Self>> for Target {
    fn from(val: TargetKind<Self>) -> Self {
        Target(val)
    }
}

impl From<TargetKind<Self>> for Match {
    fn from(val: TargetKind<Self>) -> Self {
        Self::Target(val)
    }
}

impl<T> From<Id> for TargetKind<T> {
    fn from(val: Id) -> Self {
        Self::Id(val)
    }
}

impl From<TargetKind<Self>> for TargetExpr {
    fn from(val: TargetKind<Self>) -> Self {
        Self::Target(val)
    }
}

impl From<Target> for TargetExpr {
    fn from(val: Target) -> Self {
        TargetExpr::Target(match val.0 {
            TargetKind::Id(x) => TargetKind::Id(x),
            TargetKind::TargetTuple(x) => {
                let x = x
                    .into_iter()
                    .map(|x| Node::new(x.span, TargetListItem::<TargetExpr>::convert(*x.data)))
                    .collect::<Vec<_>>();
                TargetKind::TargetTuple(x)
            }
            TargetKind::TargetList(x) => {
                let x = x
                    .into_iter()
                    .map(|x| Node::new(x.span, TargetListItem::<TargetExpr>::convert(*x.data)))
                    .collect::<Vec<_>>();
                TargetKind::TargetList(x)
            }
            TargetKind::TargetDict(x) => {
                let x = x
                    .into_iter()
                    .map(|x| Node::new(x.span, TargetDictItem::<TargetExpr>::convert(*x.data)))
                    .collect::<Vec<_>>();
                TargetKind::TargetDict(x)
            }
        })
    }
}

impl<T> TargetListItem<T> {
    pub fn convert<U>(val: TargetListItem<U>) -> Self
    where
        T: From<U>,
    {
        match val {
            TargetListItem::Spread(x) => Self::Spread(Node::new(x.span, T::from(*x.data))),
            TargetListItem::Target(x) => Self::Target(T::from(x)),
        }
    }
}

impl<T> TargetDictItem<T> {
    pub fn convert<U>(val: TargetDictItem<U>) -> Self
    where
        T: From<U>,
    {
        match val {
            TargetDictItem::KeyVal { key, val } => Self::KeyVal {
                key,
                val: Node::new(val.span, T::from(*val.data)),
            },
            TargetDictItem::Spread(x) => Self::Spread(Node::new(x.span, T::from(*x.data))),
            TargetDictItem::Target(x) => Self::Target(T::from(x)),
        }
    }
}

impl<T> fmt::Display for TargetKind<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id(val) => write!(f, "{}", val),
            Self::TargetTuple(val) => write!(f, "({})", Node::join1(&val, ", ", ",")),
            Self::TargetList(val) => write!(f, "[{}]", Node::join(&val, ", ")),
            Self::TargetDict(val) => write!(f, "{{{}}}", Node::join(&val, ", ")),
        }
    }
}

impl<T> fmt::Display for TargetListItem<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Target(val) => write!(f, "{}", val),
            Self::Spread(val) => write!(f, "...{}", val),
        }
    }
}

impl<T> fmt::Display for TargetDictItem<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Target(val) => write!(f, "{}", val),
            Self::Spread(val) => write!(f, "...{}", val),
            Self::KeyVal { key, val } => write!(f, "{}: {}", key, val),
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

impl TryFrom<Node<Expr>> for Node<TargetKind<TargetExpr>> {
    type Error = ();

    fn try_from(val: Node<Expr>) -> Result<Self, Self::Error> {
        match Node::<Target>::try_from(val) {
            Ok(x) => {
                let x = Node::<TargetExpr>::convert_from(x);
                match *x.data {
                    TargetExpr::Target(y) => Ok(Node::new(x.span, y)),
                    _ => Err(()),
                }
            }
            Err(_) => Err(()),
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
            Atom::Id(x) => Ok(Node::new(val.span, Target(TargetKind::Id(Id(x.clone()))))),
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
                                            TargetDictItem::KeyVal { key, val },
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

impl<T> TryFrom<&Node<ListItem>> for Node<TargetListItem<T>>
where
    T: From<TargetKind<T>>,
{
    type Error = ();

    fn try_from(x: &Node<ListItem>) -> Result<Self, Self::Error> {
        let span = x.span;
        match &*x.data {
            ListItem::Expr(x) => match Node::<Id>::try_from(&Node::new(span, x)) {
                Ok(x) => Ok(Node::new(
                    span,
                    TargetListItem::Target(T::from(TargetKind::Id(*x.data))),
                )),
                Err(_) => Err(()),
            },
            ListItem::Spread(x) => match Node::<Id>::try_from(x) {
                Ok(x) => Ok(Node::new(
                    span,
                    TargetListItem::Spread(Node::convert(|x| T::from(TargetKind::Id(x)), x)),
                )),
                Err(_) => Err(()),
            },
        }
    }
}

impl<T> TryFrom<&Node<DictItem>> for Node<TargetDictItem<T>>
where
    T: From<TargetKind<T>>,
    T: From<Target>,
{
    type Error = ();

    fn try_from(x: &Node<DictItem>) -> Result<Self, Self::Error> {
        match &*x.data {
            DictItem::Shorthand(y) => Ok(Node::new(
                x.span,
                TargetDictItem::Target(T::from(TargetKind::Id(y.clone()))),
            )),
            DictItem::Spread(x) => match Node::<Target>::try_from(x.clone()) {
                Ok(x) => Ok(Node::new(
                    x.span,
                    TargetDictItem::Spread(Node::<T>::convert_from(x)),
                )),
                Err(_) => Err(()),
            },
            DictItem::KeyVal { key, val } => match Node::<Target>::try_from(val.clone()) {
                Ok(val) => Ok(Node::new(
                    x.span,
                    TargetDictItem::KeyVal {
                        key: key.clone(),
                        val: Node::<T>::convert_from(val),
                    },
                )),
                Err(_) => Err(()),
            },
            _ => Err(()),
        }
    }
}
