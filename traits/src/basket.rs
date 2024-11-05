pub struct Basket<T> {
    items: Option<T>,
}

impl<T> Basket<T> {
    pub fn new(value: T) -> Self {
        Basket { items: Some(value) }
    }

    pub fn get(&mut self) -> Option<T> {
        self.items.take()
    }

    pub fn put(&mut self, value: T) {
        self.items = Some(value);
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_none()
    }
}
