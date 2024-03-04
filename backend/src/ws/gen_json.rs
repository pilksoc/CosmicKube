use cosmic_kube::local_grid::LocalGrid;
use cosmic_kube::modify_gamestate::{modify_gamestate, PlayerInfo};
use rand::Rng;
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

fn recalculate_game(state: PlayerInfo) -> String {
    debug_message(&state); //debug

    let player_initialised = state.initialised;
    let player_location = state.coordinates;

    modify_gamestate(state);

    let new_grid: LocalGrid =
        LocalGrid::from_grid_and_coord(&WORLD.lock().unwrap(), player_location, 48);
    let resp: Value;

    if player_initialised {
        // if the player is not new to the game, continue game loop
        resp = json!({
            "grid" : new_grid,
        });
    } else {
        let mut rng = rand::thread_rng();
        resp = json!({
            //reduced to 20 for debugging purposes, for the live game we should set this back to grid size (2048)
            "coordinates" : [rng.gen_range(0..20), rng.gen_range(0..20)]
        });
    }

    resp.to_string()
}

pub fn create_response(message: &str) -> String {
    match serde_json::from_str::<PlayerInfo>(message) {
        Ok(info) => recalculate_game(info),
        Err(_) => "Ding Dong!!! your json is WRONG".to_string(),
    }
}
