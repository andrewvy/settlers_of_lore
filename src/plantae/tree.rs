use std::str;

use plantae::{Plantae, Quality};
use plantae::growth::GrowthParameters;

pub struct Tree {
    pub has_bark: bool,
}

impl Tree {
    pub fn new(id: u32, _name: String) -> Plantae<Tree> {
        let generated_name = Tree::generate_name();

        Plantae {
            id,
            name: generated_name,
            ticks_per_growth: 120,
            max_growth_level: 50,
            growth_parameters: GrowthParameters {
                tick_end: 100.,
                tick_mid: 60.,
                max_weight: 500.,
            },
            inner: Tree {
                has_bark: true,
            },
        }
    }

    pub fn generate_name() -> String {
        let buffer = include_bytes!("../../resources/data/trees.txt");
        let tree_names = match str::from_utf8(buffer) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        let mut iter = tree_names.split('\n');

        iter.nth(0).unwrap().to_owned()
    }
}
