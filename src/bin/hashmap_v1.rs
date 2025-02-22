use sha2::{Sha256, Digest};
use std::convert::TryInto;

/// Hashes a string and maps it to an index within the array size
fn hash_to_index(value: &str, arr_size: usize) -> usize {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    let hash_result = hasher.finalize();
    let int_value = u64::from_be_bytes(hash_result[..8].try_into().unwrap());

    (int_value % arr_size as u64) as usize
}

#[derive(Debug)]
pub struct HashMap {
    arr: Vec<Option<(String, String)>>,
    size: usize,
}

impl HashMap {
    pub fn new(size: usize) -> Self {
        Self { 
            arr: vec![None; size],
            size,
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        let index = hash_to_index(&key, self.size);
        self.arr[index] = Some((key, value));
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        let index = hash_to_index(key, self.size);
        match &self.arr[index] {
            Some((_, value)) => Some(value),
            None => None,
        }
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        let index = hash_to_index(key, self.size);
        match self.arr[index].take() {
            Some((_, value)) => Some(value),
            None => None,
        }
    }
}

fn main() {
    let mut map = HashMap::new(50);
    
    map.insert("Primeiro".to_string(), "Segundo".to_string());
    map.insert("nome".to_string(), "Estevao".to_string());
    
    println!("{:?}", map);
    
    if let Some(v) = map.get("nome") {
        println!("Value: {}", v);
    } else {
        println!("No value found");
    }

    map.remove("nome");
    
    if let Some(v) = map.get("nome") {
        println!("Value: {}", v);
    } else {
        println!("No value found");
    }

    println!("{:?}", map);
}

