#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::LinkedList;
use std::iter;

// use chain method
pub struct MyHashSet {
    bucket: Vec<LinkedList<i32>>,
    len: i32,
}

impl MyHashSet {
    pub fn new() -> Self {
        let len: i32 = 2047;
        let bucket = iter::repeat(LinkedList::new())
            .take(len as usize)
            .collect::<Vec<LinkedList<i32>>>();
        Self { bucket, len }
    }

    pub fn add(&mut self, key: i32) {
        if self.contains(key) {
            return;
        } else {
            let index = self._hash(key);
            let list = &mut self.bucket[index];
            list.push_front(key);
        }
    }

    pub fn remove(&mut self, key: i32) {
        match self._find(key) {
            None => {}
            // zero never be used
            Some(val) => *val = 0,
        }
    }

    #[inline]
    pub fn contains(&mut self, key: i32) -> bool {
        match self._find(key) {
            Some(_) => true,
            None => false,
        }
    }

    #[inline]
    fn _find(&mut self, key: i32) -> Option<&mut i32> {
        let index = self._hash(key);
        let list = &mut self.bucket[index];
        for val in list.iter_mut() {
            if *val == key {
                return Some(val);
            }
        }

        None
    }

    #[inline]
    fn _hash(&self, key: i32) -> usize {
        key as usize % self.len as usize
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test1() {}
}
