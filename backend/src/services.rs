use std::collections::HashMap;
use crate::app_state::AppState;
use crate::model::{Poll, VoteRequest};
use crate::{Answer, CreatePollRequest};
use actix_web::http::Error;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use nanoid::nanoid;
use std::sync::{Arc, TryLockResult};
use actix_web::cookie::Cookie;
use futures::SinkExt;

// TODO
// Write integration test for vote
// Write unit tests
// Extend functionality and write endpoints to do a string poll and store data

#[get("/poll")]
async fn get_polls(data: web::Data<Arc<AppState>>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(&data.polls))
}

fn get_poll_by_id(poll_id: &str, data: &actix_web::web::Data<Arc<AppState>>) -> Poll {
    let mut all_polls = data.polls.lock().unwrap();
    all_polls.iter_mut().find(|poll| poll.id == poll_id).cloned().unwrap()
}

#[get("/poll/{poll_id}")]
async fn get_poll(
    data: web::Data<Arc<AppState>>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let poll_id = path.into_inner();
    let poll = get_poll_by_id(&poll_id, &data);

    Ok(HttpResponse::Ok().json(poll))
}

#[post("/poll")]
async fn create_poll(
    mut create_poll_request: web::Json<CreatePollRequest>,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let answers: Vec<Answer> = create_poll_request.answers.iter_mut()
        .map(|answer| Answer{ id: nanoid!(), text: answer.to_string()})
        .collect();

    let poll = Poll {
        id: nanoid!(),
        question: String::from(&create_poll_request.question),
        answers,
        votes: HashMap::new(),
    };

    let mut all_polls = data.polls.lock().unwrap();
    all_polls.push(poll.clone());
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
    let cookie_vote_id = get_cookie_value(req);
    let mut poll = get_poll_by_id(&vote_request.poll_id, &data);

    poll.votes.insert(vote_request.answer_id.to_string(), cookie_vote_id.to_string());
    broadcast_poll(&data, &poll).await;

    Ok(HttpResponse::Ok().body("done!"))
}

async fn broadcast_poll(data: &web::Data<Arc<AppState>>, poll: &Poll) {
    let lock_result = data.clients.lock();

    match lock_result {
        Ok(mut res) => {
            let a = res.iter_mut();
            for b in a {
                b.send(poll.clone()).await.map_err(|err| eprintln!("mailbox error")).unwrap();
            }
        }
        Err(_) => {
            eprint!("Could not accuire lock")
        }
    }
}
