use specs::prelude::*;

use components::*;
use plantae;
use world::World;

pub fn create_flower(
    world: &mut World,
    x: i32,
    y: i32,
    instance: plantae::PlantaeInstance<plantae::flower::Flower>,
) -> Entity {
    world
        .specs_world
        .create_entity()
        .with(Position::new(x, y))
        .with(Plantae::new(instance))
        .build()
}
