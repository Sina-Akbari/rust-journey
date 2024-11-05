pub struct Basket<T> {
    item: Option<T>,
}

impl<T> Basket<T> {
    pub fn new(value: T) -> Self {
        Basket { item: Some(value) }
    }

    pub fn get(&mut self) -> Option<T> {
        self.item.take()
    }

    pub fn put(&mut self, value: T) {
        self.item = Some(value);
    }

    pub fn is_empty(&self) -> bool {
        self.item.is_none()
    }
}
