use serde::{Deserialize, Serialize};

use crate::kube;
use crate::player;
use crate::Coordinate;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct Space {
    pub coordinate: Coordinate,
    pub contains: SpaceKind,
}
impl Space {
    pub fn new(coordinate: Coordinate, contains: SpaceKind) -> Space {
        Space {
            coordinate,
            contains,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
#[allow(clippy::module_name_repetitions)]
pub enum SpaceKind {
    Kube(kube::Kube),
    Player(player::Player),
    EmptySpace,
}

