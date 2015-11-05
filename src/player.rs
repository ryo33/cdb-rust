use graphics::math::{ Scalar, Matrix2d };
use gfx_graphics::{ GfxGraphics };
use gfx_device_gl::{ Resources, CommandBuffer, Output };

use operation::Operation;
use game::Context;

pub const DEFAULT_LENGTH: Scalar = 150.0;
pub const LENGTH_RANGE: Scalar = 200.0;
pub const DEFAULT_Y: Scalar = 0.9;
const MAX_Y: Scalar = 0.9;
const MIN_Y: Scalar = 0.5;
const MOVE: Scalar = 0.4;
const FRICTION: Scalar = 0.95;
pub const HEIGHT: Scalar = 10.0;
const COLOR: [f32; 4] = [0.3, 0.3, 0.3, 1.0];

pub struct Player {
    pub pos: [Scalar; 2],
    pub vec: [Scalar; 2],
    pub length: Scalar,
}

impl Player {
    pub fn new(con: Context) -> Self {
        Player {
            pos: [(con.width / 2) as Scalar, con.height as f64 * DEFAULT_Y],
            vec: [0.0, 0.0],
            length: DEFAULT_LENGTH,
        }
    }

    pub fn update(&mut self, con: &Context) -> &mut Self {
        self.pos = [self.pos[0] + self.vec[0], self.pos[1] + self.vec[1]];
        for v in &mut self.vec {
            *v = *v * FRICTION;
        }

        if self.pos[1] < con.height as f64 * MIN_Y {
            self.pos[1] = con.height as f64 * MIN_Y;
        }

        if self.pos[1] > con.height as f64 * MAX_Y {
            self.pos[1] = con.height as f64 * MAX_Y;
        }
        self
    }

    pub fn press(&mut self, op: &Operation) {
        use operation::{ Operation, Direction, };
        match op {
            &Operation::Move(Direction::Left) => {
                self.vec = [self.vec[0] - MOVE, self.vec[1]];
            },
            &Operation::Move(Direction::Right) => {
                self.vec = [self.vec[0] + MOVE, self.vec[1]];
            },
            &Operation::Move(Direction::Up) => {
                self.vec = [self.vec[0], self.vec[1] - MOVE];
            },
            &Operation::Move(Direction::Down) => {
                self.vec = [self.vec[0], self.vec[1] + MOVE];
            },
            _ => {},
        }
    }

    pub fn draw(&self, t: Matrix2d, g: &mut GfxGraphics<Resources, CommandBuffer<Resources>, Output>) {
        use figure;
        figure::rect(self.pos[0] - self.length / 2.0, self.pos[1], self.length, HEIGHT, COLOR, t, g);
    }
}
