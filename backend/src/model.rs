use actix::Message;
use serde::{Deserialize, Serialize};

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct PollMessage {
    pub poll: Poll,
}

#[derive(Serialize, Deserialize, Clone, Message)]
#[rtype(result = "()")]
pub struct Poll {
    pub id: u32,
    pub question: String,
    pub answers: Option<Vec<Answer>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Answer {
    pub id: u32,
    pub text: String,
    pub votes: Vec<Vote>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Vote {
    pub client: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VoteRequest {
    pub id: u32,
    pub answer: Answer,
    pub client_id: String,
}
