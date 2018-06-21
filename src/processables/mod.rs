pub mod bark;
pub mod fruit;
pub mod leaf;
pub mod oil;
pub mod seed;

pub struct Processable<T> {
    inner: T,
}

impl<T> Processable<T> {
    pub fn new(inner: T) -> Processable<T> {
        Processable {
            inner,
        }
    }
}
