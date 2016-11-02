use graphics::math::{ Scalar, Matrix2d };
use piston_window::{ G2d };

const REMAIN_TIME: i32 = 330;
const MAX_OPACITY: f32 = 0.3;

#[derive(Clone)]
pub struct LocusBall {
    pos: [Scalar; 2],
    r: Scalar,
    color: [f32; 4],
    count: i32,
}

impl LocusBall {
    pub fn new(pos: [Scalar; 2], r: Scalar, color: [f32; 4]) -> Self {
        LocusBall {
            pos: pos,
            r: r,
            color: color,
            count: REMAIN_TIME,
        }
    }

    pub fn update(&mut self) -> &mut Self {
        self.count -= 1;
        self
    }

    pub fn is_retained(&self) -> bool {
        self.count > 0
    }

    pub fn draw(&self, t: Matrix2d, g: &mut G2d) {
        use figure;
        let mut color = self.color;
        color[3] = MAX_OPACITY * self.count as f32 / REMAIN_TIME as f32;
        figure::circle(self.pos[0], self.pos[1], self.r, color, t, g);
    }
}
