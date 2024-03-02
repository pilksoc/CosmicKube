use serde::{Deserialize, Serialize};
use serde_json::json;
// these are the structs intended for use when communicating via websockets

// this is the data we expect to recieve from the player
#[derive(Serialize, Deserialize)]
pub struct PlayerInfo {
    player: String, //Player, //the player requesting the data
    coordinates: [u64; 2], //current player coordinates
    action: String, //PLACEHOLDER! we need to know what the player is doing.
}

// this is the data we expect to send to the player
// pub struct GameState {
//     grid: String, //PLACEHOLDER! This will be the partial grid state type
// }


pub fn create_response(message: &str) -> String {

    // Parse the string of data into serde_json::Value.
    let info: PlayerInfo = serde_json::from_str(message).expect("something went wrong in json parse");

    // debug: log of event to server console
    println!("{}: {} @ (x:{}, y:{})", info.player, info.action, info.coordinates[0], info.coordinates[1]);

    let resp = json!({
        "grid" : "edited grid"
    });

   resp.to_string()
}

