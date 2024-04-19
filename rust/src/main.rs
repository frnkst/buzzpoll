use std::sync::{Arc, Mutex};
use actix::{Actor, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse, HttpServer, App};
use actix_web::web::Data;
use actix_web_actors::ws;
use actix_web_actors::ws::WebsocketContext;
use futures::SinkExt;

struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
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

fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    // Create a new WebSocket actor for each connection
    let ws = MyWs{};
    let resp = ws::start(ws, &req, stream);

    // Add the sender to the list of clients

    let state = req.app_data::<web::Data<Arc<ChatState>>>().unwrap();
    let mut clients = state.clients.lock().unwrap();
    clients.push(&ws);



    resp
}

fn broadcast_message(state: web::Data<Arc<ChatState>>, message: &str) {
    let clients = state.clients.lock().unwrap();
    for client in clients.iter() {
        client.send(message).ok(); // Ignore errors for simplicity
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chat_state = Arc::new(ChatState {
        clients: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(chat_state.clone()))
            .route("/ws/", web::get().to(index))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
