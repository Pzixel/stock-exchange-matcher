use std::cmp::Ordering;
use std::marker::PhantomData;
use std::ops::Index;
use std::ops::RangeBounds;
use std::vec::Drain;

pub trait Comparer<T> {
    fn cmp<'a>(a: &'a T, b: &'a T) -> Ordering;
}

pub struct SortedVec<T, C> {
    items: Vec<T>,
    comparer: PhantomData<C>,
}

impl<T, C: Comparer<T>> SortedVec<T, C> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            comparer: PhantomData,
        }
    }

    pub fn push(&mut self, item: T) {
        let items = &self.items;
        let position = match items.binary_search_by(|x| C::cmp(x, &item)) {
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

impl<T, F> Index<usize> for SortedVec<T, F> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.items.index(index)
    }
}
