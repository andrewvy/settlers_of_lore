use std::collections::VecDeque;

pub enum Action {
    Menus(MenuAction),
}

pub enum MenuAction {
    // PushActiveMenuId(i32),
    Up,
    Down,
    // Select,
    // SetActiveMenuItemIndex(i32),
}

pub struct Store {
    queue: VecDeque<Action>,
    pub selected_menu_item_index: i32,
    pub menu_stack: Vec<i32>,
}

impl Store {
    pub fn new() -> Store {
        Store {
            queue: VecDeque::new(),
            selected_menu_item_index: 0,
            menu_stack: vec![0],
        }
    }

    pub fn dispatch(&mut self, action: Action) {
        self.queue.push_back(action);
    }

    pub fn update(&mut self) {
        while let Some(action) = self.queue.pop_front() {
            match action {
                Action::Menus(ref menu_action) => match *menu_action {
                    // MenuAction::PushActiveMenuId(menu_id) => {
                    //     self.menu_stack.push(menu_id);
                    // }
                    // MenuAction::SetActiveMenuItemIndex(item_index) => {
                    //     self.selected_menu_item_index = item_index;
                    // }
                    MenuAction::Up => {
                        self.selected_menu_item_index -= 1;
                    }
                    MenuAction::Down => {
                        self.selected_menu_item_index += 1;
                    } // MenuAction::Select => {}
                },
            }
        }
    }
}
