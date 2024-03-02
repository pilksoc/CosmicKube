use core::fmt;
use cosmic_kube::kube::Kube;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_repr::{Serialize_repr, Deserialize_repr};

// this is the data we expect to recieve from the player
#[derive(Serialize, Deserialize)]
pub struct PlayerInfo {
    player: String, //Player, //the player requesting the data
    coordinates: [u64; 2], //current player coordinates
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
}

fn recalculate_game(state: PlayerInfo) -> String {
    // debug: log of event to server console
    println!("{} @ (x:{}, y:{})", state.player, state.coordinates[0], state.coordinates[1]);
    match state.action {
        Some(p) => println!("{}: {}", p.kind, p.kube.name),
        None => println!(""),
    }

    //send action to database to get result
    //send position to database to update
    //

    let resp = json!({
        "grid" : "edited grid"
    });

   resp.to_string()
}

pub fn create_response(message: &str) -> String {

    // Parse the string of data into serde_json::Value.
    let state: PlayerInfo = serde_json::from_str(message).expect("something went wrong in json parse");

    recalculate_game(state)
}

