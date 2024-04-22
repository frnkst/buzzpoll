use crate::model::Poll;
use crate::MyWs;
use actix::Addr;
use std::sync::Mutex;

pub struct AppState {
    clients: Mutex<Vec<Addr<MyWs>>>,
    polls: Mutex<Vec<Poll>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            clients: Mutex::new(Vec::new()),
            polls: Mutex::new(Vec::new()),
        }
    }

    pub fn clients(&self) -> &Mutex<Vec<Addr<MyWs>>> {
        return &self.clients;
    }

    pub fn polls(&self) -> &Mutex<Vec<Poll>> {
        return &self.polls;
    }
}
