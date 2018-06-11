extern crate find_folder;
extern crate freetype as ft;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use graphics::{Context, Graphics};
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;

pub struct GlyphText<'a> {
    fontface: &'a ft::Face,
    text: String,
    glyphs: Box<Vec<(Texture, [f64; 2])>>,
}

impl<'a> GlyphText<'a> {
    pub fn new(fontface: &'a ft::Face, text: String) -> GlyphText<'a> {
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
                .load_char(ch as usize, ft::face::LoadFlag::RENDER)
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

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("piston-example-freetype", [1024, 720])
        .resizable(false)
        .opengl(opengl)
        .samples(2)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let freetype = ft::Library::init().unwrap();
    let font = assets.join("m5x7.ttf");
    let fontface = freetype.new_face(&font, 0).unwrap();
    fontface.set_pixel_sizes(0, 48).unwrap();

    let ref mut gl = GlGraphics::new(opengl);
    let text = GlyphText::new(&fontface, "Hello Piston!".to_owned());

    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            use graphics::{clear, color, Transformed};

            gl.draw(args.viewport(), |c, gl| {
                clear(color::WHITE, gl);
                text.render(&c.trans(0.0, 27.0), gl);
            });
        }
    }
}
