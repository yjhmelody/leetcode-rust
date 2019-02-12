#![allow(dead_code)]
#![allow(unused_variables)]

use std::iter;

// One more space
#[derive(Debug, Clone)]
pub struct MyCircularQueue {
    q: Vec<i32>,
    head: usize,
    tail: usize,
    size: usize,
}

impl MyCircularQueue {
    pub fn new(k: i32) -> Self {
        let size = k as usize + 1;
        let q = iter::repeat(0).take(size.clone()).collect::<Vec<i32>>();
        Self {
            q,
            head: 0,
            tail: 0,
            size,
        }
    }

    pub fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.q[self.tail] = value;
            self.tail += 1;
            self.tail %= self.size;
            true
        }
    }

    pub fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.head += 1;
            self.head %= self.size;
            true
        }
    }

    pub fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.q[self.head]
        }
    }

    pub fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.q[(self.tail + self.size - 1) % self.size]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    pub fn is_full(&self) -> bool {
        (self.tail + 1) % self.size == self.head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let k = 10;
        let value = 10;
        let mut obj = MyCircularQueue::new(k);
        let ret_1: bool = obj.en_queue(value);
        let ret_2: bool = obj.de_queue();
        let ret_3: i32 = obj.front();
        let ret_4: i32 = obj.rear();
        let ret_5: bool = obj.is_empty();
        let ret_6: bool = obj.is_full();
    }
}
