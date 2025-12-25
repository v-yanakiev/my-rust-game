use macroquad::prelude::*;

use crate::game_states::game_state_module::GameState;

pub fn pause_state_engage(game_state: &mut GameState) {
    // Unpause with Space key or a single touch (Started)
    if is_key_pressed(KeyCode::Space) || touches().iter().any(|t| t.phase == TouchPhase::Started){
        game_state.unpause();
    }
}
