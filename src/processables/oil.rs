use processables::Processable;

pub struct Oil {}

impl Oil {
    pub fn new() -> Processable<Oil> {
        let oil = Oil {};

        Processable::new(oil)
    }
}
