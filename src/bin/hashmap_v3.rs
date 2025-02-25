use sha2::{Sha256, Digest};
use std::convert::TryInto;

/// Hashes a key and maps it to an index within the array size.
/// We require K to implement `ToString` so we can convert it to bytes.
fn hash_to_index<K: ToString>(key: &K, arr_size: usize) -> usize {
    let mut hasher = Sha256::new();
    hasher.update(key.to_string().as_bytes());
    let hash_result = hasher.finalize();
    let int_value = u64::from_be_bytes(hash_result[..8].try_into().unwrap());
    (int_value % arr_size as u64) as usize
}

#[derive(Debug)]
pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    size: usize,
}

impl<K: ToString + Clone + PartialEq, V: Clone> HashMap<K, V> {
    pub fn new(size: usize) -> Self {
        Self { 
            buckets: vec![Vec::new(); size],
            size,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let index = hash_to_index(&key, self.size);
        self.buckets[index].push((key, value));
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = hash_to_index(key, self.size);
        for (local_key, value) in &self.buckets[index] {
            if local_key == key {
                return Some(value);
            }
        }
        None
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let index = hash_to_index(key, self.size);
        if let Some(pos) = self.buckets[index]
            .iter()
            .position(|(local_key, _)| local_key == key)
        {
            let (_removed_key, removed_value) = self.buckets[index].remove(pos);
            return Some(removed_value);
        }
        None
    }
}

fn main() {
    let mut map = HashMap::new(50);
    
    map.insert("Primeiro".to_string(), "Segundo".to_string());
    map.insert("nome".to_string(), "Estevao".to_string());
    
    println!("{:?}", map);
    
    if let Some(v) = map.get(&"nome".to_string()) {
        println!("Value: {}", v);
    } else {
        println!("No value found");
    }

    map.remove(&"nome".to_string());
    
    if let Some(v) = map.get(&"nome".to_string()) {
        println!("Value: {}", v);
    } else {
        println!("No value found");
    }
    
    println!("{:?}", map);
}

