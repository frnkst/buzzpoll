use std::io::Bytes;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use actix::{Actor, Context, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse, HttpServer, App, get};
use actix_web::web::{Data, service};
use actix_web_actors::ws;
use actix_web_actors::ws::{Message, WebsocketContext};
use bytestring::ByteString;
use futures::SinkExt;
use serde::{Deserialize, Serialize};

// What is Mutex? A mutual exclusion primitive useful for protecting shared data
// What is Arc? A thread-safe reference-counting pointer. ‘Arc’ stands for ‘Atomically Reference Counted’.

// Goal: Minimal working example for pushing a shared state to websocket clients
// 1. Broadcast to all connected clients
// 2. Broadcast a shared state
// 3. Update the shared state via a REST API

// WebSocket

#[derive(Clone)]
struct MyWs;

// Actor for WebSocket
// What is an Actor? Actors communicate via Messages, via the handle Method
// Idea: Maybe I can keep a list of Actors
impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;

    // Maybe I can add myself to the global object
    fn started(&mut self, ctx: &mut WebsocketContext<Self>) {
        // In here I can't access the global state
    }
}

// What is a Stream Handler?
// This is helper trait that allows handling Streams in a similar way
// to normal actor messages. When stream resolves its next item, handle() is called with that item.
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    // In here I can't access the global state
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                // Handle incoming messages (if needed)
                // For broadcasting, we'll focus on sending messages to all connected clients.
                ctx.text(text);
            }
            _ => (),
        }
    }
}

struct ChatState<'a> {
    pub clients: Mutex<Vec<&'a MyWs>>
}

async fn index(req: HttpRequest, stream: web::Payload, data: Data<Arc<ChatState<'_>>>) -> Result<HttpResponse, Error> {
    // In here I could access the global state
    let webSocketActor = MyWs {};

    // Here comes the trick part! Wow this just worked!
    //data.clients.lock().unwrap().push(webSocketActor.clone());
    data.clients.lock().unwrap().push(&webSocketActor.clone());

    let resp = ws::start(webSocketActor, &req, stream);
    println!("{:?}", resp);
    resp
}

/*
fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    // Create a new WebSocket actor for each connection
    // Here I could also add it to the global state
    let ws = MyWs{};
    let resp = ws::start(ws, &req, stream);

    // Add the sender to the list of clients

    let state = req.app_data::<web::Data<Arc<ChatState>>>().unwrap();
    //let mut clients = state.clients.lock().unwrap();
    //clients.push(&ws);

    resp
}
*/
/*
fn broadcast_message(state: web::Dbeforeata<Arc<ChatState>>, message: &str) {
    let clients = state.clients.lock().unwrap();
    for client in clients.iter() {
        client.send(message).ok(); // Ignore errors for simplicity
    }
}
*/

#[get("/test")]
async fn do_something(data: Data<Arc<ChatState<'_>>>) -> Result<HttpResponse, Error> {
    let len = data.clients.lock().unwrap().len();
    println!("len is {}", len);

    // now comes the last tricky part. send a message to all websocket clients
    let clients = data.clients.lock().unwrap().iter_mut();
    for c in clients {
        let bs = ByteString::from_static("hi");
        // I only have the actor but not the context which is needed as a second argument
        //c.handle(Ok(Message::Text(bs)));

    }

    Ok(HttpResponse::Ok().body("yes"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chat_state = Arc::new(ChatState {
        clients: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(chat_state.clone()))
            .service(do_something)
            .route("/ws/", web::get().to(index))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
