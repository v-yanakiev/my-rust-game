use macroquad::prelude::*;

use crate::{CIRCLE_RADIUS, game_states::game_state_module::GameState};

pub fn draw_common(
    circle: &mut crate::Shape,
    squares: &mut Vec<crate::Shape>,
    bullets: &mut Vec<crate::Shape>,
    state: &mut GameState,
) {
    draw_circle(circle.x, circle.y, CIRCLE_RADIUS, YELLOW);

    for square in squares {
        draw_rectangle(
            square.x - square.size / 2.0,
            square.y - square.size / 2.0,
            square.size,
            square.size,
            GREEN,
        );
    }

    for bullet in bullets {
        draw_circle(bullet.x, bullet.y, bullet.size / 2.0, RED);
    }
    let score=state.get_score();
    let text = format!("Score: {score}");
    let text_dimensions = measure_text(&text, None, 50, 1.0);
    draw_text(
        &text,
        screen_width() / 6.0 - text_dimensions.width / 2.0,
        screen_height() / 6.0,
        50.0,
        RED,
    );
}
