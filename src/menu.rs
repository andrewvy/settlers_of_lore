use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};

pub struct Menu {
    id: i32,
    selected_menu_item_index: usize,
    menu_items: Vec<MenuItem>,
}

impl Menu {
    pub fn render(&self, ctx: &mut Context) -> GameResult<()> {
        for (i, item) in self.menu_items.iter().enumerate() {
            item.render(ctx, i == self.selected_menu_item_index)?;
        }

        graphics::TextCached::draw_queued(ctx, graphics::DrawParam::default())?;

        Ok(())
    }

    fn up(&mut self) {
        if self.selected_menu_item_index > 0 {
            self.selected_menu_item_index -= 1;
        }
    }

    fn down(&mut self) {
        let len = self.menu_items.len();
        if self.selected_menu_item_index < (len - 1) {
            self.selected_menu_item_index += 1;
        }
    }

    fn select(&mut self) {
        match self.menu_items.get(self.selected_menu_item_index) {
            Some(_menu_item) => println!("Selected: {}", self.selected_menu_item_index),
            None => {}
        }
    }
}

pub struct MenuItem {
    x: f32,
    y: f32,
    text: String,
}

impl MenuItem {
    pub fn render(&self, ctx: &mut Context, selected: bool) -> GameResult<()> {
        if selected {
            graphics::set_color(ctx, Color::new(0.8, 0.8, 0.8, 1.0))?;
        } else {
            graphics::set_color(ctx, Color::new(0.5, 0.5, 0.5, 1.0))?;
        }

        let text = graphics::TextCached::new(self.text.clone())?;
        text.queue(ctx, graphics::Point2::new(self.x, self.y), None);

        Ok(())
    }
}

pub struct MenuState {
    active_menu_stack: Vec<i32>,
    menus: Vec<Menu>,
}

impl MenuState {
    pub fn new() -> MenuState {
        MenuState {
            active_menu_stack: vec![0],
            menus: vec![Menu {
                id: 0,
                selected_menu_item_index: 0,
                menu_items: vec![
                    MenuItem {
                        x: 50.0,
                        y: 150.0,
                        text: "Start".to_string(),
                    },
                    MenuItem {
                        x: 50.0,
                        y: 170.0,
                        text: "Settings".to_string(),
                    },
                ],
            }],
        }
    }

    pub fn up(&mut self) {
        match self.get_active_menu_mut() {
            Some(menu) => menu.up(),
            None => {}
        }
    }

    pub fn down(&mut self) {
        match self.get_active_menu_mut() {
            Some(menu) => menu.down(),
            None => {}
        }
    }

    pub fn select(&mut self) {
        match self.get_active_menu_mut() {
            Some(menu) => menu.select(),
            None => {}
        }
    }

    pub fn render(&self, ctx: &mut Context) -> GameResult<()> {
        match self.get_active_menu() {
            Some(menu) => menu.render(ctx)?,
            None => {}
        }

        Ok(())
    }

    fn get_active_menu(&self) -> Option<&Menu> {
        if let Some(active_menu_id) = self.active_menu_stack.first() {
            self.menus.iter().find(|menu| menu.id == *active_menu_id)
        } else {
            None
        }
    }

    fn get_active_menu_mut(&mut self) -> Option<&mut Menu> {
        if let Some(active_menu_id) = self.active_menu_stack.first() {
            self.menus
                .iter_mut()
                .find(|menu| menu.id == *active_menu_id)
        } else {
            None
        }
    }
}
