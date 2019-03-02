#![allow(dead_code)]

use std::collections::LinkedList;

/// double stack (or double vec)
#[derive(Default)]
struct MyQueue {
    list1: LinkedList<i32>,
    list2: LinkedList<i32>,
}

impl MyQueue {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Default::default()
    }

    /** Push element x to the back of queue. */
    pub fn push(&mut self, x: i32) {
        self.list1.push_back(x);
    }

    fn _move(&mut self) {
        if self.list2.is_empty() {
            while let Some(x) = self.list1.pop_back() {
                self.list2.push_back(x);
            }
        }
    }

    /** Removes the element from in front of queue and returns that element. */
    pub fn pop(&mut self) -> i32 {
        self._move();
        self.list2.pop_back().unwrap()
    }

    /** Get the front element. */
    pub fn peek(&mut self) -> i32 {
        self._move();
        *self.list2.back().unwrap()
    }

    /** Returns whether the queue is empty. */
    pub fn empty(&self) -> bool {
        self.list1.is_empty() && self.list2.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut q = MyQueue::new();
        assert_eq!(q.empty(), true);
        q.push(1);
        q.push(2);
        q.push(3);
        let res = q.pop();
        assert_eq!(res, 1);

        q.push(4);
        let res = q.pop();
        assert_eq!(res, 2);

        q.push(5);
        q.push(6);
        let res = q.pop();
        assert_eq!(res, 3);

        q.push(7);
        assert_eq!(q.peek(), 4);
    }
}
