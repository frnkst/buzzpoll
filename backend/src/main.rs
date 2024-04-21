use std::sync::{Arc, Mutex};
use actix::{Actor, Handler, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse, HttpServer, App};
use actix_web::middleware::Logger;
use actix_web_actors::ws;
use env_logger::Env;

mod model;
mod services;

#[derive(Clone)]
struct MyWs {}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

// Echoing all messages back
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                ctx.text(text);
            }
            _ => (),
        }
    }
}

// Sending the poll to clients
impl Handler<model::PollMessage> for MyWs {
    type Result = ();

    fn handle(&mut self, poll_message: model::PollMessage, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.text(serde_json::to_string(&poll_message.poll).unwrap());
    }
}

async fn index(req: HttpRequest, stream: web::Payload, data: web::Data<Arc<model::AppState>>) -> Result<HttpResponse, Error> {
    let actor = MyWs {};

    let addr = ws::WsResponseBuilder::new(actor, &req, stream).start_with_addr().unwrap();
    data.clients.lock().unwrap().push(addr.0);
    Ok(addr.1)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chat_state = Arc::new(model::AppState {
        clients: Mutex::new(Vec::new()),
        polls: Mutex::new(Vec::new()),
    });

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(chat_state.clone()))
            .service(services::create_poll)
            .service(services::get_poll)
            .service(services::get_polls)
            .service(services::vote)
            .route("/ws/", web::get().to(index))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
