use crate::{ws, Clients, Result};
use warp::Reply;

pub async fn ws_handler(ws: warp::ws::Ws, clients: Clients) -> Result<impl Reply> {
    println!("ws_handler"); //debug

    Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, clients)))
}

pub async fn metrics_handler(clients: Clients) -> Result<impl Reply> {
    let metricsString = "cosmic_kube_clients{type=\"connected\"} ".to_string()
        + &clients.lock().await.len().to_string()
        + "\n";
    Ok(metricsString)
}
