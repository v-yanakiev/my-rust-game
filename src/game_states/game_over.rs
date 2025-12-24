use macroquad::prelude::*;

use crate::game_states::{game_state_module::GameState, playing::PlayingState};

pub struct GameOverState {
    pub reached_score: u32,
}

pub fn game_over_state(
    circle: &mut crate::Shape,
    squares: &mut Vec<crate::Shape>,
    bullets: &mut Vec<crate::Shape>,
    game_state: &mut GameState,
) {
    let text = "Game over!";
    let text_dimensions = measure_text(text, None, 50, 1.0);
    draw_text(
        text,
        screen_width() / 2.0 - text_dimensions.width / 2.0,
        screen_height() / 2.0,
        50.0,
        RED,
    );
    if is_key_pressed(KeyCode::Space) {
        squares.clear();
        bullets.clear();
        circle.x = screen_width() / 2.0;
        circle.y = screen_height() / 2.0;
        *game_state = GameState::Playing(PlayingState { score: 0 });
    }
}
