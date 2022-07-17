use std::fmt;

use super::token::Token;
use super::types::{Integer, Span};

#[derive(Debug, PartialEq)]
pub struct Node<Kind> {
    pub span: Span,
    pub kind: Box<Kind>,
}

#[derive(Debug, PartialEq)]
pub enum Expr<'input> {
    Stmts(Vec<Node<Expr<'input>>>),
    Comprehension(Node<Expr<'input>>, Vec<Node<Expr<'input>>>),
    CompFor(
        Node<Expr<'input>>,
        Node<Expr<'input>>,
        Option<Node<Expr<'input>>>,
    ),
    TargetList(Vec<Node<Atom<'input>>>),
    Op(Node<Expr<'input>>, Token<'input>, Node<Expr<'input>>),
    Unary(Token<'input>, Node<Expr<'input>>),
    Atom(Atom<'input>),
}

#[derive(Debug, PartialEq)]
pub enum Atom<'input> {
    Enclosure(Token<'input>, Node<Expr<'input>>, Token<'input>),
    Vector(Vec<Node<Expr<'input>>>),
    Tuple(Vec<Node<Expr<'input>>>),
    Id(&'input str),
    String(String),
    Integer(Integer),
}

impl<T: std::fmt::Display> Node<T> {
    pub fn vec_to_string(nodes: &Vec<Node<T>>, delim: &'static str) -> String {
        nodes
            .iter()
            .map(|node| node.to_string())
            .collect::<Vec<String>>()
            .join(delim)
    }

    pub fn vec_to_span(v: &Vec<Node<T>>) -> Option<Span> {
        Span::reduce(&mut v.iter().map(|x| x.span.clone()))
    }
}

impl<T: std::fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl<'input> Node<Expr<'input>> {
    pub fn stmts((span, v): (Span, Vec<Node<Expr<'input>>>)) -> Node<Expr<'input>> {
        return Node {
            span,
            kind: Box::new(Expr::Stmts(v)),
        };
    }

    pub fn comprehension(
        expr: Node<Expr<'input>>,
        iter: Vec<Node<Expr<'input>>>,
    ) -> Node<Expr<'input>> {
        return Node {
            span: expr.span.clone() + Node::vec_to_span(&iter),
            kind: Box::new(Expr::Comprehension(expr, iter)),
        };
    }

    pub fn comp_for(
        span: Span,
        item: Node<Expr<'input>>,
        iter: Node<Expr<'input>>,
        ifnode: Option<Node<Expr<'input>>>,
    ) -> Node<Expr<'input>> {
        return Node {
            span,
            kind: Box::new(Expr::CompFor(item, iter, ifnode)),
        };
    }

    pub fn target_list((span, vector): (Span, Vec<Node<Atom<'input>>>)) -> Node<Expr<'input>> {
        return Node {
            span,
            kind: Box::new(Expr::TargetList(vector)),
        };
    }

    pub fn op(
        l: Node<Expr<'input>>,
        o: Token<'input>,
        r: Node<Expr<'input>>,
    ) -> Node<Expr<'input>> {
        return Node {
            span: l.span.clone() + r.span.clone(),
            kind: Box::new(Expr::Op(l, o, r)),
        };
    }

    pub fn unary(o: Token<'input>, r: Node<Expr<'input>>) -> Node<Expr<'input>> {
        return Node {
            span: o.span().clone() + r.span.clone(),
            kind: Box::new(Expr::Unary(o, r)),
        };
    }

    pub fn atom(a: Node<Atom<'input>>) -> Node<Expr<'input>> {
        return Node {
            span: a.span,
            kind: Box::new(Expr::Atom(*a.kind)),
        };
    }
}

impl<'input> Node<Atom<'input>> {
    pub fn enclosure(
        l: Token<'input>,
        n: Node<Expr<'input>>,
        r: Token<'input>,
    ) -> Node<Atom<'input>> {
        return Node {
            span: l.span().clone() + r.span().clone(),
            kind: Box::new(Atom::Enclosure(l, n, r)),
        };
    }

    pub fn vector((span, v): (Span, Vec<Node<Expr<'input>>>)) -> Node<Atom<'input>> {
        return Node {
            span,
            kind: Box::new(Atom::Vector(v)),
        };
    }

    pub fn tuple((span, v): (Span, Vec<Node<Expr<'input>>>)) -> Node<Atom<'input>> {
        return Node {
            span,
            kind: Box::new(Atom::Tuple(v)),
        };
    }

    pub fn id((span, string): (Span, &'input str)) -> Node<Atom<'input>> {
        return Node {
            span,
            kind: Box::new(Atom::Id(string)),
        };
    }

    pub fn string((span, string): (Span, String)) -> Node<Atom<'input>> {
        return Node {
            span,
            kind: Box::new(Atom::String(string)),
        };
    }

    pub fn integer((span, value): (Span, Integer)) -> Node<Atom<'input>> {
        return Node {
            span,
            kind: Box::new(Atom::Integer(value)),
        };
    }
}

impl fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Stmts(nodes) => write!(f, "{}", Node::vec_to_string(&nodes, "\n")),
            Expr::Comprehension(e, i) => write!(f, "{} {}", e, Node::vec_to_string(&i, " ")),
            Expr::CompFor(item, iter, expr) => match expr {
                Some(node) => write!(f, "FOR {} IN {} IF {}", item, iter, node),
                None => write!(f, "FOR {} IN {}", item, iter),
            },
            Expr::TargetList(v) => write!(f, "{}", Node::vec_to_string(&v, ", ")),
            Expr::Op(left, op, right) => {
                match (op.enclose(&*left.kind), op.enclose(&*right.kind)) {
                    (true, true) => {
                        write!(f, "({}){}{}{}({})", left, op.space(), op, op.space(), right)
                    }
                    (true, false) => {
                        write!(f, "({}){}{}{}{}", left, op.space(), op, op.space(), right)
                    }
                    (false, true) => {
                        write!(f, "{}{}{}{}({})", left, op.space(), op, op.space(), right)
                    }
                    (false, false) => {
                        write!(f, "{}{}{}{}{}", left, op.space(), op, op.space(), right)
                    }
                }
            }
            Expr::Unary(op, right) => match op.enclose(&*right.kind) {
                true => write!(f, "{}{}({})", op, op.space(), right),
                false => write!(f, "{}{}{}", op, op.space(), right),
            },
            Expr::Atom(a) => write!(f, "{}", a),
        }
    }
}

impl fmt::Display for Atom<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Atom::Enclosure(left, op, right) => {
                write!(
                    f,
                    "{}{}{}{}{}",
                    left,
                    left.space(),
                    op,
                    right.space(),
                    right
                )
            }
            Atom::Vector(nodes) => write!(f, "[{}]", Node::vec_to_string(&nodes, ", ")),
            Atom::Tuple(nodes) => match nodes.len() {
                1 => write!(f, "({},)", nodes[0]),
                _ => write!(f, "({})", Node::vec_to_string(&nodes, ", ")),
            },
            Atom::String(s) => write!(f, r#""{}""#, s),
            Atom::Integer(n) => write!(f, "{}", n),
            Atom::Id(s) => write!(f, "{}", s),
        }
    }
}
