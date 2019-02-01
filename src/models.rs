use crate::dto::*;
use std::cmp::Ordering;
use std::ops::Deref;

pub struct AsksOrder(pub Order);
pub struct BidsOrder(pub Order);

impl PartialOrd for AsksOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for BidsOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for AsksOrder {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialEq for BidsOrder {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for AsksOrder {}
impl Eq for BidsOrder {}

impl Ord for AsksOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.request.price.cmp(&other.request.price) {
            Ordering::Equal => self.id.cmp(&other.id),
            non_equal => non_equal,
        }
    }
}

impl Ord for BidsOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.request.price.cmp(&other.request.price).reverse() {
            Ordering::Equal => self.id.cmp(&other.id),
            non_equal => non_equal,
        }
    }
}

impl Deref for AsksOrder {
    type Target = Order;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for BidsOrder {
    type Target = Order;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Order {
    pub id: u64,
    pub request: Request,
}

impl Order {
    pub fn new(id: u64, request: Request) -> Self {
        Self { id, request }
    }
}
