extern crate find_folder;
extern crate freetype;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;

mod glyph_text;

use glyph_text::GlyphText;

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

    let freetype = freetype::Library::init().unwrap();
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
