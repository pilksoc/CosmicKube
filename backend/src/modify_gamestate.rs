// methods here are solely for modifying the state of the game!

use std::fmt;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{kube::Kube, player::Player, space::{Space, SpaceKind}, Coordinate, WORLD, CLIENTS};

// this is the data we expect to recieve from the player
#[derive(Serialize, Deserialize)]
pub struct PlayerInfo {
    pub initialised: bool,
    pub player: Player,         //Player, //the player requesting the data
    pub coordinates: [u64; 2],  //current player coordinates
    old_coordinates: Option<[u64; 2]>, //where the player was previously
    pub action: Option<Action>, // 0, block picked up 1, block placed
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ActionType {
    Pickup = 0,
    Place = 1,
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActionType::Pickup => write!(f, "pickup"),
            ActionType::Place => write!(f, "place"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Action {
    pub kind: ActionType,
    pub kube: Kube,
    pub coordinates: Coordinate,
}

pub async fn modify_gamestate(player_state: PlayerInfo) {
    // move the player's position on the grid
    move_player(player_state.old_coordinates, player_state.coordinates, player_state.player);

    // then we want to update the grid by performing action
    match player_state.action {
        Some(p) => perform_action(p).await,
        None => (),
    }
}

pub async fn perform_action(action: Action) {
    let kube_result: SpaceKind;
    match action.kind {
        ActionType::Pickup => kube_result = SpaceKind::EmptySpace,
        ActionType::Place => kube_result = SpaceKind::Kube(action.kube),
    }

    let space_in_question: Space = Space::new(action.coordinates, kube_result);
    WORLD.lock().await.insert(space_in_question);
    
}

pub async fn move_player(old_pos: Option<[u64; 2]>, new_pos: [u64; 2], player: Player) {

    let player_key = player.uuid.to_string();

    //remove the players old location in the world, if provided
    match old_pos {
        Some(c) => WORLD.lock().await.insert(Space::new(c, SpaceKind::EmptySpace)),
        _ => (),
    }

    // store the players location in the world
    let playerspace: Space = Space::new(new_pos, SpaceKind::Player(player));
    WORLD.lock().await.insert(playerspace);

    //we now store the player's last known location in the 'active clients' hashmap
    CLIENTS.lock().await.entry(player_key).and_modify(|client| client.last_position = new_pos);
}

pub async fn remove_player(player_location: Coordinate) {
    WORLD.lock().await.insert(Space::new(player_location, SpaceKind::EmptySpace));
}

// we can write some tests for these methods down here if anyone fancies it
