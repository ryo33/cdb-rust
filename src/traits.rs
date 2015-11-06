pub trait Circle {
    fn get_x(&self) -> f64;
    fn get_y(&self) -> f64;
    fn get_r(&self) -> f64;

    fn is_hit<C: Circle>(&self, other: &C) -> bool {
        let x = self.get_x() - other.get_x();
        let y = self.get_y() - other.get_y();
        let r = self.get_r() + other.get_r();
        x * x + y * y <= r * r
    }
}
