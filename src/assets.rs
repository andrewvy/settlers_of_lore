use ggez::graphics::{self, Font, FontId, Scale};
use ggez::{Context, GameResult};

pub struct Assets {
    pub font: Font,
    pub default_scale: Scale,
    display_scale_w: f32,
    display_scale_h: f32,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let font = Font::new_glyph_font(ctx, "/m5x7.ttf")?;

        let (window_w, window_h) = graphics::get_size(ctx);
        let (displayable_w, displayable_h) = graphics::get_drawable_size(ctx);
        let (display_scale_w, display_scale_h) = (
            (displayable_w as f32 / window_w as f32),
            (displayable_h as f32 / window_h as f32),
        );

        println!("scale_w {} scale_h {}", display_scale_w, display_scale_h);

        Ok(Assets {
            font: font,
            default_scale: Assets::display_independent_scale(
                display_scale_w,
                display_scale_h,
                24.0,
            ),
            display_scale_w,
            display_scale_h,
        })
    }

    pub fn display_independent_scale(scale_w: f32, scale_h: f32, pixels: f32) -> Scale {
        Scale {
            x: pixels * scale_h,
            y: pixels * scale_w,
        }
    }
}
