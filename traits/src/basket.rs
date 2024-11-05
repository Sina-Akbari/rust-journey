use super::container::Container;

pub struct Basket<T> {
    items: Option<T>,
}

impl<T> Basket<T> {
    pub fn new(value: T) -> Self {
        Basket { items: Some(value) }
    }
}

impl<T> Container<T> for Basket<T> {
    fn get(&mut self) -> Option<T> {
        self.items.take()
    }

    fn put(&mut self, value: T) {
        self.items = Some(value);
    }

    fn is_empty(&self) -> bool {
        self.items.is_none()
    }
}
