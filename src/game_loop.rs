use piston_window::*;
use graphics::math::{ Matrix2d, Scalar };
use find_folder;
use crux::Store;

use state::GameState;
use action::*;

const TITLE_WIDTH: u32 = 800;
const TITLE_HEIGHT: u32 = 600;

pub struct GameLoop {
    width: u32,
    height: u32,
    window: PistonWindow,
    background_color: [f32; 4],
    color: [f32; 4],
}

impl GameLoop {
    pub fn new(s: &GameLoopSettings) -> GameLoop {
        let opengl = OpenGL::V3_2;
        let mut window: PistonWindow =
            WindowSettings::new(s.title, (s.window_width, s.window_height))
            .exit_on_esc(true)
            .opengl(opengl)
            .build()
            .unwrap();
        GameLoop {
            width: s.width,
            height: s.height,
            window: window,
            background_color: s.background_color,
            color: s.color,
        }
    }

    pub fn run(&mut self) {
        let title = Texture::from_path(
            &mut *self.window.factory.borrow_mut(),
            &find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap().join("title.png"),
            Flip::None,
            &TextureSettings::new()
            ).unwrap();
        let mut game_state = GameState::new(self.width, self.height);
        let mut store = Store::new(game_state);
        for e in self.window.clone() {
            e.draw_2d(|c, g| {
                use figure;
                clear(self.background_color, g);

                let size = e.size();
                let width_ratio = size.width as f64 / self.width as f64;
                let height_ratio = size.height as f64 / self.height as f64;
                let mut con: Context;
                if width_ratio < height_ratio {
                    con = c.trans(0.0, (size.height as f64 - self.height as f64 * width_ratio) / 2.0).scale(width_ratio, width_ratio)
                } else {
                    con = c.trans((size.width as f64 - self.width as f64 * height_ratio) / 2.0, 0.0).scale(height_ratio, height_ratio)
                }
                let t = con.transform;
                figure::rect(0.0, 0.0, self.width as f64, self.height as f64, self.color, t, g);
                if store.state().is_title() {
                    image(&title, t.scale(self.width as f64 / TITLE_WIDTH as f64, self.height as f64 / TITLE_HEIGHT as f64), g);
                } else {
                    store.state().draw(t, g);
                }
            });
            e.update(|_args| {
                store.dispatch(GameAction::Update);
            });
            e.press(|button| {
                store.dispatch(GameAction::Press(button));
            });
            e.release(|button| {
                store.dispatch(GameAction::Release(button));
            });
        }
    }
}

pub struct GameLoopSettings {
    width: u32,
    height: u32,
    window_width: u32,
    window_height: u32,
    title: &'static str,
    color: [f32; 4],
    background_color: [f32; 4],
}

#[allow(dead_code)]
impl GameLoopSettings {
    pub fn new() -> GameLoopSettings {
        GameLoopSettings {
            width: 1920,
            height: 1080,
            window_width: 960,
            window_height: 540,
            title: "game",
            color: [1.0, 1.0, 1.0, 1.0],
            background_color: [1.0, 1.0, 1.0, 1.0],
        }
    }

    pub fn default() -> GameLoop {
        GameLoopSettings::new().get_game()
    }

    pub fn internal_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn window_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.window_width = width;
        self.window_height = height;
        self
    }

    pub fn title(&mut self, title: &'static str) -> &mut Self {
        self.title = title;
        self
    }

    pub fn color(&mut self, color: [f32; 4]) -> &mut Self {
        self.color = color;
        self
    }

    pub fn background_color(&mut self, background_color: [f32; 4]) -> &mut Self {
        self.background_color = background_color;
        self
    }

    pub fn get_game(&self) -> GameLoop {
        GameLoop::new(self)
    }
}
