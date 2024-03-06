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

    modify_gamestate(state).await;

    // The dereferencing looks a little weird. Here's what's going on:
    // Tokio's Mutex when locking returns a MutexGuard.
    // This is the same behaviour as std::sync::Mutex.
    // Thus, we first need to dereference it to get to the actual Grid type,
    // and then send a reference to the Grid type to the LocalGrid constructor.
    let new_grid = LocalGrid::from_grid_and_coord(&(*WORLD.lock().await), player_location, 48);
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

pub async fn create_response(message: &str, client_id: &str) -> String {
    match serde_json::from_str::<PlayerInfo>(message) {
        Ok(info) => recalculate_game(info, client_id).await,
        Err(_) => "Ding Dong!!! your json is WRONG".to_string(),
    }
}
