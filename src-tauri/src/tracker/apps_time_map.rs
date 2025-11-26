use std::cmp::Reverse;
use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppsTimeMap {
    #[serde(flatten)]
    inner: HashMap<String, u32>,
}

impl fmt::Debug for AppsTimeMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut vec: Vec<_> = self.inner.iter().collect();
        vec.sort_by_key(|(_, v)| std::cmp::Reverse(*v));
        f.debug_map().entries(vec.into_iter()).finish()
    }
}

impl Clone for AppsTimeMap {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl AppsTimeMap {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, val: u32) {
        self.inner.insert(key, val);
    }

    pub fn add(&mut self, key: String, val: u32) {
        if self.inner.contains_key(&key) {
            let current_val = self.inner.get(&key).unwrap();
            self.inner.insert(key, current_val + val);
        } else {
            self.inner.insert(key, val);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &u32)> {
        self.inner.iter()
    }

    pub fn get(&self, key: &str) -> Option<&u32> {
        self.inner.get(key)
    }

    pub fn sorted(&self) -> Vec<(&str, u32)> {
        let mut vec: Vec<_> = self.inner.iter().map(|(k, v)| (k.as_str(), *v)).collect();
        vec.sort_by_key(|(_, v)| Reverse(*v));
        vec
    }
}
