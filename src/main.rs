#![windows_subsystem = "windows"]

extern crate ggez;

use std::env;
use std::path;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, Keycode, Mod, MouseButton, MouseState};
use ggez::graphics::{self, Color, DrawParam, Point2, TextCached, TextFragment};
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};

mod assets;
mod menu;
mod state;
mod window;

use assets::Assets;
use menu::Menus;
use state::Store;
use window::Window;

struct MainState {
    mouse_x: i32,
    mouse_y: i32,
    has_clicked: bool,
    menus: Menus,
    arrow_key_pressed: Option<Keycode>,
    select_key_pressed: bool,
    store: Store,
    assets: Assets,
    window: Window,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let window = Window::new(ctx)?;
        let assets = Assets::new(ctx, &window)?;

        Ok(MainState {
            mouse_x: 0,
            mouse_y: 0,
            has_clicked: false,
            menus: Menus::new(),
            arrow_key_pressed: None,
            select_key_pressed: false,
            store: Store::new(),
            window,
            assets,
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {}

        if let Some(keycode) = self.arrow_key_pressed {
            match keycode {
                Keycode::Up => self.menus.up(&mut self.store),
                Keycode::Down => self.menus.down(&mut self.store),
                _ => {}
            }

            self.arrow_key_pressed = None;
        }

        self.store.update();

        if self.select_key_pressed {
            self.select_key_pressed = false;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::set_color(ctx, Color::new(0.5, 0.5, 0.5, 1.0))?;

        let fps = timer::get_fps(ctx);
        let fps_display = TextCached::new(TextFragment {
            text: format!("FPS: {}", fps),
            font_id: Some(self.assets.font.clone().into()),
            scale: Some(self.assets.default_scale),
            ..Default::default()
        })?;

        let mouse_coords = TextCached::new(TextFragment {
            text: format!("x: {}, y: {}", self.mouse_x, self.mouse_y),
            font_id: Some(self.assets.font.clone().into()),
            scale: Some(self.assets.default_scale),
            ..Default::default()
        })?;

        fps_display.queue(
            ctx,
            self.window.to_screen_coordinates(Point2::new(0.0, 0.0)),
            None,
        );
        mouse_coords.queue(
            ctx,
            self.window.to_screen_coordinates(Point2::new(0.0, 20.0)),
            None,
        );

        TextCached::draw_queued(ctx, DrawParam::default())?;

        self.menus.render(&self.store, ctx)?;

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
