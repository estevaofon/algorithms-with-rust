use sha2::{Sha256, Digest};
use std::convert::TryInto;

/// A trait for types that can be hashed into a byte vector.
/// It also provides a default method to map a hash to an index.
pub trait Hashable {
    fn hash(&self) -> Vec<u8>;

    fn hash_to_index(&self, arr_size: usize) -> usize {
        let int_value = u64::from_be_bytes(self.hash()[..8].try_into().unwrap());
        (int_value % arr_size as u64) as usize
    }
}

/// Implement `Hashable` for any type that implements `ToString`.
impl<T: ToString> Hashable for T {
    fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.to_string().as_bytes());
        hasher.finalize().as_slice().to_vec()
    }
}

#[derive(Debug)]
pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    size: usize,
}

impl<K: Hashable + Clone + PartialEq, V: Clone> HashMap<K, V> {
    pub fn new(size: usize) -> Self {
        Self { 
            buckets: vec![Vec::new(); size],
            size,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let index = key.hash_to_index(self.size);
        self.buckets[index].push((key, value));
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = key.hash_to_index(self.size);
        for (local_key, value) in &self.buckets[index] {
            if local_key == key {
                return Some(value);
            }
        }
        None
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let index = key.hash_to_index(self.size);
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

