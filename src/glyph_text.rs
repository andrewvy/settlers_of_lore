use freetype;
use graphics::{Context, Graphics};
use opengl_graphics::{Texture, TextureSettings};

pub struct GlyphText<'a> {
    fontface: &'a freetype::Face,
    text: String,
    glyphs: Box<Vec<(Texture, [f64; 2])>>,
}

impl<'a> GlyphText<'a> {
    pub fn new(fontface: &'a freetype::Face, text: String) -> GlyphText<'a> {
        let mut text_struct = GlyphText {
            fontface: fontface,
            text: text,
            glyphs: Box::new(Vec::new()),
        };

        text_struct.generate_glyphs();
        text_struct
    }

    pub fn generate_glyphs(&mut self) {
        let mut x = 10;
        let mut y = 0;
        for ch in self.text.chars() {
            self.fontface
                .load_char(ch as usize, freetype::face::LoadFlag::RENDER)
                .unwrap();

            let g = self.fontface.glyph();

            let bitmap = g.bitmap();
            let texture = Texture::from_memory_alpha(
                bitmap.buffer(),
                bitmap.width() as u32,
                bitmap.rows() as u32,
                &TextureSettings::new(),
            ).unwrap();

            self.glyphs.push((
                texture,
                [(x + g.bitmap_left()) as f64, (y - g.bitmap_top()) as f64],
            ));

            x += (g.advance().x >> 6) as i32;
            y += (g.advance().y >> 6) as i32;
        }
    }

    pub fn render<G>(&self, c: &Context, gl: &mut G)
    where
        G: Graphics<Texture = Texture>,
    {
        for &(ref texture, [x, y]) in self.glyphs.iter() {
            use graphics::*;

            Image::new_color(color::BLACK).draw(
                texture,
                &c.draw_state,
                c.transform.trans(x, y),
                gl,
            );
        }
    }
}
