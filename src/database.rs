use crate::types::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Database {
    size: usize,
    shards: Vec<HashMap<String, Value>>,
}

impl Database {
    pub fn new(size: usize, reserve: usize) -> Self {
        let mut shards = Vec::new();

        for _ in 0..size {
            shards.push(HashMap::with_capacity(reserve));
        }

        Database { size, shards }
    }

    pub fn insert(&mut self, key: &str, value: Value) {
        let shard = &mut self.shards[key.len() % self.size];

        shard.insert(key.into(), value);
    }

    pub fn flush_all(&mut self) {
        for shard in &mut self.shards {
            shard.clear();
        }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        let shard = &self.shards[key.len() % self.size];

        return shard.get(key);
    }

    pub fn remove(&mut self, key: &str) {
        let shard = &mut self.shards[key.len() % self.size];

        shard.remove(key);
    }

    pub fn exists(&self, key: &str) -> bool {
        let shard = &self.shards[key.len() % self.size];

        shard.contains_key(key)
    }
}
