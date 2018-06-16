use std::path;

use ggez::graphics;
use ggez::graphics::{Font, Scale};
use ggez::{Context, GameError, GameResult};

use warmy;

use screen::Screen;

pub struct Assets {
    pub font: Font,
    pub default_scale: Scale,
    pub asset_store: warmy::Store<Context>,
}

impl Assets {
    pub fn new(
        resource_dir: Option<path::PathBuf>,
        ctx: &mut Context,
        screen: &Screen,
    ) -> GameResult<Assets> {
        let resource_pathbuf: path::PathBuf = match resource_dir {
            Some(s) => s,
            None => ctx.filesystem.get_resources_dir().to_owned(),
        };

        let font = Font::new_glyph_font(ctx, "/fonts/m5x7.ttf")?;
        let opt = warmy::StoreOpt::default().set_root(resource_pathbuf);
        let asset_store = warmy::Store::new(opt).expect("No asset store?");

        Ok(Assets {
            font: font,
            default_scale: Assets::display_independent_scale(screen.scale_w, screen.scale_h, 24.0),
            asset_store,
        })
    }

    pub fn display_independent_scale(scale_w: f32, scale_h: f32, pixels: f32) -> Scale {
        Scale {
            x: pixels * scale_h,
            y: pixels * scale_w,
        }
    }
}

fn warmy_to_ggez_path(path: &path::Path, root: &path::Path) -> path::PathBuf {
    let stripped_path = path.strip_prefix(root)
        .expect("warmy path is outside of the warmy store somehow");

    path::Path::new("/").join(stripped_path)
}

#[derive(Debug, Clone)]
pub struct Image(pub graphics::Image);
impl warmy::Load<Context> for Image {
    type Key = warmy::FSKey;
    type Error = GameError;

    fn load(
        key: Self::Key,
        store: &mut warmy::Storage<Context>,
        ctx: &mut Context,
    ) -> Result<warmy::Loaded<Self>, Self::Error> {
        let path = warmy_to_ggez_path(key.as_path(), store.root());

        graphics::Image::new(ctx, path)
            .map(|x| warmy::Loaded::from(Image(x)))
            .map_err(|_| {
                GameError::ResourceLoadError(
                    format!("Could not load {:?}", key.as_path()).to_string(),
                )
            })
    }
}
