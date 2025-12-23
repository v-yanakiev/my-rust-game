use macroquad::prelude::*;
use crate::{MOVEMENT_SPEED, CIRCLE_RADIUS, Shape};

pub fn playing_state(circle: &mut crate::Shape, squares: &mut Vec<crate::Shape>, bullets: &mut Vec<crate::Shape>) {
    let delta_time = get_frame_time();
    if is_key_down(KeyCode::Right) {
        circle.x += MOVEMENT_SPEED * delta_time;
    }
    if is_key_down(KeyCode::Left) {
        circle.x -= MOVEMENT_SPEED * delta_time;
    }
    if is_key_down(KeyCode::Up) {
        circle.y -= MOVEMENT_SPEED * delta_time;
    }
    if is_key_down(KeyCode::Down) {
        circle.y += MOVEMENT_SPEED * delta_time;
    }
    circle.x = clamp(circle.x, CIRCLE_RADIUS, screen_width() - CIRCLE_RADIUS);
    circle.y = clamp(circle.y, CIRCLE_RADIUS, screen_height() - CIRCLE_RADIUS);

    if rand::gen_range(0, 99) >= 85 {
        let size = rand::gen_range(16.0, 64.0);
        squares.push(Shape {
            size,
            speed: rand::gen_range(200.0, 500.0),
            x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
            y: -size,
            collided: false,
        })
    }
    for square in squares.iter_mut() {
        square.y += square.speed * delta_time;
    }
    squares.retain(|square| square.y < screen_height() + square.size);

    if is_key_pressed(KeyCode::Space) {
        bullets.push(Shape {
            x: circle.x,
            y: circle.y,
            speed: circle.speed * 2.0,
            size: 5.0,
            collided: false,
        })
    }
    for bullet in bullets.iter_mut() {
        bullet.y -= bullet.speed * delta_time;
    }
    bullets.retain(|bullet| bullet.y > 0.0 - bullet.size / 2.0);

    squares.retain(|square| !square.collided);
    bullets.retain(|bullet| !bullet.collided);

    for square in squares.iter_mut(){
        for bullet in bullets.iter_mut(){
            if bullet.collides_with(square){
                bullet.collided=true;
                square.collided=true;
            }
        }
    }
    
    if squares.iter().any(|square| circle.collides_with(square)) {
        let text = "Game over!";
        let text_dimensions = measure_text(text, None, 50, 1.0);
        draw_text(
            text,
            screen_width() / 2.0 - text_dimensions.width / 2.0,
            screen_height() / 2.0,
            50.0,
            RED,
        );
    }
}