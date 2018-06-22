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
    pub growth_parameters: growth::GrowthParameters,
    pub inner: T,
}

pub struct PlantaeInstance<T> {
    instance_type: Rc<Plantae<T>>,
    pub ticks: u32,
    pub growth_level: u32,
    pub quality: Quality,
}

impl<T> PlantaeInstance<T> {
    pub fn new(instance_type: Rc<Plantae<T>>) -> PlantaeInstance<T> {
        PlantaeInstance {
            instance_type,
            ticks: 0,
            growth_level: 0,
            quality: Quality::Common,
        }
    }

    pub fn able_to_be_harvested(&self) -> bool {
        self.growth_level >= self.instance_type.max_growth_level
    }

    pub fn tick(&mut self) {
        if self.growth_level < self.instance_type.max_growth_level {
            self.ticks += 1;
        }

        if self.growth_level >= self.instance_type.max_growth_level {
            self.ticks = 0;
        } else if (self.ticks >= self.instance_type.ticks_per_growth) {
            self.ticks = 0;
            self.growth_level += 1;
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
        let tree = tree::Tree::new(1, "Acacia hybryda".to_owned());
        let flower = flower::Flower::new(1, "Cosmos bipinnatus".to_owned());

        let tree_weight = tree.growth_parameters.weight(25.0);
        let tree_max_rate = tree.growth_parameters.max_growth_rate();

        println!("tree_weight {}, tree_max_rate {}", tree_weight, tree_max_rate);

        trees.insert(1, Rc::new(tree));
        flowers.insert(1, Rc::new(flower));

        PlantaeDictionary {
            trees,
            flowers,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_tick_and_grow() {
        let mut acacia = tree::Tree::new(1, "Acacia hybryda".to_owned());
        acacia.ticks_per_growth = 1;

        let acacia_type = Rc::new(acacia);
        let mut acacia_instance = PlantaeInstance::new(acacia_type.clone());

        assert_eq!(acacia_instance.instance_type.ticks_per_growth, 1);
        assert_eq!(acacia_instance.growth_level, 0);

        acacia_instance.tick();

        assert_eq!(acacia_instance.growth_level, 1);

        for _ in 0..100 {
            acacia_instance.tick();
        }

        assert_eq!(acacia_instance.growth_level, acacia_instance.instance_type.max_growth_level);
    }
}
