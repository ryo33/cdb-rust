use piston_window::{ Button, Key };
use std::vec::Vec;
use action::*;

#[derive(Clone)]
pub struct InputState {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            left: false, right: false, up: false, down: false,
        }
    }

    pub fn reduce(&mut self, action: GameAction) {
        match action {
            GameAction::Press(button) =>
                self.update(button, true),
            GameAction::Release(button) =>
                self.update(button, false),
            _ => {}
        }
    }

    fn update(&mut self, button: Button, value: bool) {
        match get_operation(button) {
            Operation::Left => self.left = value,
            Operation::Right => self.right = value,
            Operation::Up => self.up = value,
            Operation::Down => self.down = value,
            _ => {}
        }
    }

    pub fn vec(&self) -> Vec<Operation> {
        let mut states = Vec::new();
        if self.left { states.push(Operation::Left); }
        if self.right { states.push(Operation::Right); }
        if self.up { states.push(Operation::Up); }
        if self.down { states.push(Operation::Down); }
        states
    }
}

fn get_operation(button: Button) -> Operation {
    match button {
        Button::Keyboard(key) => match key {
            Key::A => {
                Operation::Left
            },
            Key::D => {
                Operation::Right
            },
            Key::W => {
                Operation::Up
            },
            Key::S => {
                Operation::Down
            },
            Key::Space | Key::Return | Key::Z => {
                Operation::Enter
            },
            Key::X | Key::Backspace => Operation::Cancel,
            _ => Operation::None,
        },
        _ => Operation::None,
    }
}
