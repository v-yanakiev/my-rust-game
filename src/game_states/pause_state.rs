use macroquad::prelude::*;

use crate::game_states::game_state_module::GameState;

pub fn pause_state_engage(game_state: &mut GameState) {
    if is_key_pressed(KeyCode::Space){
        game_state.unpause();
    }
}
