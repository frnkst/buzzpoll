use crate::app_state::AppState;
use crate::model::{Poll, PollMessage, VoteRequest};
use crate::{Vote};
use actix_web::{get, post, web, HttpResponse};
use std::sync::Arc;
use actix_web::http::Error;

// TODO
// Who generates the unique id's? Is only the poll id a uuid or also the answers?
// What is a better shape for the vote request?
// Write unit and integration tests
// Test everything...

#[get("/poll")]
async fn get_polls(data: web::Data<Arc<AppState>>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(&data.polls))
}

#[get("/poll/{poll_id}")]
async fn get_poll(
    data: web::Data<Arc<AppState>>,
    path: web::Path<u32>,
) -> Result<HttpResponse, Error> {
    let poll_id = path.into_inner();
    let all_polls = data.polls.lock().unwrap();
    let poll = all_polls.get(&poll_id);
    Ok(HttpResponse::Ok().json(poll))
}

#[post("/poll")]
async fn create_poll(
    poll: web::Json<Poll>,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let mut all_polls = data.polls.lock().unwrap();
    all_polls.insert(poll.id, poll.0.clone());
    Ok(HttpResponse::Ok().json(poll))
}

async fn broadcast_poll(data: &web::Data<Arc<AppState>>, poll: &Poll) {
    let poll_message = PollMessage { poll: poll.clone() };
    for client in data.clients.lock().unwrap().iter_mut() {
        client
            .send(poll_message.clone())
            .await
            .expect("Could not send poll to clients");
    }
}

#[post("/vote")]
async fn vote(
    vote_request: web::Json<VoteRequest>,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let mut all_polls = data.polls.lock().unwrap();

    all_polls
        .get_mut(&vote_request.id)?
        .answers.iter_mut()
        .find(|answer| answer.id == vote_request.answer.id)?
        .votes
        .push(Vote {
            client: String::from("yey"),
        });

    broadcast_poll(&data, &all_polls.get(&vote_request.id).unwrap()).await;

    Ok(HttpResponse::Ok().body("done!"))
}
