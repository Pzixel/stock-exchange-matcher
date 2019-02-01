use std::ops::Index;
use std::ops::RangeBounds;
use std::vec::Drain;
pub struct SortedVec<T> {
    items: Vec<T>,
}

impl<T: Ord> SortedVec<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        let items = &self.items;
        let position = match items.binary_search(&item) {
            Ok(pos) => pos,
            Err(pos) => pos,
        };
        self.items.insert(position, item);
    }

    pub fn drain<R>(&mut self, range: R) -> Drain<T>
    where
        R: RangeBounds<usize>,
    {
        self.items.drain(range)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
}

impl<T> Index<usize> for SortedVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.items.index(index)
    }
}
