use std::sync::Arc;
use actix_web::{App, web, test};
use http::StatusCode;
use buzzpoll::{app_state, services};
use serde_json::Value;

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


