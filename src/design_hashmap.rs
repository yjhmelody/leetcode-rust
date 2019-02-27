#![allow(dead_code)]

use std::collections::LinkedList;
use std::iter;

type Pair = (i32, i32);

struct MyHashMap {
    bucket: Vec<LinkedList<Pair>>,
    len: i32,
}

impl MyHashMap {
    /** Initialize your data structure here. */
    fn new() -> Self {
        let len: i32 = 2047;
        let bucket = iter::repeat(LinkedList::new())
            .take(len as usize)
            .collect::<Vec<LinkedList<Pair>>>();
        Self { bucket, len }
    }

    /** value will always be non-negative. */
    fn put(&mut self, key: i32, value: i32) {
        match self._find(key) {
            Some(pair) => {
                (*pair).1 = value;
            }
            None => {
                let index = self._hash(key);
                let list = &mut self.bucket[index];
                list.push_front((key, value));
            }
        }
    }

    /** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
    fn get(&mut self, key: i32) -> i32 {
        match self._find(key) {
            Some(pair) => (*pair).1,
            None => -1,
        }
    }

    /** Removes the mapping of the specified value key if this map contains a mapping for the key */
    fn remove(&mut self, key: i32) {
        match self._find(key) {
            None => {}
            // zero never be used
            Some(val) => {
                (*val).1 = -1;
            }
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
    fn _find(&mut self, key: i32) -> Option<&mut Pair> {
        let index = self._hash(key);
        let list = &mut self.bucket[index];
        for val in list.iter_mut() {
            if (*val).0 == key {
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
    use super::*;

    #[test]
    fn test1() {
        let mut obj = MyHashMap::new();
        obj.put(1, 1);
        obj.put(2, 2);
        let res = obj.get(1);
        assert_eq!(res, 1);
        let res = obj.get(3);
        assert_eq!(res, -1);
        obj.put(2, 1);
        let res = obj.get(2);
        assert_eq!(res, 1);
        obj.remove(2);
        let res = obj.get(2);
        assert_eq!(res, -1);
    }
}
