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
    pub answers: Vec<Answer>,
}

#[derive(Serialize, Deserialize, Clone, Message, Debug, PartialEq)]
#[rtype(result = "()")]
pub struct Answer {
    pub id: String,
    pub text: String,
    pub votes: Vec<String>,
}
