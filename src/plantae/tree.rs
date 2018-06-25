use std::str;

use rand::{thread_rng, Rng};

use plantae::{Plantae};
use plantae::growth::GrowthModel;

pub struct Tree {
    pub has_bark: bool,
}

impl Tree {
    pub fn new(id: u32, _name: String) -> Plantae<Tree> {
        let generated_name = Tree::generate_name();

        Plantae {
            id,
            name: generated_name,
            growth_model: GrowthModel::new(40.0, 80.0, 500.0),
            inner: Tree {
                has_bark: true,
            },
        }
    }

    pub fn generate_name() -> String {
        let buffer = include_bytes!("../../resources/data/trees.txt");
        let tree_name_buffer = match str::from_utf8(buffer) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        let tree_names: Vec<&str> = tree_name_buffer.split('\n').collect();
        let tree_name = thread_rng().choose(&tree_names).expect("Could not generate random name");

        (*tree_name).to_owned()
    }
}
