use warmy;
use warmy::load::Store;

use ggez::{Context, GameResult};
use ggez::graphics::spritebatch::{SpriteBatch};
use ggez::graphics::{self, DrawParam, Rect, Point2, Color};
use assets::Image;

const SPRITE_DIMENSIONS: u32 = 8;

pub struct SpriteSheet {
    sprite_dimensions: u32,
    tiles_width: u32,
    tiles_height: u32,
    pub batch: SpriteBatch,
}

impl SpriteSheet {
    pub fn new(path: &str, asset_store: &mut Store<Context>, ctx: &mut Context) -> SpriteSheet {
        let image = asset_store.get::<_, Image>(&warmy::FSKey::new(path), ctx).unwrap();
        let inner = &image.borrow().0;

        SpriteSheet {
            sprite_dimensions: SPRITE_DIMENSIONS,
            tiles_width: inner.width() / SPRITE_DIMENSIONS,
            tiles_height: inner.height() / SPRITE_DIMENSIONS,
            batch: SpriteBatch::new(inner.clone()),
        }
    }

    pub fn enqueue(&mut self, sprite_x: u32, sprite_y: u32, destination: Point2) {
        let draw_param = DrawParam {
            src: Rect::new(
                (1.0 / self.tiles_width as f32) * sprite_x as f32,
                (1.0 / self.tiles_height as f32) * sprite_y as f32,
                (1.0 / self.tiles_width as f32),
                (1.0 / self.tiles_height as f32),
            ),
            dest: destination,
            scale: Point2::new(3.0, 3.0),
            color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
            ..Default::default()
        };

        self.batch.add(draw_param);
    }

    pub fn clear(&mut self) {
        self.batch.clear();
    }
}
