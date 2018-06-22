use plantae::{Plantae, Quality};
use plantae::growth::GrowthParameters;

pub struct Flower {}

impl Flower {
    pub fn new(id: u32, name: String) -> Plantae<Flower> {
        Plantae {
            id,
            name,
            ticks_per_growth: 120,
            max_growth_level: 10,
            growth_parameters: GrowthParameters {
                tick_end: 100.,
                tick_mid: 60.,
                max_weight: 500.,
            },
            inner: Flower {},
        }
    }
}
