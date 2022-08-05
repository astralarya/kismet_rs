use crate::types::Node;

pub trait Exec<T, U> {
    fn exec(&self, i: T) -> U;
}

impl<T, U, V> Exec<U, V> for Node<T>
where
    T: Exec<U, V>,
{
    fn exec(&self, i: U) -> V {
        self.data.exec(i)
    }
}
