use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::Point2;
use ggez::Context;
use warmy;
use warmy::load::Store;

use assets::Image;
use screen::Screen;

pub struct TileMap {
    sprite_dimensions: u32,
    tile_width: u32,
    tile_height: u32,
    num_tiles_x: u32,
    num_tiles_y: u32,
    scale: Point2,
    screen: Screen,
    image: warmy::Res<Image>,
}

impl TileMap {
    pub fn new(
        path: &str,
        screen: Screen,
        asset_store: &mut Store<Context>,
        ctx: &mut Context,
        sprite_dimensions: u32,
    ) -> Self {
        let image = asset_store
            .get::<_, Image>(&warmy::FSKey::new(path), ctx)
            .unwrap();
        let inner = image.borrow().0.clone();

        let num_tiles_x: u32 = (screen.screen_w / sprite_dimensions) + 1;
        let num_tiles_y: u32 = (screen.screen_h / sprite_dimensions) + 1;

        TileMap {
            sprite_dimensions,
            tile_width: inner.width() / sprite_dimensions,
            tile_height: inner.height() / sprite_dimensions,
            num_tiles_x,
            num_tiles_y,
            scale: Point2::new(screen.scale_w, screen.scale_h),
            screen,
            image,
        }
    }
}

pub struct SpriteLayer {
    pub batch: SpriteBatch,
}

impl SpriteLayer {
    pub fn new(tilemap: &TileMap) -> Self {
        let image = tilemap.image.borrow().0.clone();

        SpriteLayer {
            batch: SpriteBatch::new(image),
        }
    }
}
