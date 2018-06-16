use ggez::graphics;
use ggez::{Context, GameResult};

#[derive(Debug, Copy, Clone)]
pub struct Screen {
    pub logical_w: u32,
    pub logical_h: u32,
    pub screen_w: u32,
    pub screen_h: u32,
    pub scale_w: f32,
    pub scale_h: f32,
}

impl Screen {
    pub fn new(ctx: &mut Context) -> GameResult<Screen> {
        let (logical_w, logical_h) = graphics::get_size(ctx);
        let (screen_w, screen_h) = graphics::get_drawable_size(ctx);
        let (scale_w, scale_h) = (
            (screen_w as f32 / logical_w as f32),
            (screen_h as f32 / logical_h as f32),
        );

        Ok(Screen {
            logical_w,
            logical_h,
            screen_w,
            screen_h,
            scale_w,
            scale_h,
        })
    }

    pub fn to_screen_coordinates(&self, coords: graphics::Point2) -> graphics::Point2 {
        graphics::Point2::new(coords.x * self.scale_w, coords.y * self.scale_h)
    }
}
