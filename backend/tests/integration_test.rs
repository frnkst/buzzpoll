use actix_web::{test, web, App};
use buzzpoll::{app_state, services, Poll};
use http::StatusCode;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[actix_web::test]
async fn test_get_polls_empty() {
    let app_state = Arc::new(app_state::AppState::new());
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(services::get_polls),
    )
    .await;
    let req = test::TestRequest::get().uri("/poll").to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body, serde_json::json!({}));
}

#[actix_web::test]
async fn test_get_polls() {
    let polls: Mutex<HashMap<String, Poll>> = Mutex::new(HashMap::new());
    let expected_poll = some_poll();
    polls
        .lock()
        .unwrap()
        .insert(expected_poll.id.clone(), expected_poll.clone());

    let app_state = Arc::new(app_state::AppState {
        clients: Mutex::new(vec![]),
        polls: polls,
    });
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(services::get_polls),
    )
    .await;
    let req = test::TestRequest::get().uri("/poll").to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    let mut expected_result = HashMap::new();
    expected_result.insert(1, expected_poll);
    assert_eq!(body, serde_json::json!(expected_result));
}

#[actix_web::test]
async fn test_get_one_poll() {
    let polls: Mutex<HashMap<String, Poll>> = Mutex::new(HashMap::new());
    let expected_poll = some_poll();
    polls
        .lock()
        .unwrap()
        .insert(expected_poll.id.clone(), expected_poll.clone());

    let app_state = Arc::new(app_state::AppState {
        clients: Mutex::new(vec![]),
        polls: polls,
    });
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(services::get_poll),
    )
    .await;
    let req = test::TestRequest::get().uri("/poll/1").to_request();

    let body: Poll = test::call_and_read_body_json(&app, req).await;
    assert_eq!(body, expected_poll);
}

#[actix_web::test]
async fn test_get_one_that_doesnt_exists() {
    let polls: Mutex<HashMap<String, Poll>> = Mutex::new(HashMap::new());
    let expected_poll = some_poll();
    polls
        .lock()
        .unwrap()
        .insert(expected_poll.id.clone(), expected_poll.clone());

    let app_state = Arc::new(app_state::AppState {
        clients: Mutex::new(vec![]),
        polls: polls,
    });
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(services::get_poll),
    )
    .await;
    let req = test::TestRequest::get().uri("/poll/999").to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

fn some_poll() -> Poll {
    Poll {
        id: "test-poll".to_string(),
        question: String::from("Where is the love?"),
        answers: HashMap::new(),
        votes: HashMap::new(),
    }
}
