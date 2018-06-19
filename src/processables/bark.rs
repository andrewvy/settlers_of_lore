use processables::Processable;

pub struct Bark {}

impl Bark {
    pub fn new() -> Processable<Bark> {
        let bark = Bark {};

        Processable::new(bark)
    }
}
