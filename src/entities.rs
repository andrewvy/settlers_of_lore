use specs::Entity;
use world::World;
use components::*;

pub fn create_grass(world: &mut World, x: i32, y: i32) -> Entity {
    world.specs_world
        .create_entity()
        .with(Position::new(x, y))
        .build()
}
