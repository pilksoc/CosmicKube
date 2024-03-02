
// these are the structs intended for use when communicating via websockets

// this is the data we expect to recieve from the player
pub struct PlayerInfo {
    player: Player, //the player requesting the data
    coordinates: [u64; 2], //current player coordinates
    action: String, //PLACEHOLDER! we need to know what the player is doing.
}

// this is the data we expect to send to the player
pub struct GameState {
    grid: String, //PLACEHOLDER! This will be the partial grid state type
}

