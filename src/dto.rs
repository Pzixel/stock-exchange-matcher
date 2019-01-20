#[derive(Debug, Eq, PartialEq)]
pub enum MatchingResult {
    Queued,
    Executed(Vec<u64>),
    Cancelled,
}

#[derive(Debug)]
pub enum Side {
    Ask, // sell
    Bid, // buy
}

#[derive(Debug)]
pub enum RequestType {
    Limit,
    FillOrKill,
    ImmediateOrCancel,
}

pub struct Request {
    pub side: Side,
    pub price: u64,
    pub size: u64,
    pub user_id: u64,
    pub request_type: RequestType,
}
