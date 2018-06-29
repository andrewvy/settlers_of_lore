use ggez::graphics::spritebatch::{SpriteBatch, SpriteIdx};
use ggez::graphics::{Color, DrawParam, Point2, Rect};
use ggez::Context;
use warmy;
use warmy::load::Store;

use assets::Image;
use screen::Screen;

#[derive(Hash, Eq, PartialEq)]
pub struct Tile {
    pub sprite_layer: i32,
    pub sprite_id: i32,
}

#[derive(Clone)]
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
    tilemap: TileMap,
    pub batch: SpriteBatch,
}

impl SpriteLayer {
    pub fn new(tilemap: TileMap) -> Self {
        let image = tilemap.image.borrow().0.clone();

        SpriteLayer {
            tilemap,
            batch: SpriteBatch::new(image),
        }
    }

    pub fn add(&mut self, tile: &Tile, x: i32, y: i32) -> SpriteIdx {
        let x: usize = x as usize * self.tilemap.sprite_dimensions as usize;
        let y: usize = y as usize * self.tilemap.sprite_dimensions as usize;

        let sprite_x = tile.sprite_id as usize % self.tilemap.tile_width as usize;
        let sprite_y = tile.sprite_id as usize / self.tilemap.tile_width as usize;

        let draw_param = DrawParam {
            src: Rect::new(
                (1.0 / self.tilemap.tile_width as f32) * sprite_x as f32,
                (1.0 / self.tilemap.tile_height as f32) * sprite_y as f32,
                1.0 / self.tilemap.tile_width as f32,
                1.0 / self.tilemap.tile_height as f32,
            ),
            dest: self.tilemap
                .screen
                .to_screen_coordinates(Point2::new(x as f32, y as f32)),
            scale: self.tilemap.scale,
            color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
            ..Default::default()
        };

        self.batch.add(draw_param)
    }

    pub fn clear(&mut self) {
        self.batch.clear();
    }
}
