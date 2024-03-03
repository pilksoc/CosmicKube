use serde::{Deserialize, Serialize};
use crate::Coordinate;
use crate::grid::Grid;
use crate::space::Space;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalGrid {
    spaces: Vec<Space>,
}
impl LocalGrid {
    pub fn from_grid_and_coord(grid: &Grid, coordinate: Coordinate, local_size: u64) -> LocalGrid {
        let mut spaces: Vec<Space> = Vec::new();
        for space in grid.get_neighbours_n_away(coordinate, local_size) {
            spaces.push(space.clone())
        }
        LocalGrid { spaces }
    }
}
