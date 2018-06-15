use std::collections::VecDeque;
use std::rc::Rc;

use ggez::graphics;
use ggez::{Context, GameResult};

use gui::{Renderable, Widget, WidgetEvent};
use input::Buttons;
use state::Store;

pub struct Menu {
    menu_options: Vec<String>,
    x: f32,
    y: f32,
    spacing: f32,
    store: Rc<Store>,
}

impl Menu {
    pub fn new(x: f32, y: f32, spacing: f32, store: Rc<Store>) -> Box<Menu> {
        Box::new(Menu {
            menu_options: Vec::new(),
            x,
            y,
            spacing,
            store,
        })
    }
}

impl Renderable for Menu {
    fn render(&self, ctx: &mut Context) -> GameResult<()> {
        let text = graphics::TextCached::new("MenuWidget")?;
        text.queue(ctx, graphics::Point2::new(self.x, self.y), None);

        Ok(())
    }
}

impl Widget for Menu {
    fn interact(&self, interaction: Buttons, messages: &mut VecDeque<WidgetEvent>) {}
}
