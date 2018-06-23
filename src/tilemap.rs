use rand::{distributions, thread_rng, Rng};
use warmy;
use warmy::load::Store;

use ggez::graphics::spritebatch::{SpriteBatch, SpriteIdx};
use ggez::graphics::{Color, DrawParam, Point2, Rect};
use ggez::Context;

use screen::Screen;

use assets::Image;

const TILE_MAP_WIDTH: usize = 65;
const TILE_MAP_HEIGHT: usize = 45;
const SPRITE_DIMENSIONS: u32 = 8;

pub struct Tile(SpriteIdx);

pub struct TileMap {
    sprite_dimensions: u32,
    tile_width: u32,
    tile_height: u32,
    num_tiles_x: u32,
    num_tiles_y: u32,
    tiles: Vec<Tile>,
    scale: Point2,
    screen: Screen,
    pub batch: SpriteBatch,
}

impl TileMap {
    pub fn new(
        path: &str,
        screen: Screen,
        asset_store: &mut Store<Context>,
        ctx: &mut Context,
    ) -> TileMap {
        let image = asset_store
            .get::<_, Image>(&warmy::FSKey::new(path), ctx)
            .unwrap();
        let inner = &image.borrow().0;

        let num_tiles_x: u32 = (screen.screen_w / SPRITE_DIMENSIONS) + 1;
        let num_tiles_y: u32 = (screen.screen_h / SPRITE_DIMENSIONS) + 1;

        TileMap {
            sprite_dimensions: SPRITE_DIMENSIONS,
            tile_width: inner.width() / SPRITE_DIMENSIONS,
            tile_height: inner.height() / SPRITE_DIMENSIONS,
            num_tiles_x,
            num_tiles_y,
            tiles: Vec::with_capacity((num_tiles_x * num_tiles_y) as usize),
            batch: SpriteBatch::new(inner.clone()),
            scale: Point2::new(screen.scale_w, screen.scale_h),
            screen,
        }
    }

    pub fn generate(&mut self) {
        for i in 0..self.tiles.capacity() {
            let mut rng = thread_rng();

            let x: usize = (i % self.num_tiles_x as usize) * SPRITE_DIMENSIONS as usize;
            let y: usize = (i / self.num_tiles_x as usize) * SPRITE_DIMENSIONS as usize;

            let tile_range = distributions::Uniform::new_inclusive(1, 100);

            let mut sprite_y = 3;
            let mut sprite_x = 2;

            match rng.sample(&tile_range) {
                98 | 99 => {
                    sprite_x = 3;
                }
                100 => {
                    sprite_x = 2;
                    sprite_y = 2;
                }
                _ => {}
            }

            let draw_param = DrawParam {
                src: Rect::new(
                    (1.0 / self.tile_width as f32) * sprite_x as f32,
                    (1.0 / self.tile_height as f32) * sprite_y as f32,
                    1.0 / self.tile_width as f32,
                    1.0 / self.tile_height as f32,
                ),
                dest: self.screen
                    .to_screen_coordinates(Point2::new(x as f32, y as f32)),
                scale: self.scale,
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
