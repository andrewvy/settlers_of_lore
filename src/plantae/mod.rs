use std::rc::Rc;
use std::collections::HashMap;

pub mod growth;
pub mod tree;
pub mod flower;

pub enum Quality {
    Perfect,
    Ideal,
    Uncommon,
    Common,
}

pub struct Plantae<T> {
    pub id: u32,
    pub name: String,
    pub max_growth_level: u32,
    pub ticks_per_growth: u32,
    pub growth_model: growth::GrowthModel,
    pub inner: T,
}

pub struct PlantaeInstance<T> {
    instance_type: Rc<Plantae<T>>,
    pub ticks: u32,
    pub quality: Quality,
}

impl<T> PlantaeInstance<T> {
    pub fn new(instance_type: Rc<Plantae<T>>) -> PlantaeInstance<T> {
        PlantaeInstance {
            instance_type,
            ticks: 0,
            quality: Quality::Common,
        }
    }

    pub fn tick(&mut self) {
        self.ticks += 1;
    }
}

pub struct PlantaeDictionary {
    pub trees: HashMap<u32, Rc<Plantae<tree::Tree>>>,
    pub flowers: HashMap<u32, Rc<Plantae<flower::Flower>>>,
}

impl PlantaeDictionary {
    pub fn new() -> PlantaeDictionary {
        let mut trees = HashMap::new();
        let mut flowers = HashMap::new();
        let tree = tree::Tree::new(1, "Acacia hybryda".to_owned());
        let flower = flower::Flower::new(1, "Cosmos bipinnatus".to_owned());

        let tree_max_rate = tree.growth_model.max_growth_rate;
        let ticks_when_growing = tree.growth_model.tick_mid / 3.0;
        let ticks_when_harvestable = tree.growth_model.tick_mid + (tree.growth_model.tick_end - tree.growth_model.tick_mid) / 2.0;
        let ticks_when_decay = tree.growth_model.tick_end + (tree.growth_model.tick_end - tree.growth_model.tick_mid) / 2.0;

        let mut i = 0;

        while i <= 120 {
            let tick = i as f64;
            let tree_weight = tree.growth_model.weight(tick);

            let mut state = "growing";

            if (tick < ticks_when_growing) {
                state = "sprouting";
            } else if (tick >= ticks_when_growing) && (tick < ticks_when_harvestable) {
                state = "growing"
            } else if (tick >= ticks_when_harvestable) && (tick < ticks_when_decay) {
                state = "harvestable";
            } else if (tick >= ticks_when_decay) {
                state = "decaying";
            }

            println!("state: {}, tick: {}, plant_weight: {:2.2}g", state, tick, tree_weight);

            i += 2;
        }

        trees.insert(1, Rc::new(tree));
        flowers.insert(1, Rc::new(flower));

        PlantaeDictionary {
            trees,
            flowers,
        }
    }
}
