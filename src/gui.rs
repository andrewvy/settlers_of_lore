use ggez::{Context, GameResult};
use input::Buttons;

pub trait WidgetEvent {}

pub trait Renderable {
    fn render(&self, ctx: &mut Context) -> GameResult<()>;
}

pub trait Widget: Renderable {
    fn interact(&self, interaction: Buttons);
}

pub struct Panel {
    widgets: Vec<Box<Widget>>,
}

impl Panel {
    pub fn interact(&self, interaction: Buttons) {
        for widget in self.widgets.iter() {
            widget.interact(interaction)
        }
    }

    pub fn update(&self) {}

    pub fn render(&self, ctx: &mut Context) -> GameResult<()> {
        for widget in self.widgets.iter() {
            widget.render(ctx)?;
        }

        Ok(())
    }
}

pub struct GuiManager {
    active_panel: Option<Box<Panel>>,
    panels: Vec<Box<Panel>>,
}

impl GuiManager {
    pub fn new() -> GuiManager {
        GuiManager {
            active_panel: None,
            panels: Vec::new(),
        }
    }

    pub fn interact(&self, interaction: Buttons) {
        for panel in self.panels.iter() {
            panel.interact(interaction);
        }
    }

    pub fn update(&self) {
        for panel in self.panels.iter() {
            panel.update();
        }
    }

    pub fn render(&self, ctx: &mut Context) -> GameResult<()> {
        for panel in self.panels.iter() {
            panel.render(ctx)?;
        }

        Ok(())
    }
}
