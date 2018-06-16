use std::rc::Rc;
use std::collections::HashMap;

pub mod tree;
pub mod flower;

enum Quality {
    Perfect,
    Ideal,
    Uncommon,
    Common,
}

pub struct Plantae<T> {
    id: u32,
    name: String,
    ticks_per_growth: u32,
    quality: Quality,
    inner: T,
}

pub struct PlantaeInstance<T> {
    instance: Rc<Plantae<T>>,
    growth_level: u32,
}

pub struct PlantaeDictionary {
    trees: HashMap<u32, Rc<Plantae<tree::Tree>>>,
    flowers: HashMap<u32, Rc<Plantae<flower::Flower>>>,
}

impl PlantaeDictionary {
    pub fn new() -> PlantaeDictionary {
        let mut trees = HashMap::new();
        let mut flowers = HashMap::new();

        trees.insert(1, Rc::new(tree::Tree::new(1, "Acacia hybryda".to_owned())));
        flowers.insert(1, Rc::new(flower::Flower::new(1, "Cosmos bipinnatus".to_owned())));

        PlantaeDictionary {
            trees,
            flowers,
        }
    }
}
