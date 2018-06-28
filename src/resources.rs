use std::collections::HashMap;

use specs::Entity;

use tilemap::Tile;

pub struct EntityMap {
    pub tiles: HashMap<Tile, Entity>,
}

impl EntityMap {
    pub fn new() -> Self {
        EntityMap {
            tiles: HashMap::new(),
        }
    }
}

pub struct BackgroundMap {
    pub tiles: HashMap<Tile, Option<Entity>>,
}

impl BackgroundMap {
    pub fn new() -> Self {
        BackgroundMap {
            tiles: HashMap::new(),
        }
    }

    pub fn generate(&mut self) {
        for x in 0..128 {
            for y in 0..128 {
                let tile = Tile {
                    sprite_layer: 0,
                    sprite_id: 77,
                    x,
                    y,
                };

                self.tiles.insert(tile, None);
            }
        }
    }
}
