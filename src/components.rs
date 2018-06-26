use specs::prelude::*;

#[derive(Debug, Default)]
pub struct Player {}

impl Component for Player {
    type Storage = NullStorage<Self>;
}

#[derive(Debug, Default)]
pub struct Position {
    x: i32,
    y: i32
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position {
            x,
            y,
        }
    }
}

#[derive(Debug, Default)]
pub struct Sprite {
    atlas_id: i32,
    sprite_id: i32,
}

impl Component for Sprite {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct Solid;

impl Component for Solid {
    type Storage = NullStorage<Self>;
}
