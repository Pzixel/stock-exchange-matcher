use std::cmp::Ordering;
use std::ops::Index;
use std::ops::RangeBounds;
use std::vec::Drain;

pub struct SortedVec<T, F> {
    items: Vec<T>,
    comparer_fn: F,
}

impl<T, F: FnMut(&'_ T, &'_ T) -> Ordering> SortedVec<T, F> {
    pub fn new(comparer_fn: F) -> Self {
        Self {
            items: Vec::new(),
            comparer_fn,
        }
    }

    pub fn insert(&mut self, item: T) {
        let items = &self.items;
        let comparer_fn = &mut self.comparer_fn;
        let position = match items.binary_search_by(|x| (comparer_fn)(x, &item)) {
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
}

impl<T, F> Index<usize> for SortedVec<T, F> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.items.index(index)
    }
}
