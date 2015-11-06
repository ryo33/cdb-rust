use std::vec::Vec;

use operation::{ Operation, Direction, };

pub struct InputState {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub enter: bool,
    pub cancel: bool,
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            left: false, right: false, up: false, down: false,
            enter: false, cancel: false,
        }
    }

    pub fn press(&mut self, op: Operation) -> &mut Self {
        self.update(op, true)
    }

    pub fn release(&mut self, op: Operation) -> &mut Self {
        self.update(op, false)
    }

    fn update(&mut self, op: Operation, value: bool) -> &mut Self {
        match op {
            Operation::Move(dir) => match dir {
                Direction::Left => self.left = value,
                Direction::Right => self.right = value,
                Direction::Up => self.up = value,
                Direction::Down => self.down = value,
            },
            Operation::Enter => self.enter = value,
            Operation::Cancel => self.cancel = value,
            _ => {}
        }
        self
    }

    pub fn vec(&self) -> Vec<Operation> {
        let mut states = Vec::new();
        if self.left { states.push(Operation::Move(Direction::Left)); }
        if self.right { states.push(Operation::Move(Direction::Right)); }
        if self.up { states.push(Operation::Move(Direction::Up)); }
        if self.down { states.push(Operation::Move(Direction::Down)); }
        if self.enter { states.push(Operation::Enter); }
        if self.cancel { states.push(Operation::Cancel); }
        states
    }
}
