use processables::Processable;

pub struct Leaf {}

impl Leaf {
    pub fn new() -> Processable<Leaf> {
        let leaf = Leaf {};

        Processable::new(leaf)
    }
}
