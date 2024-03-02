use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use warp::{filters::ws::Message, Filter, Rejection};

mod handlers;
mod ws;

// type that represents a connecting client
#[derive(Debug, Clone)]
pub struct Client {
    pub client_id: String,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

// type aliases!
type Clients = Arc<Mutex<HashMap<String, Client>>>;
type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {

    //initialise a hashmap to store currently connected clients. We may want some more logic here if we want currently connected clients to be stored somewhere
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    println!("configuring websocket route"); //debug
    let ws_route = warp::path("ws")
    .and(warp::ws())
    .and(with_clients(clients.clone()))
    .and_then(handlers::ws_handler);

    let routes = ws_route.with(warp::cors().allow_any_origin());
    println!("starting server"); //debug
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

