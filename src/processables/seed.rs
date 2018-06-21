use processables::Processable;

pub struct Seed {}

impl Seed {
    pub fn new() -> Processable<Seed> {
        let seed = Seed {};

        Processable::new(seed)
    }
}
