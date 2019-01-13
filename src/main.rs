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
                    let has_sum = self
                        .bids
                        .iter()
                        .filter(|x| x.request.price <= request.price)
                        .map(|x| x.request.size)
                        .scan(0, |s, x| {
                            *s += x;
                            Some(*s)
                        })
                        .any(|x| x >= request.price);
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
}

#[cfg(test)]
mod tests {
    use crate::dto::*;
    use crate::Matcher;

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
}
