use super::Node;


pub struct NodeIter<T>(
    dyn Iterator<Item = Node<T>>
);

impl<T> Iterator for NodeIter<T> {
    type Item = Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
