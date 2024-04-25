use crate::app_state::AppState;
use crate::model::{Poll, VoteRequest};
use crate::{Answer, CreatePollRequest};
use actix_web::http::Error;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use nanoid::nanoid;
use std::sync::Arc;
use actix_web::cookie::Cookie;

// TODO
// Write unit and integration tests
// Test everything...
// Never use unwrap
// Write endpoints to do a string poll and store data

#[get("/poll")]
async fn get_polls(data: web::Data<Arc<AppState>>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(&data.polls))
}

#[get("/poll/{poll_id}")]
async fn get_poll(
    data: web::Data<Arc<AppState>>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let poll_id = path.into_inner();
    let all_polls = data.polls.lock().unwrap();
    let poll = all_polls.get(&poll_id);

    match poll {
        None => Ok(HttpResponse::NotFound().body(format!("Poll with id {} not found", &poll_id))),
        Some(poll) => Ok(HttpResponse::Ok().json(poll)),
    }
}

#[post("/poll")]
async fn create_poll(
    create_poll_request: web::Json<CreatePollRequest>,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let poll = Poll {
        id: nanoid!(),
        question: String::from(&create_poll_request.question),
        answers: create_poll_request
            .answers
            .iter()
            .map(|answer| Answer {
                id: nanoid!(),
                text: String::from(answer),
                votes: Vec::new(),
            })
            .collect(),
    };

    let mut all_polls = data.polls.lock().unwrap();
    all_polls.insert(poll.id.clone(), poll.clone());
    Ok(HttpResponse::Ok().json(poll))
}

fn get_cookie_value(req: HttpRequest) -> Cookie<'static> {
    req.cookie("buzzpoll").expect("No cookie found with the name buzzpoll")
}

#[post("/vote")]
async fn vote(
    req: HttpRequest,
    vote_request: web::Json<VoteRequest>,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    //let cookie_vote_id = get_cookie_value(req);
    let cookie_vote_id = String::from("frank");

    let mut all_polls = data.polls.lock().unwrap();
    let answer = all_polls
        .get_mut(&vote_request.poll_id)
        .unwrap()
        .answers
        .iter_mut()
        .find(|answer| answer.id == vote_request.answer_id)
        .unwrap();

    // This is highly questionable... There must be a much better alternative
    answer.votes.push(cookie_vote_id.to_string());
    answer.votes.sort_unstable();
    answer.votes.dedup();

    broadcast_poll(&data, &all_polls.get(&vote_request.poll_id).unwrap()).await;

    Ok(HttpResponse::Ok().body("done!"))
}

async fn broadcast_poll(data: &web::Data<Arc<AppState>>, poll: &Poll) {
    for client in data.clients.lock().unwrap().iter_mut() {
        client
            .send(poll.clone())
            .await
            .expect("Could not send poll to clients");
    }
}
