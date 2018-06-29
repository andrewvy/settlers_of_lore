use specs::prelude::*;

use components;
use plantae::GrowthState;
use resources;
use tilemap::Tile;

const GAME_TICKS_PER_GROWTH_TICK: u32 = 6;

#[derive(Default)]
pub struct Plantae {
    pub ticks: u32,
}

impl<'a> System<'a> for Plantae {
    type SystemData = (
        Write<'a, resources::EntityMap>,
        WriteStorage<'a, components::Plantae>,
        ReadStorage<'a, components::Position>,
    );

    fn run(&mut self, (mut entity_map, mut plantae, position): Self::SystemData) {
        self.ticks += 1;

        if self.ticks >= GAME_TICKS_PER_GROWTH_TICK {
            for (mut plantae, position) in (&mut plantae, &position).join() {
                plantae.instance.tick();

                let sprite_id = match plantae.instance.state {
                    GrowthState::Sprouting => 0,
                    GrowthState::Growing => 1,
                    GrowthState::Harvestable => 3,
                    GrowthState::Decaying => 4,
                };

                let tile = Tile {
                    sprite_layer: 1,
                    sprite_id,
                };

                entity_map.tiles.insert((position.x, position.y), tile);
            }

            self.ticks = 0;
        }
    }
}
