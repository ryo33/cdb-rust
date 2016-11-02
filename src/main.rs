extern crate crux;
extern crate piston_window;
extern crate gfx_graphics;
extern crate gfx_device_gl;
extern crate graphics;
extern crate sprite;
extern crate find_folder;
extern crate num;
extern crate rand;

use game_loop::GameLoopSettings;

mod game_loop;
mod state;
mod action;
mod figure;
mod traits;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    let mut game_loop = GameLoopSettings::new()
        .window_size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .internal_size(640, 480)
        .color([0.1, 0.13, 0.1, 1.0])
        .title("CrazyDancingBall").get_game();
    game_loop.run();
}
