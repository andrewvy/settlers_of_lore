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
use widgets;

pub struct AppState {
    assets: Assets,
    screen: Screen,
    gui_manager: GuiManager,
    input_binding: InputBinding,
    controller_state: ControllerState,
    sprite_batch: spritebatch::SpriteBatch,
    store: Rc<Store>,
    pub asset_store: warmy::Store<Context>,
}

impl AppState {
    pub fn new(resource_dir: Option<path::PathBuf>, ctx: &mut Context) -> GameResult<AppState> {
        let screen = Screen::new(ctx)?;
        let assets = Assets::new(ctx, &screen)?;
        let input_binding = InputBinding::new();
        let controller_state = ControllerState::new();

        let resource_pathbuf: path::PathBuf = match resource_dir {
            Some(s) => s,
            None => ctx.filesystem.get_resources_dir().to_owned(),
        };

        let opt = warmy::StoreOpt::default().set_root(resource_pathbuf);
        let mut asset_store = warmy::Store::new(opt).expect("No asset store?");

        let tileset = asset_store
            .get::<_, assets::Image>(&warmy::FSKey::new("cb_temple_b.png"), ctx)
            .unwrap();

        let sprite_batch = spritebatch::SpriteBatch::new((tileset.borrow().0).clone());

        let mut gui_manager = GuiManager::new();

        let store = Store::new();

        gui_manager
            .widgets
            .push(widgets::menu::Menu::new(0.0, 150.0, 50.0, store.clone()));

        Ok(AppState {
            asset_store,
            assets,
            controller_state,
            gui_manager,
            input_binding,
            screen,
            sprite_batch,
            store,
        })
    }
}

impl event::EventHandler for AppState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {}

        if self.controller_state.get_button_pressed(Buttons::Action) {
            let gui_events = self.gui_manager.interact(Buttons::Action);

            println!("{:?}", gui_events);
        }

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
            self.screen.to_screen_coordinates(Point2::new(5.0, 0.0)),
            None,
        );

        TextCached::draw_queued(ctx, DrawParam::default())?;

        let p = DrawParam {
            src: Rect::new(
                (1.0 / 32.0) * 1.0,
                (1.0 / 35.0) * 1.0,
                (1.0 / 32.0) * 4.0,
                (1.0 / 35.0) * 4.0,
            ),
            dest: Point2::new(6.0, 32.0),
            scale: Point2::new(2.0, 2.0),
            color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
            ..Default::default()
        };

        self.sprite_batch.add(p);

        graphics::draw(ctx, &self.sprite_batch, Point2::new(0.0, 0.0), 0.0)?;

        self.sprite_batch.clear();
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
