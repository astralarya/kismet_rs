use crate::types::Node;

pub trait Exec<U, V, E> {
    fn exec(&self, i: U) -> Result<V, E>;
}

impl<T, U, V, E> Exec<U, V, E> for Node<T>
where
    T: Exec<U, V, E>,
{
    fn exec(&self, i: U) -> Result<V, E> {
        self.data.exec(i)
    }
}
