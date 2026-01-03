use crate::{
    execution::{latency::LatencyModel, request::*},
    stream::hotloop::Mbo,
};

#[derive(Debug)]
pub enum Ack {
    Accepted,
    Rejected,
}

#[derive(Debug)]
pub enum Request {
    Trade(Trade),
    Modify(Modify),
    Cancel(Cancel),
}

impl Request {
    pub fn process<L: LatencyModel>(&mut self, mbo: &Mbo, l: &L) {
        match self {
            Request::Trade(r) => r.submit(mbo, l),
            Request::Modify(r) => r.submit(mbo, l),
            Request::Cancel(r) => r.submit(mbo, l),
        }
    }
}
