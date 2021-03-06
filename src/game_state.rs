use graphics::math::{ Matrix2d };
use piston_window::{ G2d };

use operation::*;
use input_state::InputState;
use game::Game;

pub struct GameState {
    input_state: InputState,
    count: i32,
    game: Game,
    state: State,
}

impl GameState {
    pub fn new(width: u32, height: u32) -> Self {
        GameState {
            input_state: InputState::new(),
            count: 0,
            game: Game::new(width, height),
            state: State::Title,
        }
    }

    pub fn is_title(&self) -> bool {
        match self.state {
            State::Title => true,
            _ => false,
        }
    }

    pub fn draw(&self, t: Matrix2d, g: &mut G2d) {
        match self.state {
            State::Title => {
            },
            State::Game => {
                self.game.draw(t, g);
            },
            State::Result => {
            },
        }
    }

    pub fn update(&mut self) {
        match self.state {
            State::Title => {
                if self.input_state.enter {
                    self.state = State::Game;
                }
            },
            State::Game => {
                self.game.update(&self.input_state);
                if self.game.is_ended() {
                    self.game = self.game.initialize();
                    self.state = State::Title;
                    // TODO self.state = State::Result;
                }
            },
            State::Result => {
                if self.input_state.enter {
                    self.state = State::Game;
                } else if self.input_state.cancel {
                    self.state = State::Title;
                }
            },
        }
    }

    pub fn press(&mut self, op: Operation) {
        self.input_state.press(op);
    }

    pub fn release(&mut self, op: Operation) {
        self.input_state.release(op);
    }
}

pub enum State {
    Title,
    Game,
    Result,
}
