use crate::web::Data;
use std::sync::{Arc, RwLock};
use actix::{Actor, StreamHandler};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web::web::route;
use actix_web_actors::ws;

struct Game {
    websockets: Vec<Arc<RwLock<MyWs>>>,
}

impl Game {
    pub fn new() -> Self {
        Game { websockets: vec![] }
    }

    pub fn add_websocket(&mut self, my_ws: Arc<RwLock<MyWs>>) {
        self.websockets.push(my_ws);
    }

    pub fn some_method(&mut self) {
        // Do something to influence internal state.
        self.push_state();
    }

    pub fn push_state(&self) {
        for w in &self.websockets {
            // Send updates to clients using the WebsocketContext.
            //w.write().unwrap().ctx.text("hello");
        }
    }
}

struct MyWs {
    game: Arc<RwLock<Game>>,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                self.game.write().unwrap().some_method();
                ctx.text(text);
            }
            _ => (),
        }
    }
}

struct GameWrapper {
    game: Arc<RwLock<Game>>,
}

impl GameWrapper {
    pub fn new(game: Arc<RwLock<Game>>) -> Self {
        GameWrapper { game }
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let game = Arc::new(RwLock::new(Game::new()));
    let game_wrapper = RwLock::new(GameWrapper::new(game.clone()));
    let game_wrapper_data = web::Data::new(game_wrapper);

    HttpServer::new(move || {
        App::new()
            .app_data(game_wrapper_data.clone())
            .route("/play_game", web::get().to(play_game))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

pub async fn play_game(
    req: HttpRequest,
    stream: web::Payload,
    game_wrapper: web::Data<GameWrapper>,
) -> impl Responder {
    let my_ws = MyWs {
        game: game_wrapper.game.clone(),
    };
    //let my_ws = Arc::new(RwLock::new(my_ws));
    //let my_ws = MyWs { game: "bla"};

    let mut game = game_wrapper.game.write().unwrap();
    //game.add_websocket(my_ws.clone());

    let resp = ws::start(my_ws, &req, stream);
    resp.unwrap_or_else(|e| HttpResponse::from_error(e))
}
