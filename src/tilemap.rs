use warmy;
use warmy::load::Store;

use ggez::graphics::spritebatch::{SpriteBatch, SpriteIdx};
use ggez::graphics::{Point2};
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
            batch: SpriteBatch::new(inner.clone()),
            scale: Point2::new(screen.scale_w, screen.scale_h),
            screen,
        }
    }

    pub fn clear(&mut self) {
        self.batch.clear();
    }
}
