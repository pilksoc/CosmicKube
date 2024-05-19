#![feature(async_closure)]
use std::convert::Infallible;
use cosmic_kube::{Clients, CLIENTS};
use warp::{Filter, Rejection};
use std::backtrace::Backtrace;
use std::panic;

mod handlers;
mod ws;
mod mem_client;

// type aliases!
type Result<T> = std::result::Result<T, Rejection>;


#[tokio::main(flavor = "multi_thread")]
async fn main() {
    panic::set_hook(Box::new(|info| {
        //let stacktrace = Backtrace::capture();
        let stacktrace = Backtrace::force_capture();
        println!("Got panic. @info:{}\n@stackTrace:{}", info, stacktrace);
        std::process::abort();
    }));

    //initialise a hashmap to store currently connected clients. We may want some more logic here if we want currently connected clients to be stored somewhere
    println!("Turning console on"); //debug
    console_subscriber::init();

    println!("Configuring websocket route"); //debug
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_clients(CLIENTS.clone()))
        .and_then(handlers::ws_handler)
        .or(warp::path("metrics")
            .and(with_clients(CLIENTS.clone()))
            .and_then(handlers::metrics_handler));

    let routes = ws_route.with(warp::cors().allow_any_origin());
    println!("Starting server on http://0.0.0.0:8000"); //debug
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}
