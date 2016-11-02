use graphics::math::{Matrix2d};
use piston_window::{ Button, Key };
use piston_window::{G2d};
use crux::State;

use action::*;
use state::InputState;
use state::game::Game;

#[derive(Clone)]
pub enum PageState {
    Title,
    Game,
    Result,
}

#[derive(Clone)]
pub struct GameState {
    input_state: InputState,
    count: i32,
    game: Game,
    state: PageState,
}

impl GameState {
    pub fn new(width: u32, height: u32) -> Self {
        GameState {
            input_state: InputState::new(),
            count: 0,
            game: Game::new(width, height),
            state: PageState::Title,
        }
    }

    pub fn is_title(&self) -> bool {
        match self.state {
            PageState::Title => true,
            _ => false,
        }
    }

    pub fn draw(&self, t: Matrix2d, g: &mut G2d) {
        match self.state {
            PageState::Title => {
            },
            PageState::Game => {
                self.game.draw(t, g);
            },
            PageState::Result => {
            },
        }
    }

    fn update(&mut self) {
        match self.state {
            PageState::Game => {
                for ope in self.input_state.vec() {
                    self.game.input(ope);
                }
                self.game.update();
                if self.game.is_ended() {
                    self.game = self.game.initialize();
                    self.state = PageState::Title;
                    // TODO self.state = PageState::Result;
                }
            },
            _ => {}
        }
    }

    fn input(&mut self, ope: Operation) {
        match ope {
            Operation::Enter => {
                match self.state {
                    PageState::Title => self.state = PageState::Game,
                    PageState::Result => self.state = PageState::Game,
                    _ => {}
                }
            },
            Operation::Cancel => {
                match self.state {
                    PageState::Result => self.state = PageState::Game,
                    _ => {}
                }
            },
            _ => {}
        }
    }
}

impl State for GameState {
    type Action = GameAction;
    fn reduce(&mut self, action: GameAction) {
        self.input_state.reduce(action.clone());
        match action {
            GameAction::Update => self.update(),
            GameAction::Operation(ope) => self.input(ope),
            GameAction::Press(Button::Keyboard(Key::Space)) => self.input(Operation::Enter),
            _ => {}
        }
    }
}
