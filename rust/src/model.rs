use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use actix_web_actors::ws::WebsocketContext;
use crate::MyWs;

pub(crate) struct WsAppState {
    pub(crate) clients: Mutex<Vec<*mut WebsocketContext<MyWs>>>,
}

pub(crate) struct AppState {
    pub(crate) all_polls: Arc<Mutex<Vec<Poll>>>,
    pub(crate) clients: Arc<Mutex<Vec<WebsocketContext<MyWs>>>>,
}

#[derive(Serialize, Deserialize, Clone)]
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
