use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};

use state::{Action, MenuAction, Store};

pub struct Menu {
    id: i32,
    menu_items: Vec<MenuItem>,
}

impl Menu {
    pub fn render(&self, store: &Store, ctx: &mut Context) -> GameResult<()> {
        for (i, item) in self.menu_items.iter().enumerate() {
            item.render(store, ctx, i as i32 == store.selected_menu_item_index)?;
        }

        graphics::TextCached::draw_queued(ctx, graphics::DrawParam::default())?;

        Ok(())
    }
}

pub struct MenuItem {
    x: f32,
    y: f32,
    text: String,
}

impl MenuItem {
    pub fn render(&self, _store: &Store, ctx: &mut Context, selected: bool) -> GameResult<()> {
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

pub struct Menus {
    menus: Vec<Menu>,
}

impl Menus {
    pub fn new() -> Menus {
        Menus {
            menus: vec![
                Menu {
                    id: 0,
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
                },
                Menu {
                    id: 1,
                    menu_items: vec![
                        MenuItem {
                            x: 50.0,
                            y: 150.0,
                            text: "Game 1".to_string(),
                        },
                        MenuItem {
                            x: 50.0,
                            y: 170.0,
                            text: "Game 2".to_string(),
                        },
                        MenuItem {
                            x: 50.0,
                            y: 190.0,
                            text: "Game 3".to_string(),
                        },
                    ],
                },
            ],
        }
    }

    pub fn up(&mut self, store: &mut Store) {
        let selected_menu_item_index = store.selected_menu_item_index;

        if selected_menu_item_index > 0 {
            store.dispatch(Action::Menus(MenuAction::Up));
        }
    }

    pub fn down(&mut self, store: &mut Store) {
        let selected_menu_item_index = store.selected_menu_item_index;

        match self.get_active_menu(store) {
            Some(menu) => {
                let len = menu.menu_items.len() as i32;

                if selected_menu_item_index < len - 1 {
                    store.dispatch(Action::Menus(MenuAction::Down));
                }
            }
            None => {}
        }
    }

    pub fn render(&self, store: &Store, ctx: &mut Context) -> GameResult<()> {
        if let Some(menu) = self.get_active_menu(store) {
            menu.render(store, ctx)?;
        }

        Ok(())
    }

    fn get_active_menu(&self, store: &Store) -> Option<&Menu> {
        if let Some(active_menu_id) = store.menu_stack.first() {
            self.menus.iter().find(|menu| menu.id == *active_menu_id)
        } else {
            None
        }
    }
}
