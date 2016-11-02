use graphics::math::{ Scalar, Matrix2d };
use piston_window::{ G2d, Size };

use action::*;

pub const DEFAULT_LENGTH: Scalar = 150.0;
pub const LENGTH_RANGE: Scalar = 240.0;
const LENGTH_TRANS_STEP: i32 = 200;
pub const DEFAULT_Y: Scalar = 0.95;
const MAX_Y: Scalar = 0.95;
const MIN_Y: Scalar = 0.5;
const MOVE: Scalar = 0.4;
const FRICTION: Scalar = 0.95;
pub const HEIGHT: Scalar = 10.0;
const COLOR: [f32; 4] = [0.8, 0.8, 0.9, 0.8];

#[derive(Clone)]
pub struct Player {
    pub pos: [Scalar; 2],
    pub vec: [Scalar; 2],
    pub length: Scalar,
    pub length_trans: Scalar,
    pub length_trans_step: i32,
}

impl Player {
    pub fn new(width: u32, height: u32) -> Self {
        Player {
            pos: [(width / 2) as Scalar, height as f64 * DEFAULT_Y],
            vec: [0.0, 0.0],
            length: DEFAULT_LENGTH,
            length_trans: 0.0,
            length_trans_step: 0,
        }
    }

    fn transform(&mut self) {
        if self.length_trans_step != 0 {
            self.length_trans_step -= 1;
            self.length += (self.length_trans - self.length) / LENGTH_TRANS_STEP as Scalar;
        }
    }

    pub fn transform_length(&mut self, trans: Scalar) {
        self.length_trans_step = LENGTH_TRANS_STEP;
        self.length_trans = trans;
    }

    pub fn update(&mut self, width: u32, height: u32) -> &mut Self {
        self.transform();
        self.pos = [self.pos[0] + self.vec[0], self.pos[1] + self.vec[1]];
        for v in &mut self.vec {
            *v = *v * FRICTION;
        }

        if self.pos[1] < height as f64 * MIN_Y {
            self.pos[1] = height as f64 * MIN_Y;
            self.vec[1] = 0.;
        }

        if self.pos[1] > height as f64 * MAX_Y {
            self.pos[1] = height as f64 * MAX_Y;
            self.vec[1] = 0.;
        }
        self
    }

    pub fn input(&mut self, op: Operation) {
        match op {
            Operation::Left => {
                self.vec = [self.vec[0] - MOVE, self.vec[1]];
            },
            Operation::Right => {
                self.vec = [self.vec[0] + MOVE, self.vec[1]];
            },
            Operation::Up => {
                self.vec = [self.vec[0], self.vec[1] - MOVE];
            },
            Operation::Down => {
                self.vec = [self.vec[0], self.vec[1] + MOVE];
            },
            _ => {},
        }
    }

    pub fn draw(&self, t: Matrix2d, g: &mut G2d) {
        use figure;
        figure::rect(self.pos[0] - self.length / 2.0, self.pos[1], self.length, HEIGHT, COLOR, t, g);
    }
}
