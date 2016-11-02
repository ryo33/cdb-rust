use piston_window::Button;

#[derive(Clone)]
pub enum Operation {
    Left,
    Right,
    Up,
    Down,
    Enter,
    Cancel,
    None,
}

#[derive(Clone)]
pub enum GameAction {
    Update,
    Operation(Operation),
    Press(Button),
    Release(Button),
}
