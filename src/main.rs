mod dto;
use crate::dto::*;

fn main() {
    println!("Hello, world!");
}

struct Matcher {
    asks: Vec<Order>,
    bids: Vec<Order>,
    current_request_id: u64,
}

impl Matcher {
    pub fn new() -> Self {
        Self {
            asks: Vec::new(),
            bids: Vec::new(),
            current_request_id: 0,
        }
    }

    pub fn try_match(&mut self, request: Request) -> MatchingResult {
        match request.request_type {
            RequestType::FillOrKill => match request.side {
                Side::Ask => {
                    let has_sum = self.has_sum(request);
                    if !has_sum {
                        return MatchingResult::Cancelled;
                    }
                    unimplemented!()
                }
                Side::Bid => unimplemented!(),
            },
            RequestType::Limit => match request.side {
                Side::Ask => {
                    let has_sum = self.has_sum(request);
                    if !has_sum {
                        return MatchingResult::Cancelled;
                    }
                    unimplemented!()
                }
                Side::Bid => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }

    fn has_sum(&self, request: Request) -> bool {
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
                .any(|x| x >= request.price),
            Side::Bid => self
                .asks
                .iter()
                .filter(|x| x.request.price >= request.price)
                .map(|x| x.request.size)
                .scan(0, |s, x| {
                    *s += x;
                    Some(*s)
                })
                .any(|x| x <= request.price),
        }
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
