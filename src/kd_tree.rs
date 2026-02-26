#[derive(Clone, Debug)]
enum Node<Coord, Value> {
    /// Value assumed by coordinates within a space.
    Leaf(Value),

    /// Coordinate and axis indicating where a space is split.
    Partition { coord: Coord, axis: usize },
}

pub struct KdTree<Coord, Value> {
    nodes: Vec<Option<Node<Coord, Value>>>
}

const DEFAULT_CAPACITY: usize = 50;

impl<Coord, Value> KdTree<Coord, Value>
where
    Coord: Clone + PartialOrd,
    Value: Clone,
{
    pub fn capacity(&self) -> usize {
        self.nodes.len()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: vec![None; capacity],
        }
    }

    pub fn new() -> Self {
        Self::default()
    }
}

impl<Coord, Value> Default for KdTree<Coord, Value>
where
    Coord: Clone + PartialOrd,
    Value: Clone,
{
    fn default() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }
}

#[cfg(test)]
mod tests {
    use crate::kd_tree::{DEFAULT_CAPACITY, KdTree};

    #[test]
    fn default() {
        let tree: KdTree<u64, u64> = Default::default();
        assert_eq!(tree.capacity(), DEFAULT_CAPACITY);
    }

    #[test]
    fn new() {
        let tree: KdTree<u64, u64> = KdTree::new();
        assert_eq!(tree.capacity(), DEFAULT_CAPACITY);
    }

    #[test]
    fn with_custom_capacity() {
        let capacity = 712;
        let tree: KdTree<u64, u64> = KdTree::with_capacity(capacity);
        assert_eq!(tree.capacity(), capacity);
    }
}
