mod model;
mod services;

use std::sync::Mutex;
use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use crate::services::{create_poll, get_polls, hello};


/// Define HTTP actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

struct AppState {
    all_polls: Mutex<Vec<model::Poll>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let state = web::Data::new(AppState {
        all_polls: Mutex::new(Vec::new())
    });


    HttpServer::new(move || App::new()
        .app_data(state.clone())
        .service(hello)
        .service(create_poll)
        .service(get_polls)
        .route("/ws/", web::get().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
