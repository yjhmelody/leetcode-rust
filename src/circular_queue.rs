#![allow(dead_code)]
#![allow(unused_variables)]

use std::iter;

// One more space
pub struct MyCircularQueue {
    q: Vec<i32>,
    head: usize,
    tail: usize,
    size: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    /** Initialize your data structure here. Set the size of the queue to be k. */
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

    /** Insert an element into the circular queue. Return true if the operation is successful. */
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

    /** Delete an element from the circular queue. Return true if the operation is successful. */
    pub fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.head += 1;
            self.head %= self.size;
            true
        }
    }

    /** Get the front item from the queue. */
    pub fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.q[self.head]
        }
    }

    /** Get the last item from the queue. */
    pub fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.q[(self.tail + self.size - 1) % self.size]
        }
    }

    /** Checks whether the circular queue is empty or not. */
    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    /** Checks whether the circular queue is full or not. */
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
