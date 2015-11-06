use std::vec::Vec;
use graphics::Transformed;
use graphics::math::{ Scalar, Matrix2d };
use gfx_graphics::{ GfxGraphics };
use gfx_device_gl::{ Resources, CommandBuffer, Output };
use rand::{ self, Rng };

use input_state::InputState;
use player::{ self, Player };
use ball::{ self, Ball };
use object::{ self, Object, Sort };
use locus_ball::LocusBall;
use traits::Circle;

const OBJECT_FREQUECY: i32 = 80;
const LOCUS_FREQUENCY: i32 = 3;
const MIN_DISTANCE: f64 = 150.0; // Minimal distance of ball and spawn position of object
const OBJECT_RANDOM_LIMIT: i32 = 100;

pub struct Game {
    context: Context,
    bar: Player,
    ball: Ball,
    loci: Vec<LocusBall>,
    objects: Vec<Object>,
    count: i32,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        let mut con = Context::new(width, height);
        Game {
            context: con.clone(),
            bar: Player::new(con.clone()),
            ball: Ball::new(con.clone()),
            loci: Vec::new(),
            objects: Vec::new(),
            count: 0,
        }
    }

    pub fn initialize(&self) -> Self {
        Self::new(self.context.width, self.context.height)
    }

    pub fn draw(&self, t: Matrix2d, g: &mut GfxGraphics<Resources, CommandBuffer<Resources>, Output>) {
        self.bar.draw(t, g);
        for o in self.objects.iter() {
            o.draw(t, g);
        }
        for l in self.loci.iter() {
            l.draw(t, g);
        }
        self.ball.draw(t, g);
    }

    pub fn update(&mut self, input_state: &InputState) {
        self.count += 1;

        for op in input_state.vec().iter() {
            self.bar.press(op);
        }

        for o in self.objects.iter_mut() {
            if ! o.hit && o.is_hit(&self.ball) {
                o.hit(&mut self.bar, &mut self.ball);
            }
            o.update();
        }
        self.objects.retain(|o| o.is_retained());

        for l in self.loci.iter_mut() {
            l.update();
        }
        self.loci.retain(|l| l.is_retained());

        self.bar.update(&self.context);
        self.ball.update(&self.context, &self.bar);

        // Add objects
        let mut rng = rand::thread_rng();
        if rng.gen_range(0, OBJECT_FREQUECY) == 0 {
            // TODO Regenerate if it already hits to ball
            let sort = match rng.gen_range(0, 9) {
                0 => Sort::Fall,
                1 => Sort::Reflect,
                2 => Sort::ReflectV,
                3 => Sort::ReflectH,
                4 => Sort::BallSize,
                5 => Sort::BarLength,
                6 => Sort::Warp,
                7 => Sort::Accelerator,
                8 => Sort::Random,
                _ => panic!(),
            };
            let r = rng.gen_range(object::MIN_R, object::MAX_R);
            for i in 0..OBJECT_RANDOM_LIMIT {
                let pos = [rng.gen_range(0.0, self.context.width as Scalar) as f64, rng.gen_range(0.0, self.context.height as Scalar * player::DEFAULT_Y - ball::DEFAULT_R)];
                if ! self.ball.is_hit(&(pos, r + MIN_DISTANCE)) {
                    self.objects.push(Object::new(pos, r, sort));
                    break;
                }
            }
        }

        // Add locus
        if self.count % LOCUS_FREQUENCY == 0 {
            self.loci.push(self.ball.get_locus_ball());
        }
    }

    pub fn is_ended(&self) -> bool {
        self.ball.is_ended()
    }
}

impl Circle for ([f64; 2], f64) {
    fn get_x(&self) -> f64 {
        self.0[0]
    }

    fn get_y(&self) -> f64 {
        self.0[1]
    }

    fn get_r(&self) -> f64 {
        self.1
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
