use std::sync::Arc;
use actix::{Actor, Handler, StreamHandler};
use actix_cors::Cors;
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, web};
use actix_web::middleware::Logger;
use actix_web_actors::ws;
use env_logger::Env;
use crate::{app_state, model, services};


#[actix_web::main]
pub async fn run() -> std::io::Result<()>  {
    let app_state = Arc::new(app_state::AppState::new());

    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .app_data(web::Data::new(app_state.clone()))
            .service(services::create_poll)
            .service(services::get_poll)
            .service(services::get_polls)
            .service(services::vote)
            .route("/ws/", web::get().to(start_websocket))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[derive(Clone)]
pub struct MyWs {}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("Websocket stopped, now removing the client from the list. But how???");

    }
}

// Echoing all messages back
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            ctx.text(text);
        }
    }
}

// Sending the poll to clients
impl Handler<model::Poll> for MyWs {
    type Result = ();

    fn handle(&mut self, poll: model::Poll, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.text(serde_json::to_string(&poll).expect("Could no serialize poll"));
    }
}

pub async fn start_websocket(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<Arc<app_state::AppState>>,
) -> Result<HttpResponse, Error> {
    let actor = MyWs {};
    let (addr, response) = ws::WsResponseBuilder::new(actor, &req, stream)
        .start_with_addr()
        .expect("Could not start new actor");
    data.clients.lock().unwrap().push(addr);
    println!("Added client. Now serving {} clients over websockets", data.clients.lock().unwrap().len());
    Ok(response)
}
