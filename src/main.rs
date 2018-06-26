#![allow(dead_code)]
#![windows_subsystem = "windows"]

extern crate ggez;
extern crate rand;
extern crate specs;
extern crate warmy;

use std::path;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;
use ggez::ContextBuilder;

mod app;
mod assets;
mod components;
mod entities;
mod gui;
mod input;
mod plantae;
mod processables;
mod screen;
mod state;
mod tilemap;
mod widgets;
mod world;

use app::AppState;

fn main() {
    let mut context_builder = ContextBuilder::new("settlers_of_lore", "vy")
        .window_setup(WindowSetup::default().title("Settlers of Lore"))
        .window_mode(WindowMode::default().dimensions(1024, 720));

    let cargo_path: Option<path::PathBuf> = option_env!("CARGO_MANIFEST_DIR").map(|env_path| {
        let mut res_path = path::PathBuf::from(env_path);
        res_path.push("resources");
        res_path
    });

    if let Some(ref s) = cargo_path {
        context_builder = context_builder.add_resource_path(s);
    }

    let ctx = &mut context_builder.build().unwrap();
    let state = &mut AppState::new(cargo_path, ctx).unwrap();

    event::run(ctx, state).unwrap();
}
