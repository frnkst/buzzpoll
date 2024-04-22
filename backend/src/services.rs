use std::sync::Arc;
use crate::model::{Poll, PollMessage, VoteRequest, AppState};
use actix_web::{get, post, web, Error, HttpResponse};
use crate::model;

#[get("/poll")]
async fn get_polls(data: web::Data<Arc<AppState>>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(&data.polls()))
}

#[get("/poll/{poll_id}")]
async fn get_poll(
    data: web::Data<Arc<AppState>>,
    path: web::Path<u32>,
) -> Result<HttpResponse, Error> {
    let poll_id = path.into_inner();
    let all_polls = data.polls().lock().unwrap();
    let poll = all_polls.iter().find(|x| x.id == poll_id);
    Ok(HttpResponse::Ok().json(poll))
}

#[post("/poll")]
async fn create_poll(
    poll: web::Json<Poll>,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let mut all_polls = data.polls().lock().unwrap();
    all_polls.push(poll.0.clone());
    Ok(HttpResponse::Ok().json(poll))
}

async fn broadcast_poll(data: &web::Data<Arc<AppState>>, poll: &Poll) {
    let poll_message = PollMessage{ poll: poll.clone() };
    for client in data.clients().lock().unwrap().iter_mut() {
        client.send(poll_message.clone()).await.expect("Could not send poll to clients");
    }
}

#[post("/vote")]
async fn vote(
    vote_request: web::Json<VoteRequest>,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let mut all_polls = data.polls().lock().unwrap();

    // Find the poll
    if let Some(poll) = all_polls.iter_mut().find(|p| p.id == vote_request.id) {
        // Find the answer
        if let Some(answer) = poll
            .answers
            .as_mut()
            .and_then(|answers| answers.iter_mut().find(|a| a.id == vote_request.answer.id))
        {
            // Push a new vote into the votes vector
            answer.votes.push(model::Vote {
                client: "example_client".to_string(),
            });
            println!("Vote added successfully!");
            broadcast_poll(&data, poll).await;
        } else {
            println!("Answer with the id {} not found", vote_request.answer.id)
        }
    } else {
        println!("Poll with the id {} not found", vote_request.id)
    }

    Ok(HttpResponse::Ok().body("done!"))
}
