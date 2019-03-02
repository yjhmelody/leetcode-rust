#![allow(dead_code)]

use std::borrow::BorrowMut;
use std::collections::LinkedList;

#[derive(Default)]
struct MyStack {
    list1: LinkedList<i32>,
    list2: LinkedList<i32>,
    switch: bool,
}

/// Maybe there are some else better methods
impl MyStack {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
        if self.switch {
            self.list1.push_back(x);
        } else {
            self.list2.push_back(x);
        }
    }

    /** Removes the element on top of the stack and returns that element. */
    fn pop(&mut self) -> i32 {
        let q1;
        let q2;
        if self.switch {
            q1 = self.list1.borrow_mut();
            q2 = self.list2.borrow_mut();
        } else {
            q1 = self.list2.borrow_mut();
            q2 = self.list1.borrow_mut();
        }
        self.switch = !self.switch;

        while q1.len() != 1 {
            if let Some(x) = q1.pop_front() {
                q2.push_back(x);
            }
        }
        q1.pop_front().unwrap()
    }

    /** Get the top element. */
    fn top(&mut self) -> i32 {
        let q1;
        let q2;
        if self.switch {
            q1 = self.list1.borrow_mut();
            q2 = self.list2.borrow_mut();
        } else {
            q1 = self.list2.borrow_mut();
            q2 = self.list1.borrow_mut();
        }
        self.switch = !self.switch;

        while q1.len() != 1 {
            if let Some(x) = q1.pop_front() {
                q2.push_back(x);
            }
        }

        let res = q1.pop_front().unwrap();
        q2.push_back(res);
        res
    }

    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        self.list1.is_empty() && self.list2.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut q = MyStack::new();
        assert_eq!(q.empty(), true);

        q.push(1);
        q.push(2);
        let res = q.top();
        assert_eq!(res, 2);

        q.push(3);
        let res = q.pop();
        assert_eq!(res, 3);

        let res = q.top();
        assert_eq!(res, 2);

        q.push(4);
        let res = q.pop();
        assert_eq!(res, 4);

        q.push(5);
        q.push(6);
        let res = q.pop();
        assert_eq!(res, 6);

        q.push(7);
        assert_eq!(q.top(), 7);
    }
}
