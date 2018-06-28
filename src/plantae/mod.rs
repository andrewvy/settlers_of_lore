use std::collections::HashMap;
use std::rc::Rc;

pub mod flower;
pub mod growth;
pub mod tree;

pub enum Quality {
    Perfect,
    Ideal,
    Uncommon,
    Common,
}

#[derive(Copy, Clone, Debug)]
pub enum GrowthState {
    Sprouting,
    Growing,
    Harvestable,
    Decaying,
}

// This struct represents the genomic factors for growth, turned
// into a PlantaeInstance once planted.
//
// Examples: flower seed, tree nuts, fruits, etc.
pub struct PlantaeEmbryo {
    instance_type_id: u32,
    growth_model: growth::GrowthModel,
}

pub struct Plantae<T> {
    pub id: u32,
    pub name: String,
    pub growth_model: growth::GrowthModel,
    pub inner: T,
}

pub struct PlantaeInstance<T> {
    instance_type: Rc<Plantae<T>>,
    ticks_until_growing: f64,
    ticks_until_harvestable: f64,
    ticks_until_decay: f64,
    pub weight: f64,
    pub ticks: f64,
    pub quality: Quality,
    pub state: GrowthState,
}

impl<T> PlantaeInstance<T> {
    pub fn new(instance_type: Rc<Plantae<T>>) -> PlantaeInstance<T> {
        let ticks_until_growing = instance_type.growth_model.tick_mid / 3.0;
        let ticks_until_harvestable = instance_type.growth_model.tick_mid
            + (instance_type.growth_model.tick_end - instance_type.growth_model.tick_mid) / 2.0;
        let ticks_until_decay = instance_type.growth_model.tick_end
            + (instance_type.growth_model.tick_end - instance_type.growth_model.tick_mid) / 2.0;

        PlantaeInstance {
            instance_type,
            quality: Quality::Common,
            state: GrowthState::Sprouting,
            ticks: 0.0,
            weight: 0.0,
            ticks_until_decay,
            ticks_until_growing,
            ticks_until_harvestable,
        }
    }

    pub fn tick(&mut self) {
        self.ticks += 1.0;
        self.weight = self.instance_type.growth_model.weight(self.ticks);

        if self.ticks < self.ticks_until_growing {
            self.state = GrowthState::Sprouting;
        } else if (self.ticks >= self.ticks_until_growing)
            && (self.ticks < self.ticks_until_harvestable)
        {
            self.state = GrowthState::Growing;
        } else if (self.ticks >= self.ticks_until_harvestable)
            && (self.ticks < self.ticks_until_decay)
        {
            self.state = GrowthState::Harvestable;
        } else if self.ticks >= self.ticks_until_decay {
            self.state = GrowthState::Decaying;
        }
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
        let tree = Rc::new(tree::Tree::new(1, "Acacia hybryda".to_owned()));
        let flower = Rc::new(flower::Flower::new(1, "Cosmos bipinnatus".to_owned()));

        trees.insert(1, tree);
        flowers.insert(1, flower);

        PlantaeDictionary { trees, flowers }
    }
}
