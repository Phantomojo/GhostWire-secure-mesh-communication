use crate::core::message::Message;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub struct MessageCache {
    queue: Arc<Mutex<VecDeque<Message>>>,
}

impl MessageCache {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn add(&self, msg: Message) {
        let mut q = self.queue.lock().unwrap();
        q.push_back(msg);
    }

    pub fn pop(&self) -> Option<Message> {
        let mut q = self.queue.lock().unwrap();
        q.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        let q = self.queue.lock().unwrap();
        q.is_empty()
    }
} 