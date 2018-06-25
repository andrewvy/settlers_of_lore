use specs::prelude::*;

#[derive(Debug, Default)]
pub struct Player {}

impl Component for Player {
    type Storage = NullStorage<Self>;
}

#[derive(Debug, Default)]
pub struct Position {
    x: f32,
    y: f32
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}
