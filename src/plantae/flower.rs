use plantae::growth::GrowthModel;
use plantae::Plantae;

#[derive(Debug)]
pub struct Flower {}

impl Flower {
    pub fn new(id: u32, name: String) -> Plantae<Flower> {
        Plantae {
            id,
            name,
            growth_model: GrowthModel::new(60.0, 100.0, 500.0),
            inner: Flower {},
        }
    }
}
