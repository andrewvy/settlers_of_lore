pub mod fruit;
pub mod oil;

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
