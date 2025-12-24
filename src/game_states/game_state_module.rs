use crate::game_states::{game_over::GameOverState, playing::PlayingState};

pub enum GameState {
    MainMenu,
    Playing(PlayingState),
    Paused(PlayingState),
    GameOver(GameOverState),
}

impl GameState {
    pub fn to_main_menu(&mut self) {
        if matches!(self, GameState::MainMenu) {
            panic!("Tried to transition from main menu to main menu!");
        }
        *self = GameState::MainMenu;
    }

    pub fn to_game_over(&mut self) {
        let score = match self {
            GameState::Playing(p) => p.score,
            _ => panic!("Tried to end the game while not playing!"),
        };
        *self = GameState::GameOver(GameOverState {
            reached_score: score,
        });
    }
    pub fn pause(&mut self) {
        let score = match self {
            GameState::Playing(p) => p.score,
            _ => panic!("Tried to pause the game while not playing!"),
        };
        *self = GameState::Paused(PlayingState { score });
    }
    pub fn unpause(&mut self) {
        let score = match self {
            GameState::Paused(p) => p.score,
            _ => panic!("Tried to unpause the game while not paused!"),
        };
        *self = GameState::Playing(PlayingState { score });
    }
    pub fn increment_score(&mut self, increment_amount: u32) {
        match self {
            GameState::Playing(p) => {
                p.score += increment_amount;
            }
            _ => panic!("Can't increment score from this state!"),
        };
    }
    pub fn get_score(&self) -> u32 {
        return match self {
            GameState::Playing(p) | GameState::Paused(p) => p.score,
            GameState::GameOver(p) =>p.reached_score,
            _ => panic!("Can't get score from this state!"),
        };
    }
}
