use specs::Entity;

pub struct Map {
    pub tiles: HashMap<(i32, i32), Entity>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            tiles: HashMap::new(),
        }
    }
}
