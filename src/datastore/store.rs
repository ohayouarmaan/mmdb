use std::{collections::HashMap, time::SystemTime};

#[derive(Debug)]
pub struct DataItem {
    pub data: String,
    pub expiry: Option<SystemTime>
}

pub struct DataStore {
    pub memory: HashMap<String, DataItem>
}

impl DataStore {
    pub fn new() -> Self {
        Self {
            memory: HashMap::new()
        }
    }

    pub fn set(&mut self, key: String, value: DataItem) {
        self.memory.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<&DataItem> {
        self.memory.get(&key)
    }

    pub fn remove(&mut self, key: String) -> Option<DataItem> {
        self.memory.remove(&key)
    }
}
