use serde::{Deserialize, Serialize};
use std::sync::{Mutex};
use actix::{Addr, Message};
use crate::MyWs;

pub struct AppState {
    clients: Mutex<Vec<Addr<MyWs>>>,
    polls: Mutex<Vec<Poll>>
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            clients: Mutex::new(Vec::new()),
            polls: Mutex::new(Vec::new())
        }
    }

    pub fn clients(&self) -> &Mutex<Vec<Addr<MyWs>>> {
        return &self.clients;
    }

    pub fn polls(&self) -> &Mutex<Vec<Poll>> {
        return &self.polls;
    }
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub(crate) struct PollMessage {
    pub(crate) poll: Poll,
}

#[derive(Serialize, Deserialize, Clone, Message)]
#[rtype(result = "()")]
pub(crate) struct Poll {
    pub(crate) id: u32,
    pub(crate) question: String,
    pub(crate) answers: Option<Vec<Answer>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Answer {
    pub(crate) id: u32,
    pub(crate) text: String,
    pub(crate) votes: Vec<Vote>,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Vote {
    pub(crate) client: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct VoteRequest {
    pub(crate) id: u32,
    pub(crate) answer: Answer,
    pub(crate) client_id: String,
}
