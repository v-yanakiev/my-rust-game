use macroquad::prelude::*;

use crate::game_states::{game_over::game_over_state, game_state_module::GameState, pause_state::pause_state_engage, playing::PlayingState};
mod game_states;

const MOVEMENT_SPEED: f32 = 200.0;
const CIRCLE_RADIUS: f32 = 16.0;

#[macroquad::main("My game")]
async fn main() {
    let mut game_state: GameState = GameState::Playing(PlayingState { score: 0 });
    rand::srand(miniquad::date::now() as u64);
    let mut squares: Vec<Shape> = vec![];
    let mut circle = Shape {
        size: CIRCLE_RADIUS,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        collided: false,
    };
    let mut bullets: Vec<Shape> = vec![];

    loop {
        game_states::common::draw_common(&mut circle, &mut squares, &mut bullets);
        
        match game_state {
            GameState::Playing(_) => {
                game_states::playing::playing_state(
                    &mut circle,
                    &mut squares,
                    &mut bullets,
                    &mut game_state,
                );
            }
            GameState::MainMenu => todo!(),
            GameState::Paused(_) => {
                pause_state_engage(&mut game_state);
            },
            GameState::GameOver(_) => {
                game_over_state(&mut circle, &mut squares, &mut bullets, &mut game_state)
            }
        }

        next_frame().await;
    }
}

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    collided: bool,
}

impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }
    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}
