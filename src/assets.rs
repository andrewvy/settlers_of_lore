use ggez::graphics::{Font, Scale};
use ggez::{Context, GameResult};

use screen::Screen;

pub struct Assets {
    pub font: Font,
    pub default_scale: Scale,
}

impl Assets {
    pub fn new(ctx: &mut Context, screen: &Screen) -> GameResult<Assets> {
        let font = Font::new_glyph_font(ctx, "/m5x7.ttf")?;

        Ok(Assets {
            font: font,
            default_scale: Assets::display_independent_scale(screen.scale_w, screen.scale_h, 24.0),
        })
    }

    pub fn display_independent_scale(scale_w: f32, scale_h: f32, pixels: f32) -> Scale {
        Scale {
            x: pixels * scale_h,
            y: pixels * scale_w,
        }
    }
}
