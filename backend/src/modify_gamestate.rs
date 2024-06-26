// methods here are solely for modifying the state of the game!
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{kube::Kube, player::Player, space::{Space, SpaceKind}, Coordinate, WORLD, CLIENTS};

// this is the data we expect to recieve from the player
/// The data to be received from the player's client.
#[derive(Serialize, Deserialize)]
pub struct PlayerInfo {
    /// Whether the player is new to the game.
    pub initialised: bool,
    // The player requesting the data.
    pub player: Player,
    /// Current player coordinates.
    pub coordinates: [u64; 2],
    /// Where the player was previously.
    old_coordinates: Option<[u64; 2]>,
    /// The action the player performed, if any. Represented using 0 for block picked up, 1 for block placed.
    pub action: Option<Action>
}

/// The type of action performed.
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ActionType {
    /// The player has picked up a block.
    Pickup = 0,
    /// The player has placed a block.
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
    println!("Moving player");
    // move the player's position on the grid
    move_player(player_state.old_coordinates, player_state.coordinates, player_state.player).await;

    // then we want to update the grid by performing action
    println!("Performing action");
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

    println!("Removing old position");
    //remove the players old location in the world, if provided
    match old_pos {
        Some(c) => WORLD.lock().await.insert(Space::new(c, SpaceKind::EmptySpace)),
        _ => (),
    }

    println!("Adding new position");
    // store the players location in the world
    let playerspace: Space = Space::new(new_pos, SpaceKind::Player(player));
    WORLD.lock().await.insert(playerspace);

    println!("Updating player position");
    //we now store the player's last known location in the 'active clients' hashmap
    tokio::spawn(
        async move {
            CLIENTS.lock().await
                .entry(player_key)
                .and_modify(|client| client.last_position = new_pos);
            Ok::<(), ()>(())
        }
    );

    println!("Updated that bastard of a player!!");
}

pub async fn remove_player(player_location: Coordinate) {
    WORLD.lock().await.insert(Space::new(player_location, SpaceKind::EmptySpace));
}

// we can write some tests for these methods down here if anyone fancies it
