// LRU Cache solution in Rust using double linkedlist & hashmap ( ~ 98.276µs (0.098276 ms) )
// Runtime: 149ms - Beats 53.97%of users with Rust
// Memory: 103.95mb - Beats 41.27%of users with Rust
#![allow(unused)]
use std::collections::hash_map::Entry::{Occupied, Vacant};
// use std::collections::{HashMap, LinkedList};
use std::collections::HashMap;  // self coded LinkedList has slightly better performance

struct LRUCache{
    capacity: usize,
    list: LinkedList,
    map: HashMap<i32, (i32, *mut Node)>,
}

#[derive(Clone)]
struct Node {
    data: i32,
    next: Option<Box<Node>>,
    prev: Option<*mut Node>,
}

struct LinkedList {
    head: Option<Box<Node>>,
    tail: Option<*mut Node>,
}

impl LinkedList {
    fn new() -> Self {
        Self { head: None, tail: None }
    }

    fn push_front(&mut self, value: i32) -> *mut Node {
        let mut new_node = Box::new(Node {
            data: value,
            next: None,
            prev: None,
        });
        let mut head = self.head.take();
        if let Some(ref mut head) = head {
            head.prev = Some(&mut *new_node as *mut Node);
        } else {
            self.tail = Some(&mut *new_node as *mut Node);
        }
        new_node.next = head;
        let a = &mut *new_node as *mut Node;
        self.head = Some(new_node);
        a
    }

    fn pop_back(&mut self) -> Option<i32> {
        self.tail.take().map(|tail| unsafe {
            let tail = &*tail;
            if let Some(prev) = tail.prev {
                (*prev).next = None;
                self.tail = Some(prev);
            } else {
                self.head = None;
            }
            tail.data
        })
    }

    fn remove(&mut self, node: *mut Node) {
        let mut node = unsafe { &mut *node };
        match (node.prev.take(), node.next.take()) {
            (Some(prev), Some(mut next)) => unsafe {
                next.prev = Some(prev);
                (*prev).next = Some(next);
            }
            (Some(prev), None) => {
                unsafe { (*prev).next = None; }
                self.tail = Some(prev);
            }
            (None, Some(mut next)) => {
                next.prev = None;
                self.head = Some(next);
            }
            (None, None) => {
                self.head = None;
                self.tail = None;
            }
        }
    }
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            list: LinkedList::new(),
            map: HashMap::new(),
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        if let Some((value, node)) = self.map.remove(&key) {
            self.list.remove(node);
            let node = self.list.push_front(key);
            self.map.insert(key, (value, node));
            value
        } else {
            -1
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        if let Some((_, node)) = self.map.remove(&key) {
            self.list.remove(node);
        } else if self.map.len() == self.capacity {
            if let Some(key) = self.list.pop_back() {
                self.map.remove(&key);
            }
        }
        let node = self.list.push_front(key);
        self.map.insert(key, (value, node));
    }
}

fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();


    let mut cache = LRUCache::new(3); // Create a new LRUCache with a capacity of 3

    // Test the LRUCache operations
    cache.put(1, 1);
    cache.put(2, 2);
    cache.put(3, 3);
    cache.put(4, 4); // This will evict key 1 as the capacity is reached

    println!("Get key 1: {}", cache.get(1)); // Output: -1 (not found, as it was evicted)
    println!("Get key 2: {}", cache.get(2)); // Output: 2
    println!("Get key 3: {}", cache.get(3)); // Output: 3
    println!("Get key 4: {}", cache.get(4)); // Output: 4

    cache.put(5, 5); // This will evict key 2 as it's the least recently used

    println!("Get key 2: {}", cache.get(2)); // Output: -1 (not found, as it was evicted)
    println!("Get key 3: {}", cache.get(3)); // Output: 3
    println!("Get key 4: {}", cache.get(4)); // Output: 4
    println!("Get key 5: {}", cache.get(5)); // Output: 5


    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
