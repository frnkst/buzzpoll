use std::env::var;
use std::sync::Mutex;
use actix::{Actor, StreamHandler};
use actix_web::{web, get, post, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[derive(Serialize, Deserialize, Clone)]
struct Poll {
    id: u32,
    question: String,
    answers: Option<Vec<Answer>>
}

#[derive(Serialize, Deserialize, Clone)]
struct Answer {
    id: u32,
    text: String,
    votes: Vec<Vote>
}

#[derive(Serialize, Deserialize, Clone)]
struct Vote {
    client: String,
}


#[post("/poll")]
async fn createPoll(poll: web::Json<Poll>, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let mut all_polls = data.all_polls.lock().unwrap();
    all_polls.push(poll.0.clone());
    Ok(HttpResponse::Ok().json(poll))
}

#[get("/poll")]
async fn getPolls(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(&data.all_polls))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

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
    all_polls: Mutex<Vec<Poll>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let state = web::Data::new(AppState {
        all_polls: Mutex::new(Vec::new())
    });


    HttpServer::new(move || App::new()
        .app_data(state.clone())
        .service(hello)
        .service(createPoll)
        .service(getPolls)
        .route("/ws/", web::get().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
