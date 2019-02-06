use crate::collections::*;
use crate::dto::*;
use crate::models::*;

struct Matcher {
    asks: SortedVec<AsksOrder>,
    bids: SortedVec<BidsOrder>,
    current_request_id: u64,
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
                                requests_count_to_approve - 1 // TODO: Fix bug with unchanged size
                            } else {
                                requests_count_to_approve
                            };

                            let filled_requests = self.bids.drain(0..items_to_drain).map(|x| x.id).collect();
                            MatchingResult::Executed(filled_requests)
                        }
                        _ => MatchingResult::Cancelled,
                    }
                }
                Side::Bid => {
                    let requests_count_to_approve = self.requests_count_to_approve(&request);
                    match requests_count_to_approve {
                        Some(requests_count_to_approve) => {
                            let mut size = request.size;
                            for i in 0..requests_count_to_approve - 1 {
                                size -= self.asks[i as usize].request.size;
                            }

                            let items_to_drain = if self.asks[requests_count_to_approve - 1].request.size > size {
                                requests_count_to_approve - 1
                            } else {
                                requests_count_to_approve
                            };

                            let filled_requests = self.asks.drain(0..items_to_drain).map(|x| x.id).collect();
                            MatchingResult::Executed(filled_requests)
                        }
                        _ => MatchingResult::Cancelled,
                    }
                }
            },
            RequestType::Limit => {
                let mut request = request;
                match request.side {
                    Side::Ask => {
                        self.asks.push(AsksOrder(Order::new(self.current_request_id, request)));
                    }
                    Side::Bid => {
                        self.bids.push(BidsOrder(Order::new(self.current_request_id, request)));
                    }
                }

                self.current_request_id += 1;
                MatchingResult::Queued
            }
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
                .position(|x| x >= request.size),
        }
        .map(|i| i + 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::dto::*;
    use crate::matcher::Matcher;
    use assert_matches::assert_matches;

    #[test]
    pub fn test_limit() {
        let mut matcher = Matcher::new();
        let request1 = Request {
            side: Side::Ask,
            price: 10,
            size: 10,
            user_id: 0,
            request_type: RequestType::Limit,
        };

        let request2 = Request {
            side: Side::Ask,
            price: 10,
            size: 10,
            user_id: 0,
            request_type: RequestType::Limit,
        };

        let request3 = Request {
            side: Side::Ask,
            price: 10,
            size: 10,
            user_id: 0,
            request_type: RequestType::Limit,
        };

        let request4 = Request {
            side: Side::Bid,
            price: 10,
            size: 15,
            user_id: 0,
            request_type: RequestType::Limit,
        };

        let request5 = Request {
            side: Side::Bid,
            price: 10,
            size: 15,
            user_id: 0,
            request_type: RequestType::Limit,
        };

        let result1 = matcher.try_match(request1);
        let result2 = matcher.try_match(request2);
        let result3 = matcher.try_match(request3);
        let result4 = matcher.try_match(request4);
        let result5 = matcher.try_match(request5);

        assert_eq!(result1, MatchingResult::Queued);
        assert_eq!(result2, MatchingResult::Queued);
        assert_eq!(result3, MatchingResult::Queued);
        assert_matches!(result4, MatchingResult::Executed(_));
        assert_matches!(result5, MatchingResult::Executed(_));
    }

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
    pub fn test_fill_or_kill_sell_success() {
        let mut matcher = Matcher::new();
        let bid_request = Request {
            side: Side::Ask,
            price: 10,
            size: 5,
            user_id: 0,
            request_type: RequestType::Limit,
        };

        let bid_request2 = Request {
            side: Side::Ask,
            price: 10,
            size: 5,
            user_id: 0,
            request_type: RequestType::Limit,
        };

        let ask_request = Request {
            side: Side::Bid,
            price: 10,
            size: 10,
            user_id: 0,
            request_type: RequestType::FillOrKill,
        };

        let bid_result = matcher.try_match(bid_request);
        let bid_result2 = matcher.try_match(bid_request2);
        let ask_result = matcher.try_match(ask_request);

        assert_eq!(bid_result, MatchingResult::Queued);
        assert_eq!(bid_result2, MatchingResult::Queued);
        assert_matches!(ask_result, MatchingResult::Executed(_));
    }

    #[test]
    pub fn test_fill_or_kill_sell_empty() {
        let mut matcher = Matcher::new();
        let request = Request {
            side: Side::Bid,
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
            size: 5,
            user_id: 0,
            request_type: RequestType::Limit,
        };

        let bid_request2 = Request {
            side: Side::Bid,
            price: 10,
            size: 5,
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
        let bid_result2 = matcher.try_match(bid_request2);
        let ask_result = matcher.try_match(ask_request);

        assert_eq!(bid_result, MatchingResult::Queued);
        assert_eq!(bid_result2, MatchingResult::Queued);
        assert_matches!(ask_result, MatchingResult::Executed(_));
    }
}
