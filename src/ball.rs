use graphics::math::{ Scalar, Matrix2d };
use piston_window::{ G2d };
use num;
use player;

use locus_ball::LocusBall;
use game::Context;
use traits::Circle;

pub const DEFAULT_VEC: [Scalar; 2] = [2.4, 2.4];
pub const VEC_RANGE: Scalar = 1.0;
pub const DEFAULT_R: Scalar = 28.0;
pub const R_RANGE: Scalar = 40.0;
const DEFAULT_Y: Scalar = 0.05;
const BIAS: Scalar = 30.0;
const R_TRANS_STEP: i32 = 80;
const RAINBOW_CYCLE: i32 = 100;
const END_HEIGHT: f64 = 1.3;

pub struct Ball {
    pub pos: [Scalar; 2],
    pub vec: [Scalar; 2],
    pub r: Scalar,
    pub r_trans: Scalar,
    pub r_trans_step: i32,
    end: bool,
    color: [f32; 4],
    collide_count: i32, // Score
    count: i32,
}

impl Ball {
    pub fn new(con: Context) -> Self {
        Ball {
            pos: [con.width as Scalar / 2.0, con.height as Scalar * DEFAULT_Y],
            vec: DEFAULT_VEC,
            r: DEFAULT_R,
            r_trans: 0.0,
            r_trans_step: 0,
            end: false,
            color: [1.0, 0.0, 0.0, 1.0],
            collide_count: 0,
            count: 0,
        }
    }

    pub fn get_locus_ball(&self) -> LocusBall {
        LocusBall::new(self.pos, self.r, self.color)
    }

    pub fn is_ended(&self) -> bool {
        self.end
    }

    fn transform(&mut self) {
        if self.r_trans_step != 0 {
            self.r_trans_step -= 1;
            self.r += (self.r_trans - self.r) / R_TRANS_STEP as Scalar;
        }
    }

    pub fn transform_r(&mut self, trans: Scalar) {
        self.r_trans_step = R_TRANS_STEP;
        self.r_trans = trans;
    }

    pub fn update(&mut self, con: &Context, bar: &player::Player) -> &mut Self {
        self.count += 1;
        self.transform();

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
        if self.pos[1] + self.r > con.height as Scalar * END_HEIGHT {
            self.end = true;
        }
        // check collide with bar
        if self.pos[0] + self.r + self.r / 2.5 > bar.pos[0] - bar.length / 2.0 &&
           self.pos[0] - self.r - self.r / 2.5 < bar.pos[0] + bar.length / 2.0 {
            if self.pos[1] < bar.pos[1] + player::HEIGHT && self.pos[1] + self.r > bar.pos[1] {
                if self.pos[1] < bar.pos[1] && self.vec[1] > 0.0 {
                    self.collide_count += 1;
                    self.pos[1] = bar.pos[1] - self.r;
                    self.vec[1] = - self.vec[1];
                    self.vec[0] += ((self.pos[0] - bar.pos[0]) / BIAS) - bar.vec[0];
                    self.vec[1] += bar.vec[1] / 2.0;
                }
            }
        }
        // rainbow
        let x = (self.count - self.count / RAINBOW_CYCLE * RAINBOW_CYCLE) as f32 / RAINBOW_CYCLE as f32;
        self.color = match self.count / RAINBOW_CYCLE % 6 {
            0 => [1.0    , x      , 0.0    , 1.0],
            1 => [1.0 - x,     1.0, 0.0    , 1.0],
            2 => [0.0    ,     1.0, x      , 1.0],
            3 => [0.0    , 1.0 - x,     1.0, 1.0],
            4 => [x      , 0.0    ,     1.0, 1.0],
            5 => [    1.0, 0.0    , 1.0 - x, 1.0],
            _ => [1., 1., 1., 1.],
        };
        self
    }

    pub fn draw(&self, t: Matrix2d, g: &mut G2d) {
        use figure;
        figure::circle(self.pos[0], self.pos[1], self.r, self.color, t, g);
    }
}

impl Circle for Ball {
    fn get_x(&self) -> f64 {
        self.pos[0]
    }

    fn get_y(&self) -> f64 {
        self.pos[1]
    }

    fn get_r(&self) -> f64 {
        self.r
    }
}
