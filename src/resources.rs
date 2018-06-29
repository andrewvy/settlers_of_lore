use std::collections::HashMap;

use tilemap::Tile;

#[derive(Default)]
pub struct EntityMap {
    pub tiles: HashMap<(i32, i32), Tile>,
}

impl EntityMap {
    pub fn new() -> Self {
        EntityMap {
            tiles: HashMap::new(),
        }
    }
}

#[derive(Default)]
pub struct BackgroundMap {
    pub tiles: HashMap<(i32, i32), Tile>,
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
                };

                self.tiles.insert((x, y), tile);
            }
        }
    }
}
