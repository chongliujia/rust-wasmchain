use std::collections::HashMap;

pub trait State {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;
    fn set(&mut self, key: Vec<u8>, value: Vec<u8>);
}

pub struct InMemoryState {
    inner: HashMap<Vec<u8>, Vec<u8>>,
}

impl InMemoryState {
    pub fn new() -> Self {
        Self { inner: HashMap::new() }
    }
}

impl State for InMemoryState {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.inner.get(key).cloned()
    }

    fn set(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.inner.insert(key, value);
    }
}
