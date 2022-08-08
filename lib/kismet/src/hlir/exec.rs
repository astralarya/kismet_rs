use crate::types::BaseNode;

pub trait Exec<U, V, E> {
    fn exec(&self, i: U) -> Result<V, E>;
}

impl<S, T, U, V, E> Exec<U, V, BaseNode<S, E>> for BaseNode<S, T>
where
    T: Exec<U, V, E>,
    BaseNode<S, E>: TryFrom<E>,
    S: Clone,
    E: Clone,
{
    fn exec(&self, i: U) -> Result<V, BaseNode<S, E>> {
        self.data
            .exec(i)
            .map_err(|x| match BaseNode::<S, E>::try_from(x.clone()) {
                Ok(x) => x,
                Err(_) => BaseNode::new(self.span.clone(), x),
            })
    }
}
