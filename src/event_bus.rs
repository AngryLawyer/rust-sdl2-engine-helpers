pub struct EventBus<T> {
    items: Vec<T>
}

impl<T> EventBus<T> {
    pub fn new() -> EventBus<T> {
        EventBus {
            items: vec![]
        }
    }

    pub fn enqueue(&mut self, event: T) {
        self.items.push(event);
    }

    pub fn next(&mut self) -> Option<T> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }
}
