extern crate ggez;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, Keycode, Mod, MouseState};
use ggez::graphics::{self, Color, DrawParam, Font, HorizontalAlign as HAlign, Layout, Point2,
                     Rect, Scale, TextCached, TextFragment};
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};

use std::collections::BTreeMap;
use std::env;
use std::f32;
use std::path;

struct MainState {
    texts: BTreeMap<&'static str, TextCached>,
    button: Button,
    mouse_x: i32,
    mouse_y: i32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut texts = BTreeMap::new();

        /*
        let font = Font::new_glyph_font(ctx, "/m5x7.ttf")?;

        let mut text = TextCached::new(TextFragment {
            text: "Hello, world!".to_string(),
            font_id: Some(font.clone().into()),
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
            button: Button {
                color: Color::new(0.5, 0.5, 0.5, 1.0),
                rectangle: Rect {
                    x: 50.0,
                    y: 150.0,
                    w: 150.0,
                    h: 50.0,
                },
                hovered: false,
            },
        })
    }
}

struct Button {
    rectangle: Rect,
    color: Color,
    hovered: bool,
}

impl Button {
    fn update(&mut self, mouse_x: i32, mouse_y: i32) {
        if (((mouse_x as f32) > self.rectangle.x)
            && ((mouse_x as f32) < self.rectangle.x + self.rectangle.w))
            && (((mouse_y as f32) > self.rectangle.y)
                && ((mouse_y as f32) < self.rectangle.y + self.rectangle.h))
        {
            self.hovered = true;
        } else {
            self.hovered = false;
        }
    }

    fn render(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_color(ctx, self.color)?;

        let mut draw_mode = graphics::DrawMode::Line(1.0);

        if self.hovered {
            draw_mode = graphics::DrawMode::Fill;
        }

        graphics::rectangle(ctx, draw_mode, self.rectangle)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {}

        self.button.update(self.mouse_x, self.mouse_y);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

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

        self.button.render(ctx)?;

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
        _keycode: Keycode,
        _keymod: Mod,
        _repeat: bool,
    ) {
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
