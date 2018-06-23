use plantae::{Plantae, Quality};
use plantae::growth::GrowthModel;

pub struct Flower {}

impl Flower {
    pub fn new(id: u32, name: String) -> Plantae<Flower> {
        Plantae {
            id,
            name,
            ticks_per_growth: 120,
            max_growth_level: 10,
            growth_model: GrowthModel::new(60.0, 100.0, 500.0),
            inner: Flower {},
        }
    }
}
