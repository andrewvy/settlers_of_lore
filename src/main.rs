#![windows_subsystem = "windows"]

extern crate ggez;

use std::env;
use std::path;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;
use ggez::ContextBuilder;

mod app;
mod assets;
mod input;
mod menu;
mod screen;
mod state;

use app::AppState;

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

    let state = &mut AppState::new(ctx).unwrap();

    event::run(ctx, state).unwrap();
}
