use macroquad::prelude::*;

use crate::CIRCLE_RADIUS;

pub fn draw_common(
    circle: &mut crate::Shape,
    squares: &mut Vec<crate::Shape>,
    bullets: &mut Vec<crate::Shape>,
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
}
