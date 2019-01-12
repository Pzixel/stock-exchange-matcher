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

    pub fn add_request(&mut self, request: Request) {
        self.current_request_id += 1;
        let vec = match request.side {
            Side::Ask => &mut self.asks,
            Side::Bid => &mut self.bids,
        };
        vec.push(Order {
            id: self.current_request_id,
            request,
        });
    }
}

#[derive(Debug)]
enum Side {
    Ask, // buy
    Bid, // sell
}

#[derive(Debug)]
enum RequestType {
    Limit,
    FillOrKill,
    ImmediateOrCancel,
}

struct Request {
    side: Side,
    price: u64,
    size: u64,
    user_id: u64,
    request_type: RequestType,
}

struct Order {
    id: u64,
    request: Request,
}
