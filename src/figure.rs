use graphics::math::{ Scalar, Matrix2d };
use piston_window::{ G2d };
use graphics;

pub fn circle(x: Scalar, y: Scalar, r: Scalar, color: [f32; 4], t: Matrix2d, g: &mut G2d) {
    graphics::ellipse(color, [x - r, y - r, r * 2., r * 2.], t, g);
}

pub fn rect(x: Scalar, y: Scalar, width: Scalar, height: Scalar, color: [f32; 4], t: Matrix2d, g: &mut G2d) {
    graphics::rectangle(color, [x, y, width, height], t, g);
}
