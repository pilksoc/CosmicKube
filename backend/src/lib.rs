use std::sync::Mutex;

use crate::grid::Grid;

pub mod grid;
pub mod space;
pub mod kube;
pub mod player;
pub mod local_grid;
pub mod cache_client;
pub mod recipe;

#[macro_use]
extern crate lazy_static;

pub type Coordinate = [u64; 2];

static SIZE: u64 = 2048;

lazy_static! {
    pub static ref WORLD: Mutex<Grid> = Mutex::new(Grid::new(SIZE, SIZE));
}

