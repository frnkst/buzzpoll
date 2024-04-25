use std::collections::HashMap;
use actix::Message;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Message, Debug, PartialOrd, PartialEq)]
#[rtype(result = "()")]
pub struct CreatePollRequest {
    pub question: String,
    pub answers: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VoteRequest {
    pub poll_id: String,
    pub answer_id: String,
}

#[derive(Serialize, Deserialize, Clone, Message, Debug, PartialEq)]
#[rtype(result = "()")]
pub struct Poll {
    pub id: String,
    pub question: String,
    pub answers: HashMap<String, String>,
    pub votes: HashMap<String, String>,
}


