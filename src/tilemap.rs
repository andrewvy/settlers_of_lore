use rand::{thread_rng, Rng};
use warmy;
use warmy::load::Store;

use ggez::graphics::spritebatch::{SpriteBatch, SpriteIdx};
use ggez::graphics::{self, Color, DrawParam, Point2, Rect};
use ggez::Context;

use assets::Image;

const TILE_MAP_WIDTH: usize = 80;
const TILE_MAP_HEIGHT: usize = 80;
const SPRITE_DIMENSIONS: u32 = 8;

pub struct Tile(SpriteIdx);

pub struct TileMap {
    sprite_dimensions: u32,
    tiles_width: u32,
    tiles_height: u32,
    tiles: Vec<Tile>,
    pub batch: SpriteBatch,
}

impl TileMap {
    pub fn new(path: &str, asset_store: &mut Store<Context>, ctx: &mut Context) -> TileMap {
        let image = asset_store
            .get::<_, Image>(&warmy::FSKey::new(path), ctx)
            .unwrap();
        let inner = &image.borrow().0;

        TileMap {
            sprite_dimensions: SPRITE_DIMENSIONS,
            tiles_width: inner.width() / SPRITE_DIMENSIONS,
            tiles_height: inner.height() / SPRITE_DIMENSIONS,
            tiles: Vec::with_capacity(TILE_MAP_HEIGHT * TILE_MAP_WIDTH),
            batch: SpriteBatch::new(inner.clone()),
        }
    }

    pub fn generate(&mut self) {
        for i in 0..self.tiles.capacity() {
            let mut rng = thread_rng();

            let x: usize = (i % TILE_MAP_WIDTH) * SPRITE_DIMENSIONS as usize;
            let y: usize = (i / TILE_MAP_WIDTH) * SPRITE_DIMENSIONS as usize;

            let sprite_x: u32 = rng.gen_range(3, 5);
            let sprite_y: u32 = rng.gen_range(3, 5);

            let draw_param = DrawParam {
                src: Rect::new(
                    (1.0 / self.tiles_width as f32) * sprite_x as f32,
                    (1.0 / self.tiles_height as f32) * sprite_y as f32,
                    1.0 / self.tiles_width as f32,
                    1.0 / self.tiles_height as f32,
                ),
                dest: Point2::new(x as f32, y as f32),
                scale: Point2::new(1.0, 1.0),
                color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
                ..Default::default()
            };

            let sprite_idx = self.batch.add(draw_param);

            self.tiles.push(Tile(sprite_idx));
        }
    }

    pub fn clear(&mut self) {
        self.batch.clear();
    }
}
