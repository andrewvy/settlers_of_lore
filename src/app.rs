use std::path;
use std::rc::Rc;

use ggez::event::{self, Keycode, Mod};
use ggez::graphics::{self, spritebatch, Color, DrawParam, Point2, Rect, TextCached, TextFragment};
use ggez::timer;
use ggez::{Context, GameResult};
use warmy;

use assets::{self, Assets};
use gui::GuiManager;
use input::{Buttons, ControllerState, InputBinding};
use screen::Screen;
use state::Store;
use tilemap::TileMap;
use widgets;

pub struct AppState {
    assets: Assets,
    screen: Screen,
    gui_manager: GuiManager,
    input_binding: InputBinding,
    controller_state: ControllerState,
    store: Rc<Store>,
    tilemap: TileMap,
}

impl AppState {
    pub fn new(resource_dir: Option<path::PathBuf>, ctx: &mut Context) -> GameResult<AppState> {
        let screen = Screen::new(ctx)?;
        let mut assets = Assets::new(resource_dir, ctx, &screen)?;
        let input_binding = InputBinding::new();
        let controller_state = ControllerState::new();

        let mut gui_manager = GuiManager::new();

        let store = Store::new();

        gui_manager
            .widgets
            .push(widgets::menu::Menu::new(0.0, 150.0, 50.0, store.clone()));

        let mut tilemap = TileMap::new("/images/cb_temple_b.png", screen, &mut assets.asset_store, ctx);
        tilemap.generate();

        Ok(AppState {
            assets,
            controller_state,
            gui_manager,
            input_binding,
            screen,
            store,
            tilemap,
        })
    }
}

impl event::EventHandler for AppState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            if self.controller_state.get_button_pressed(Buttons::Action) {
                let gui_events = self.gui_manager.interact(Buttons::Action);

                println!("{:?}", gui_events);
            }

            self.controller_state.update();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_color(ctx, Color::new(0.0, 0.0, 0.0, 1.0))?;
        graphics::draw(ctx, &self.tilemap.batch, Point2::new(0.0, 0.0), 0.0)?;

        let fps = timer::get_fps(ctx);
        let fps_display = TextCached::new(TextFragment {
            text: format!("FPS: {}", fps),
            font_id: Some(self.assets.font.clone().into()),
            scale: Some(self.assets.default_scale),
            ..Default::default()
        })?;

        fps_display.queue(
            ctx,
            self.screen.to_screen_coordinates(Point2::new(5.0, 0.0)),
            None,
        );

        TextCached::draw_queued(ctx, DrawParam::default())?;

        self.gui_manager.render(ctx)?;

        graphics::present(ctx);
        timer::yield_now();

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool,
    ) {
        if let Some(button) = self.input_binding.resolve(keycode) {
            self.controller_state.button_down(button);
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if let Some(button) = self.input_binding.resolve(keycode) {
            self.controller_state.button_up(button);
        }
    }
}
