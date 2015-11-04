use graphics::math::{ Scalar, Matrix2d };
use gfx_graphics::{ GfxGraphics };
use gfx_device_gl::{ Resources, CommandBuffer, Output };
use num;

use game::Context;

const DEFAULT_VEC: [Scalar; 2] = [3.0, 3.0];
const DEFAULT_Y: Scalar = 0.05;
const DEFAULT_R: Scalar = 10.0;

pub struct Ball {
    pub pos: [Scalar; 2],
    pub vec: [Scalar; 2],
    pub r: Scalar,
}

impl Ball {
    pub fn new(con: Context) -> Self {
        Ball {
            pos: [con.width as Scalar / 2.0, con.height as Scalar * DEFAULT_Y],
            vec: DEFAULT_VEC,
            r: DEFAULT_R,
        }
    }

    pub fn update(&mut self, con: &Context) -> &mut Self {
        self.pos = [self.pos[0] + self.vec[0], self.pos[1] + self.vec[1]];
        let w_h = [con.width as Scalar, con.height as Scalar];
        for i in 0..2 {
            if self.pos[i] < 0.0 {
                self.pos[i] = 0.0;
                self.vec[i] = num::abs(self.vec[i]) as Scalar;
            }
            if self.pos[i] > w_h[i] {
                self.pos[i] = w_h[i];
                self.vec[i] = - num::abs(self.vec[i]) as Scalar;
            }
        }
        self
    }

    pub fn draw(&self, t: Matrix2d, g: &mut GfxGraphics<Resources, CommandBuffer<Resources>, Output>) {
        use figure;
        figure::circle(self.pos[0], self.pos[1], self.r, [1.0, 1.0, 1.0, 1.0], t, g);
    }
}
