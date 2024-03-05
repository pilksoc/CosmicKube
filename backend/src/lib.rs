use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use warp::filters::ws::Message;

use crate::grid::Grid;

pub mod grid;
pub mod space;
pub mod kube;
pub mod player;
pub mod local_grid;
pub mod cache_client;
pub mod recipe;
pub mod vecmap;
pub mod modify_gamestate;

#[macro_use]
extern crate lazy_static;

// type that represents a connecting client
#[derive(Debug, Clone)]
pub struct Client {
    pub client_id: String,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
    pub last_position: Coordinate,
}

pub type Coordinate = [u64; 2];
pub type Clients = Arc<Mutex<HashMap<String, Client>>>;

static SIZE: u64 = 2048;

lazy_static! {
    pub static ref WORLD: Mutex<Grid> = Mutex::new(Grid::new(SIZE, SIZE));
    pub static ref CLIENTS: Clients = Arc::new(Mutex::new(HashMap::new()));
}

