// HashMapImplementation.Rs

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const DEFAULT_MAX_SIZE: u64 = 256;

#[derive(Debug)]
pub struct HashMap<K, V> {
    curr_size: usize,
    arr: [Option<KeyValue<K, V>>; DEFAULT_MAX_SIZE as usize],
}

#[derive(Debug, Clone)]
pub struct KeyValue<K, V> {
    key: K,
    value: V,
    next: Option<Box<KeyValue<K, V>>>,
}

impl<K: Clone + Hash + PartialEq, V: Copy> HashMap<K, V> {
    pub fn new() -> HashMap<K, V> {
        HashMap {
            curr_size: 0,
            arr: [None; DEFAULT_MAX_SIZE as usize],
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let hash_val: u64 = Self::hash_key(key.clone());
        let position = hash_val % DEFAULT_MAX_SIZE;

        match &self.arr[position as usize] {
            Some(_) => self.update_or_link_new_val(key, value, position as usize),
            None => {
                self.insert_new_value(key, value, position as usize);
                None
            }
        }
    }

    pub fn get(&self, key: K) -> Option<V> {
        let hash_val: u64 = Self::hash_key(key.clone());
        let position = hash_val % DEFAULT_MAX_SIZE;

        match &self.arr[position as usize] {
            Some(_) => self.check_list_for_key(key, position as usize),
            None => None,
        }
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        let hash_val: u64 = Self::hash_key(key.clone());
        let position = hash_val % DEFAULT_MAX_SIZE;

        match &self.arr[position as usize] {
            Some(_) => self.check_item_in_list_and_remove(key, position as usize),
            None => None,
        }
    }

    fn hash_key<K: Hash>(key: K) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    // Implementation of helper methods...
}
