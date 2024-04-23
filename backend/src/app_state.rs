use crate::model::Poll;
use crate::MyWs;
use actix::Addr;
use std::sync::Mutex;

pub struct AppState {
    pub clients: Mutex<Vec<Addr<MyWs>>>,
    pub polls: Mutex<Vec<Poll>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            clients: Mutex::new(Vec::new()),
            polls: Mutex::new(Vec::new()),
        }
    }
}
