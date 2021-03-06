use graphics::math::{ Scalar, Matrix2d };
use piston_window::{ G2d };
use rand::{ self, Rng };

use player;
use ball;
use traits::Circle;

const OPACITY: f32 = 0.6;
const DISAPPEAR_COUNT: i32 = 50;
pub const MAX_R: f64 = 50.0;
pub const MIN_R: f64 = 10.0;

pub struct Object {
    pub pos: [Scalar; 2],
    pub r: Scalar,
    pub sort: Sort,
    pub hit: bool,
    pub dc: i32,
}

impl Object {
    pub fn new(pos: [f64; 2], r: f64, sort: Sort) -> Self {
        Object {
            pos: pos,
            r: r,
            sort: sort,
            hit: false,
            dc: 0,
        }
    }

    pub fn draw(&self, t: Matrix2d, g: &mut G2d) {
        use figure;
        let mut color = self.color();
        color[3] -= OPACITY * 1.0 / DISAPPEAR_COUNT as f32 * self.dc as f32;
        figure::circle(self.pos[0], self.pos[1], self.r, color, t, g);
    }

    pub fn update(&mut self) {
        if self.hit {
            self.dc += 1;
        }
    }

    pub fn is_retained(&self) -> bool {
        if self.dc > DISAPPEAR_COUNT {
            false
        } else {
            true
        }
    }

    pub fn color(&self) -> [f32; 4] {
        match self.sort {
            Sort::Fall => [0.0, 1.0, 0.0, OPACITY],
            Sort::Random => [1.0, 1.0, 1.0, OPACITY],
            Sort::Reflect => [1.0, 0.0, 0.0, OPACITY],
            Sort::ReflectV => [0.0, 0.0, 1.0, OPACITY],
            Sort::ReflectH => [0.0, 1.0, 1.0, OPACITY],
            Sort::BallSize => [1.0, 0.7, 0.9, OPACITY],
            Sort::BarLength => [1.0, 0.0, 1.0, OPACITY],
            Sort::Warp => [1.0, 0.6, 0.0, OPACITY],
            Sort::Accelerator => [1.0, 1.0, 0.0, OPACITY],
        }
    }

    pub fn hit(&mut self, bar: &mut player::Player, ball: &mut ball::Ball) {
        match self.sort {
            Sort::Fall => {
                ball.vec = [rand::thread_rng().gen_range(- ball::DEFAULT_VEC[0], ball::DEFAULT_VEC[0]), ball::DEFAULT_VEC[1]];
            },
            Sort::Random => {
                let mut rng = rand::thread_rng();
                let min = ball::DEFAULT_VEC[0] - ball::VEC_RANGE / 2.0;
                let max = ball::DEFAULT_VEC[0] + ball::VEC_RANGE / 2.0;
                ball.vec = match rng.gen_range(0, 4) {
                    0 => [rng.gen_range(- max, max), - rng.gen_range(min, max)],
                    1 => [rng.gen_range(- max, max), - rng.gen_range(min, max)],
                    2 => [rng.gen_range(- max, max),   rng.gen_range(min, max)],
                    3 => [rng.gen_range(- max, max), - rng.gen_range(min, max)],
                    _ => panic!(),
                }
            },
            Sort::Reflect => {
                for i in 0..2 {
                    ball.vec[i] *= -1.0;
                }
            },
            Sort::ReflectV => {
                ball.vec[1] *= -1.0;
            },
            Sort::ReflectH => {
                ball.vec[0] *= -1.0;
            },
            Sort::BallSize => {
                ball.transform_r(rand::thread_rng().gen_range(ball::DEFAULT_R - ball::R_RANGE / 2.0, ball::DEFAULT_R + ball::R_RANGE / 2.0));
            },
            Sort::BarLength => {
                bar.transform_length(rand::thread_rng().gen_range(player::DEFAULT_LENGTH - player::LENGTH_RANGE / 2.0, player::DEFAULT_LENGTH + player::LENGTH_RANGE / 2.0));
            },
            Sort::Warp => {
                ball.pos = self.pos;
            },
            Sort::Accelerator => {
                let s = if ball.vec[1] > 0.0 { 1.0 } else { -1.0 };
                ball.vec[1] = s * rand::thread_rng().gen_range(ball::DEFAULT_VEC[1] - ball::VEC_RANGE / 2.0, ball::DEFAULT_VEC[1] + ball::VEC_RANGE / 2.0);
            },
        }
        self.hit = true;
    }
}

impl Circle for Object {
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

pub enum Sort {
    Fall,
    Random,
    Reflect,
    ReflectV,
    ReflectH,
    BallSize,
    BarLength,
    Warp,
    Accelerator,
}
