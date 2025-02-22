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
    buckets: Vec<Vec<Option<(K, V)>>>,
    size: usize,
}

impl<K: ToString + std::clone::Clone + std::cmp::PartialEq, V: std::clone::Clone> HashMap<K, V> {
    pub fn new(size: usize) -> Self {
        Self { 
            buckets: vec![Vec::new(); size],
            size,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let index = hash_to_index(&key, self.size);
        // self.arr[index] = Some((key, value));
        let arr = &mut self.buckets[index];
        arr.push(Some((key, value)));
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = hash_to_index(key, self.size);
        let arr = &self.buckets[index];
        for i in arr {
            if let Some((local_key, value)) = i {
                if local_key == key {
                    return Some(value);
                }
            }
        }
        None
    }

    //// This remove method uses the same match structure as the original implementation.
    //pub fn remove(&mut self, key: &K) -> Option<V> {
    //    let index = hash_to_index(key, self.size);
    //    match self.arr[index].take() {
    //        Some((_, value)) => Some(value),
    //        None => None,
    //    }
    //}
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

    //map.remove(&"nome".to_string());
    //
    //if let Some(v) = map.get(&"nome".to_string()) {
    //    println!("Value: {}", v);
    //} else {
    //    println!("No value found");
    //}
    //println!("{:?}", map);
    //let mut map2 = HashMap::new(50);
    //map2.insert("id".to_string(), 1234);
    //println!("{:?}", map2);

}

