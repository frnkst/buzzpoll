mod model;
mod services;

use crate::model::{AppState, WsAppState};
use crate::services::{create_poll, get_poll, get_polls, vote};
use actix::{Actor, ActorContext, Addr, Context, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::sync::{Arc, Mutex};
use actix_web::web::Data;
use actix_web_actors::ws::WebsocketContext;


/// Define HTTP actor
struct MyWs {
    data: Data<AppState>,
}


impl Actor for MyWs {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        //println!("pushed one client");
        // &self.app_data.clients.lock().unwrap().push(ctx);
        // println!("number of clients {:?}", &self.app_data.clients.lock().unwrap().len());
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        //println!("schlafen {:?}", msg);

        let len = self.data.clients.lock().unwrap().len();
        println!("test {:?}", len);

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

async fn index(req: HttpRequest, stream: web::Payload, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    println!("frank");
    //let state = Data::new(WsAppState {
    //    clients: Mutex::new(Vec::new()),
    //});

    // let resp = ws::start(MyWs { app_data: state.clone() }, &req, stream);
    let resp = ws::start(MyWs{ data}, &req, stream);
    //println!("frank {:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let state = web::Data::new(AppState {
        all_polls: Arc::new(Mutex::new(Vec::new())),
        clients: Arc::new(Mutex::new(Vec::new())),
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
