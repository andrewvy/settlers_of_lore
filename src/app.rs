use ggez::event::{self, Keycode, Mod};
use ggez::graphics::{self, Color, DrawParam, Point2, TextCached, TextFragment};
use ggez::timer;
use ggez::{Context, GameResult};

use assets::Assets;
use input::{Buttons, ControllerState, InputBinding};
use menu::Menus;
use screen::Screen;
use state::Store;

pub struct AppState {
    menus: Menus,
    store: Store,
    assets: Assets,
    screen: Screen,
    input_binding: InputBinding,
    controller_state: ControllerState,
}

impl AppState {
    pub fn new(ctx: &mut Context) -> GameResult<AppState> {
        let screen = Screen::new(ctx)?;
        let assets = Assets::new(ctx, &screen)?;
        let input_binding = InputBinding::new();
        let controller_state = ControllerState::new();

        Ok(AppState {
            menus: Menus::new(),
            store: Store::new(),
            screen,
            assets,
            input_binding,
            controller_state,
        })
    }
}

impl event::EventHandler for AppState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {}

        if self.controller_state.get_button_pressed(Buttons::Up) {
            self.menus.up(&mut self.store);
        } else if self.controller_state.get_button_pressed(Buttons::Down) {
            self.menus.down(&mut self.store);
        }

        self.store.update();
        self.controller_state.update();

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

        fps_display.queue(
            ctx,
            self.screen.to_screen_coordinates(Point2::new(0.0, 0.0)),
            None,
        );
        TextCached::draw_queued(ctx, DrawParam::default())?;

        self.menus.render(&self.store, ctx)?;

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
