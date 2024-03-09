// LRU Cache solution using double linkedlist & hashmap ( ~ 0.1ms )
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, LinkedList};

#[derive(Default)]
struct LRUCache {
    m: HashMap<i32, i32>,
    q: LinkedList<i32>,
    c: HashMap<i32, i32>,
    n: usize,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            n: capacity as _,
            ..Default::default()
        }
    }

    fn use_key(&mut self, key: i32) {
        self.q.push_back(key);
        *self.c.entry(key).or_default() += 1;
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&v) = self.m.get(&key) {
            self.use_key(key);
            v
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.m.len() == self.n && !self.m.contains_key(&key) {
            while let Some(v) = self.q.pop_front() {
                if *self.c.entry(v).and_modify(|e| *e -= 1).or_default() == 0 {
                    self.c.remove(&v);
                    self.m.remove(&v);
                    break;
                }
            }
        };
        self.m.insert(key, value);
        self.use_key(key);
    }
}

fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();


    let mut cache = LRUCache::new(2);

    println!("Output: {:?}", cache.get(2)); // Output: -1

    cache.put(1, 1);
    cache.put(2, 2);
    println!("Output: {:?}", cache.get(1)); // Output: 1

    cache.put(3, 3);
    println!("Output: {:?}", cache.get(2)); // Output: -1

    cache.put(4, 4);
    println!("Output: {:?}", cache.get(1)); // Output: -1
    println!("Output: {:?}", cache.get(3)); // Output: 3
    println!("Output: {:?}", cache.get(4)); // Output: 4


    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
