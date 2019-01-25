mod collections;
mod dto;
use crate::collections::*;
use crate::dto::*;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
}

struct AsksComparer;
struct BidsComparer;

impl Comparer<Order> for AsksComparer {
    fn cmp<'a>(a: &'a Order, b: &'a Order) -> Ordering {
        unimplemented!()
    }
}

impl Comparer<Order> for BidsComparer {
    fn cmp<'a>(a: &'a Order, b: &'a Order) -> Ordering {
        unimplemented!()
    }
}

struct Matcher {
    asks: SortedVec<Order, AsksComparer>,
    bids: SortedVec<Order, BidsComparer>,
    current_request_id: u64,
}

struct Order {
    pub id: u64,
    pub request: Request,
}

impl Order {
    pub fn new(id: u64, request: Request) -> Self {
        Self { id, request }
    }
}

impl Matcher {
    pub fn new() -> Self {
        Self {
            asks: SortedVec::new(),
            bids: SortedVec::new(),
            current_request_id: 0,
        }
    }

    pub fn try_match(&mut self, request: Request) -> MatchingResult {
        match request.request_type {
            RequestType::FillOrKill => match request.side {
                Side::Ask => {
                    let requests_count_to_approve = self.requests_count_to_approve(&request);
                    match requests_count_to_approve {
                        Some(requests_count_to_approve) => {
                            let mut size = request.size;
                            for i in 0..requests_count_to_approve - 1 {
                                size -= self.bids[i as usize].request.size;
                            }

                            let items_to_drain = if self.bids[requests_count_to_approve - 1].request.size > size {
                                requests_count_to_approve - 1
                            } else {
                                requests_count_to_approve
                            };

                            let filled_requests = self.bids.drain(0..items_to_drain).map(|x| x.id).collect();
                            MatchingResult::Executed(filled_requests)
                        }
                        _ => MatchingResult::Cancelled,
                    }
                }
                Side::Bid => unimplemented!(),
            },
            RequestType::Limit => match request.side {
                Side::Ask => unimplemented!(),
                Side::Bid => {
                    self.bids.push(Order::new(self.current_request_id, request));
                    self.current_request_id += 1;
                    MatchingResult::Queued
                }
            },
            _ => unimplemented!(),
        }
    }

    fn requests_count_to_approve(&self, request: &Request) -> Option<usize> {
        match request.side {
            Side::Ask => self
                .bids
                .iter()
                .filter(|x| x.request.price <= request.price)
                .map(|x| x.request.size)
                .scan(0, |s, x| {
                    *s += x;
                    Some(*s)
                })
                .position(|x| x >= request.size),
            Side::Bid => self
                .asks
                .iter()
                .filter(|x| x.request.price >= request.price)
                .map(|x| x.request.size)
                .scan(0, |s, x| {
                    *s += x;
                    Some(*s)
                })
                .position(|x| x <= request.size),
        }
        .map(|i| i + 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::dto::*;
    use crate::Matcher;
    use assert_matches::assert_matches;

    #[test]
    pub fn test_fill_or_kill_buy_empty() {
        let mut matcher = Matcher::new();
        let request = Request {
            side: Side::Ask,
            price: 10,
            size: 10,
            user_id: 0,
            request_type: RequestType::FillOrKill,
        };

        let result = matcher.try_match(request);

        assert_eq!(result, MatchingResult::Cancelled);
    }

    #[test]
    pub fn test_fill_or_kill_buy_success() {
        let mut matcher = Matcher::new();
        let bid_request = Request {
            side: Side::Bid,
            price: 10,
            size: 10,
            user_id: 0,
            request_type: RequestType::Limit,
        };

        let ask_request = Request {
            side: Side::Ask,
            price: 10,
            size: 10,
            user_id: 0,
            request_type: RequestType::FillOrKill,
        };

        let bid_result = matcher.try_match(bid_request);
        let ask_result = matcher.try_match(ask_request);

        assert_eq!(bid_result, MatchingResult::Queued);
        assert_matches!(ask_result, MatchingResult::Executed(_));
    }
}
