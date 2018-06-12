use std::collections::HashMap;
use std::hash::Hash;

use ggez::event::Keycode;

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
pub enum Buttons {
    Up,
    Down,
    Left,
    Right,
    Action,
}

#[derive(Debug, Copy, Clone)]
struct ButtonState {
    pressed: bool,
    pressed_last_frame: bool,
}

impl ButtonState {
    pub fn new() -> ButtonState {
        ButtonState {
            pressed: false,
            pressed_last_frame: false,
        }
    }
}

pub struct ControllerState {
    buttons: HashMap<Buttons, ButtonState>,
}

impl ControllerState {
    pub fn new() -> ControllerState {
        ControllerState {
            buttons: HashMap::new(),
        }
    }

    pub fn update(&mut self) {
        for (_button, button_state) in self.buttons.iter_mut() {
            button_state.pressed_last_frame = button_state.pressed;
        }
    }

    pub fn button_down(&mut self, button: Buttons) {
        let f = || ButtonState::new();
        let button_state = self.buttons.entry(button).or_insert_with(f);
        button_state.pressed = true;
    }

    pub fn button_up(&mut self, button: Buttons) {
        let f = || ButtonState::new();
        let button_state = self.buttons.entry(button).or_insert_with(f);
        button_state.pressed = false;
    }

    pub fn get_button_pressed(&self, button: Buttons) -> bool {
        let button_state = self.get_button_state(button);
        button_state.pressed && !button_state.pressed_last_frame
    }

    fn get_button_state(&self, button: Buttons) -> ButtonState {
        let default = ButtonState::new();
        let button_state = self.buttons.get(&button).unwrap_or(&default);
        *button_state
    }
}

pub struct InputBinding {
    bindings: HashMap<Keycode, Buttons>,
}

impl InputBinding {
    pub fn new() -> InputBinding {
        let mut bindings = HashMap::new();

        bindings.insert(Keycode::Up, Buttons::Up);
        bindings.insert(Keycode::Down, Buttons::Down);
        bindings.insert(Keycode::Left, Buttons::Left);
        bindings.insert(Keycode::Right, Buttons::Right);
        bindings.insert(Keycode::Return, Buttons::Action);

        InputBinding { bindings }
    }

    pub fn resolve(&self, keycode: Keycode) -> Option<Buttons> {
        self.bindings.get(&keycode).cloned()
    }
}
