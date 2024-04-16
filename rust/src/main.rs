mod model;
mod services;

use crate::model::{AppState, WsAppState};
use crate::services::{create_poll, get_poll, get_polls, vote};
use actix::{Actor, ActorContext, Addr, Context, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::sync::Mutex;
use actix_web_actors::ws::WebsocketContext;


/// Define HTTP actor
struct MyWs {
    app_data: web::Data<app_data::WsAppState>
}


impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        &self.state.clients.push(ctx);
        // println!("number of clients {:?}");
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        println!("schlafen {:?}", msg);

        &self.state.clients.push(ctx);

        // for ctx in &self.state.clients {
        //     unsafe { ctx.as_mut().expect("REASON").text("well well well"); }
        // }

        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(_)) => ctx.text("frank"),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload, app_data: web::Data<ws::AppState>) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs { app_data: app_data.clone() }, &req, stream);
    println!("frank {:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        all_polls: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(get_polls)
            .service(get_poll)
            .service(create_poll)
            .service(vote)
            .route("/ws/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
