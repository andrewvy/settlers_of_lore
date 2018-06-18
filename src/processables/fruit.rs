use processables::Processable;

pub struct Fruit {}

impl Fruit {
    pub fn new() -> Processable<Fruit> {
        let fruit = Fruit {}

        Processable::new(fruit)
    }
}
