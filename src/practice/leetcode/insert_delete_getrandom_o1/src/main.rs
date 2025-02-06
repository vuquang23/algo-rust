// https://leetcode.com/problems/insert-delete-getrandom-o1/

use rand::Rng;
use std::collections::HashSet;

struct RandomizedSet {
    list: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        Self {
            list: HashSet::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        self.list.insert(val)
    }

    fn remove(&mut self, val: i32) -> bool {
        self.list.remove(&val)
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..self.list.len());
        *self.list.iter().nth(idx).unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
