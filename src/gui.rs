use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use ggez::graphics;
use ggez::{Context, GameResult};

use input::Buttons;

#[derive(Debug)]
pub enum WidgetEvent {
    ButtonEvent(ButtonEvent),
}

#[derive(Debug)]
pub enum ButtonEvent {
    Clicked,
}

pub trait Renderable {
    fn render(&self, ctx: &mut Context) -> GameResult<()>;
}

pub trait Widget: Renderable {
    fn interact(&self, interaction: Buttons, messages: &mut VecDeque<WidgetEvent>);
}

pub struct GuiManager {
    pub widgets: Vec<Box<Widget>>,
}

impl GuiManager {
    pub fn new() -> GuiManager {
        GuiManager {
            widgets: Vec::new(),
        }
    }

    pub fn interact(&self, interaction: Buttons) -> VecDeque<WidgetEvent> {
        let mut messages: VecDeque<WidgetEvent> = VecDeque::new();

        for widget in self.widgets.iter() {
            widget.interact(interaction, &mut messages);
        }

        messages
    }

    pub fn update(&self) {}

    pub fn render(&self, ctx: &mut Context) -> GameResult<()> {
        for widget in self.widgets.iter() {
            widget.render(ctx)?;
        }

        Ok(())
    }
}

pub struct Button {
    x: f32,
    y: f32,
    text: String,
}

impl Button {
    pub fn new(x: f32, y: f32, text: String) -> Box<Button> {
        Box::new(Button { x, y, text })
    }
}

impl Renderable for Button {
    fn render(&self, ctx: &mut Context) -> GameResult<()> {
        let text = graphics::TextCached::new(self.text.clone())?;
        text.queue(ctx, graphics::Point2::new(self.x, self.y), None);

        Ok(())
    }
}

impl Widget for Button {
    fn interact(&self, interaction: Buttons, messages: &mut VecDeque<WidgetEvent>) {
        messages.push_back(WidgetEvent::ButtonEvent(ButtonEvent::Clicked));
    }
}
