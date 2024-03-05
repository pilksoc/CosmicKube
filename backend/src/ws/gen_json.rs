use cosmic_kube::local_grid::LocalGrid;
use cosmic_kube::modify_gamestate::{modify_gamestate, PlayerInfo};
use serde_json::{json, Value};
use cosmic_kube::WORLD;


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

async fn recalculate_game(state: PlayerInfo, id: &str) -> String {
    debug_message(&state); //debug

    let player_initialised = state.initialised;
    let player_location = state.coordinates;

    modify_gamestate(state);

    let new_grid: LocalGrid =
        LocalGrid::from_grid_and_coord(&WORLD.lock().await.unwrap(), player_location, 48);
    let resp: Value;

    if player_initialised {
        // if the player is not new to the game, continue game loop
        resp = json!({
            "grid" : new_grid,
        });
    } else {
        resp = json!({
            "coordinates" : player_location,
            "uuid" : id
        });
    }

    resp.to_string()
}

pub fn create_response(message: &str, client_id: &str) -> String {
    match serde_json::from_str::<PlayerInfo>(message) {
        Ok(info) => recalculate_game(info, client_id),
        Err(_) => "Ding Dong!!! your json is WRONG".to_string(),
    }
}
