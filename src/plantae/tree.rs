use plantae::{Plantae, Quality};

pub struct Tree {
    pub has_bark: bool,
}

impl Tree {
    pub fn new(id: u32, name: String) -> Plantae<Tree> {
        Plantae {
            id,
            name,
            ticks_per_growth: 120,
            quality: Quality::Common,
            inner: Tree {
                has_bark: true,
            },
        }
    }
}
