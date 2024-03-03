use cosmic_kube::kube::Kube;
use cosmic_kube::local_grid::LocalGrid;
use cosmic_kube::player::Player;
use cosmic_kube::space::{Space, SpaceKind};
use cosmic_kube::Coordinate;
use core::fmt;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_repr::{Deserialize_repr, Serialize_repr};
use cosmic_kube::WORLD;

//example valid json:
// { "initialised": true, "player": "charlie zoot", "coordinates": [10, 10], "action": { "kind": 1, "kube": { "id": {"uuid": "f7993723-2529-50c4-950d-ba104d29b5df" }, "name": "dirt" }, "coordinates": [10,11] } }


// this is the data we expect to recieve from the player
#[derive(Serialize, Deserialize)]
pub struct PlayerInfo {
    initialised: bool,
    player: Player,         //Player, //the player requesting the data
    coordinates: [u64; 2],  //current player coordinates
    action: Option<Action>, // 0, block picked up 1, block placed
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
enum ActionType {
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
    kind: ActionType,
    kube: Kube,
    coordinates: Coordinate,
}

fn perform_action(action: Action) {
    let kube_result: SpaceKind;
    match action.kind {
        ActionType::Pickup => kube_result = SpaceKind::EmptySpace,
        ActionType::Place => kube_result = SpaceKind::Kube(action.kube),
    }

    let space_in_question: Space = Space::new(action.coordinates, kube_result);
    WORLD.lock().unwrap().insert(space_in_question);
}

fn debug_message(state: &PlayerInfo) {
    // debug: log of event to server console
    println!(
        "{} @ (x:{}, y:{})",
        state.player.username, state.coordinates[0], state.coordinates[1]
    );
    let mut _has_action: bool = true;
    match &state.action {
        Some(p) => println!("{}: {}", p.kind, p.kube.name),
        None => _has_action = false,
    }
}

fn recalculate_game(state: PlayerInfo) -> String {
    debug_message(&state); //debug

    // store the players location in the world
    let playerspace: Space = Space::new(state.coordinates, SpaceKind::Player(state.player));
    let mut grid = WORLD.lock().unwrap();
    grid.insert(playerspace);

    // then we want to update the grid by performing action
    match state.action {
        Some(p) => perform_action(p),
        _ => (),
    }

    let new_grid: LocalGrid =
        LocalGrid::from_grid_and_coord(&WORLD.lock().unwrap(), state.coordinates, 48);
    let resp: Value;

    if state.initialised {
        // if the player is not new to the game, continue game loop
        resp = json!({
            "grid" : new_grid,
        });
    } else {
        let mut rng = rand::thread_rng();
        resp = json!({
            "coordinates" : [rng.gen_range(0..2048), rng.gen_range(0..2048)]
        });
    }

    resp.to_string()
}

pub fn create_response(message: &str) -> String {
    // Parse the string of data into serde_json::Value.
    let state: PlayerInfo =
        serde_json::from_str(message).expect("something went wrong in json parse");

    recalculate_game(state)
}
