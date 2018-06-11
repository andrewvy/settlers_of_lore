extern crate ggez;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, Keycode, Mod, MouseButton, MouseState};
use ggez::graphics::{self, Color, DrawParam, Point2, TextCached};
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};

mod menu;

use std::collections::BTreeMap;
use std::env;
use std::f32;
use std::path;

use menu::MenuState;

struct MainState {
    texts: BTreeMap<&'static str, TextCached>,
    mouse_x: i32,
    mouse_y: i32,
    has_clicked: bool,
    menu_state: MenuState,
    arrow_key_pressed: Option<Keycode>,
    select_key_pressed: bool,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let texts = BTreeMap::new();

        /*
        let font = Font::new_glyph_font(ctx, "/m5x7.ttf")?;

        let mut text = TextCached::new(TextFragment {
            text: "Hello, world!".to_string(),
            font_id: So_e(font.clone().into()),
            scale: Some(Scale::uniform(48.0)),
            ..Default::default()
        })?;

        text.set_bounds(
            Point2::new(300.0, f32::INFINITY),
            Some(Layout::default().h_align(HAlign::Center)),
        );

        texts.insert("0_hello", text);
        */

        Ok(MainState {
            texts,
            mouse_x: 0,
            mouse_y: 0,
            has_clicked: false,
            menu_state: MenuState::new(),
            arrow_key_pressed: None,
            select_key_pressed: false,
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {}

        if let Some(keycode) = self.arrow_key_pressed {
            match keycode {
                Keycode::Up => {
                    self.menu_state.up();
                }
                Keycode::Down => {
                    self.menu_state.down();
                }
                _ => {}
            }
        }

        if self.select_key_pressed {
            self.menu_state.select();
            self.select_key_pressed = false;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::set_color(ctx, Color::new(0.5, 0.5, 0.5, 1.0))?;

        let fps = timer::get_fps(ctx);
        let fps_display = TextCached::new(format!("FPS: {}", fps))?;

        let mouse_coords = TextCached::new(format!("x: {}, y: {}", self.mouse_x, self.mouse_y))?;

        graphics::draw(ctx, &fps_display, Point2::new(0.0, 0.0), 0.0)?;
        graphics::draw(ctx, &mouse_coords, Point2::new(0.0, 20.0), 0.0)?;

        let mut height = 0.0;
        for (_key, text) in &self.texts {
            text.queue(ctx, Point2::new(0.0, 20.0 + height), None);
            height += 20.0 + text.height(ctx) as f32;
        }

        TextCached::draw_queued(ctx, DrawParam::default())?;

        self.menu_state.render(ctx)?;

        graphics::present(ctx);
        timer::yield_now();

        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _state: MouseState,
        x: i32,
        y: i32,
        _xrel: i32,
        _yrel: i32,
    ) {
        self.mouse_x = x;
        self.mouse_y = y;
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool,
    ) {
        match keycode {
            Keycode::Up | Keycode::Down | Keycode::Left | Keycode::Right => {
                self.arrow_key_pressed = Some(keycode);
            }
            Keycode::Return => {
                self.select_key_pressed = true;
            }
            _ => {}
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Up | Keycode::Down | Keycode::Left | Keycode::Right => {
                self.arrow_key_pressed = None;
            }
            Keycode::Return => {
                self.select_key_pressed = false;
            }
            _ => {}
        }
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        match button {
            MouseButton::Left => {
                self.mouse_x = x;
                self.mouse_y = y;
                self.has_clicked = true;
            }
            _ => {}
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: i32, _y: i32) {
        match button {
            MouseButton::Left => {
                self.has_clicked = false;
            }
            _ => {}
        }
    }
}

fn main() {
    let ctx = &mut ContextBuilder::new("settlers_of_lore", "vy")
        .window_setup(WindowSetup::default().title("Settlers of Lore"))
        .window_mode(WindowMode::default().dimensions(1024, 720))
        .build()
        .unwrap();

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    let state = &mut MainState::new(ctx).unwrap();

    event::run(ctx, state).unwrap();
}
