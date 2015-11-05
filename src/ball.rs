use graphics::math::{ Scalar, Matrix2d };
use gfx_graphics::{ GfxGraphics };
use gfx_device_gl::{ Resources, CommandBuffer, Output };
use num;
use player;

use game::Context;

pub const DEFAULT_VEC: [Scalar; 2] = [2.0, 2.0];
pub const VEC_RANGE: Scalar = 0.8;
pub const DEFAULT_R: Scalar = 25.0;
pub const R_RANGE: Scalar = 30.0;
const DEFAULT_Y: Scalar = 0.05;
const BIAS: Scalar = 30.0;

pub struct Ball {
    pub pos: [Scalar; 2],
    pub vec: [Scalar; 2],
    pub r: Scalar,
    end: bool,
}

impl Ball {
    pub fn new(con: Context) -> Self {
        Ball {
            pos: [con.width as Scalar / 2.0, con.height as Scalar * DEFAULT_Y],
            vec: DEFAULT_VEC,
            r: DEFAULT_R,
            end: false,
        }
    }

    pub fn is_ended(&self) -> bool {
        self.end
    }

    pub fn update(&mut self, con: &Context, bar: &player::Player) -> &mut Self {
        self.pos = [self.pos[0] + self.vec[0], self.pos[1] + self.vec[1]];
        if self.pos[0] - self.r < 0.0 {
            self.pos[0] = self.r;
            self.vec[0] = num::abs(self.vec[0]) as Scalar;
        }
        if self.pos[1] - self.r < 0.0 {
            self.pos[1] = self.r;
            self.vec[1] = num::abs(self.vec[1]) as Scalar;
        }
        if self.pos[0] + self.r > con.width as Scalar {
            self.pos[0] = con.width as Scalar - self.r;
            self.vec[0] = - num::abs(self.vec[0]) as Scalar;
        }
        if self.pos[1] + self.r > con.height as Scalar {
            self.end = true;
        }
        if self.pos[0] + self.r + self.r / 2.5 > bar.pos[0] - bar.length / 2.0 &&
           self.pos[0] - self.r - self.r / 2.5 < bar.pos[0] + bar.length / 2.0 {
            if self.pos[1] < bar.pos[1] + player::HEIGHT && self.pos[1] + self.r > bar.pos[1] {
                if self.pos[1] < bar.pos[1] && self.vec[1] > 0.0 {
                    self.pos[1] = bar.pos[1] - self.r;
                    self.vec[1] = - self.vec[1];
                    self.vec[0] += ((self.pos[0] - bar.pos[0]) / BIAS) - bar.vec[0];
                    self.vec[1] += bar.vec[1] / 2.0;
                }
            }
        }
        self
    }

    pub fn draw(&self, t: Matrix2d, g: &mut GfxGraphics<Resources, CommandBuffer<Resources>, Output>) {
        use figure;
        figure::circle(self.pos[0], self.pos[1], self.r, [1.0, 1.0, 1.0, 1.0], t, g);
    }
}
