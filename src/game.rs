use graphics::Transformed;
use graphics::math::{ Scalar, Matrix2d };
use gfx_graphics::{ GfxGraphics };
use gfx_device_gl::{ Resources, CommandBuffer, Output };

use operation::*;
use input_state::InputState;
use figure;
use player::Player;
use ball::Ball;

pub struct Game {
    input_state: InputState,
    context: Context,
    bar: Player,
    ball: Ball,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        let mut con = Context::new(width, height);
        Game {
            input_state: InputState::new(),
            context: con.clone(),
            bar: Player::new(con.clone()),
            ball: Ball::new(con.clone()),
        }
    }

    pub fn draw(&self, t: Matrix2d, g: &mut GfxGraphics<Resources, CommandBuffer<Resources>, Output>) {
        self.bar.draw(t, g);
        self.ball.draw(t, g);
    }

    pub fn update(&mut self) {
        for op in self.input_state.vec().iter() {
            self.bar.press(op);
        }
        self.bar.update(&self.context);
        self.ball.update(&self.context);
    }

    pub fn press(&mut self, op: Operation) {
        self.input_state.press(op);
    }

    pub fn release(&mut self, op: Operation) {
        self.input_state.release(op);
    }
}

#[derive(Clone)]
pub struct Context {
    pub width: u32,
    pub height: u32,
}

impl Context {
    pub fn new(width: u32, height: u32) -> Context {
        Context {
            width: width,
            height: height,
        }
    }

    pub fn trans(&self, t: Matrix2d) -> Matrix2d {
        t.trans((self.width / 2) as Scalar, (self.height / 2) as Scalar)
    }
}
