use plantae::{Plantae, Quality};

pub struct Flower {}

impl Flower {
    pub fn new(id: u32, name: String) -> Plantae<Flower> {
        Plantae {
            id,
            name,
            ticks_per_growth: 120,
            max_growth_level: 10,
            inner: Flower {},
        }
    }
}
