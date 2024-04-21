use std::sync::{Arc, Mutex};
use actix::{Actor, Addr, AsyncContext, Handler, Message, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse, HttpServer, App};
use actix_web::middleware::Logger;
use actix_web_actors::ws;
use env_logger::Env;
use model::Poll;

mod model;
mod services;

// What is Mutex? A mutual exclusion primitive useful for protecting shared data
// What is Arc? A thread-safe reference-counting pointer. ‘Arc’ stands for ‘Atomically Reference Counted’.

// Goal: Minimal working example for pushing a shared state to websocket clients
// 1. Broadcast to all connected clients
// 2. Broadcast a shared state
// 3. Update the shared state via a REST API

// WebSocket

#[derive(Clone)]
struct MyWs {}

// Actor for WebSocket
// What is an Actor? Actors communicate via Messages, via the handle Method
// Idea: Maybe I can keep a list of Actors
impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;

    // Maybe I can add myself to the global object
    fn started(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        // In here I can't access the global state
        //let addr = ctx.address();
        //self.addr = addr;
        println!("stored the address of the first context address: {:?}", ctx.address())
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

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct PollMessage {
    pub poll: Poll,
}

/// Handler for Poll message.
impl Handler<PollMessage> for MyWs {
    type Result = ();

    fn handle(&mut self, poll_message: PollMessage, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.text(format!("hi there {}", poll_message.poll.id));
    }
}

struct ChatState {
    pub clients: Mutex<Vec<Addr<MyWs>>>,
    pub all_polls: Mutex<Vec<Poll>>
}

async fn index(req: HttpRequest, stream: web::Payload, data: web::Data<Arc<ChatState>>) -> Result<HttpResponse, Error> {
    // In here I could access the global state
    let webSocketActor = MyWs {};

    // Here comes the trick part! Wow this just worked!
    //data.clients.lock().unwrap().push(webSocketActor.clone());
    //data.clients.lock().unwrap().push(webSocketActor.clone());

    let resp = ws::WsResponseBuilder::new(webSocketActor, &req, stream).start_with_addr();
    let addr = resp.unwrap();
    println!("{:?}", addr.0);
    data.clients.lock().unwrap().push(addr.0);
    Ok(addr.1)
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



/*
#[get("/test")]
async fn do_something(data: web::Data<Arc<ChatState>>) -> Result<HttpResponse, Error> {
    // This works!
    //let len = data.clients.lock().unwrap().len();
    //println!("len is {}", len);

    // now comes the last tricky part. send a message to all websocket clients
    /*
    for a in data.clients.lock().unwrap().iter_mut() {
        a.send(ClientMessage{ id: 5}).await.expect("TODO: panic message");
        //adr.do_send(ClientMessage{ id: 55});

    }
    */

    //let _ = MyWs{}.send(ClientMessage{id : 5});

    Ok(HttpResponse::Ok().body("yes"))
}
*/
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chat_state = Arc::new(ChatState {
        clients: Mutex::new(Vec::new()),
        all_polls: Mutex::new(Vec::new()),
    });

    env_logger::init_from_env(Env::default().default_filter_or("debug"));

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
