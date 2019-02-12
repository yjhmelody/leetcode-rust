#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct MinStack {
    top: Option<Box<StackNode>>,
}

#[derive(Debug, Clone)]
struct StackNode {
    val: i32,
    next: Option<Box<StackNode>>,
}

impl MinStack {
    pub fn new() -> Self {
        Self { top: None }
    }

    pub fn is_empty(&self) -> bool {
        match &self.top {
            None => true,
            Some(_) => false,
        }
    }

    pub fn push(&mut self, x: i32) {
        match &self.top {
            None => {
                self._push(x);
            }

            Some(_) => {
                let min = self.get_min();
                if x < min {
                    self._push(x);
                } else {
                    self._push(min);
                }
            }
        }
        self._push(x);
    }

    fn _push(&mut self, x: i32) {
        println!("{}", x);
        let mut node = StackNode { val: x, next: None };
        let next = self.top.take();
        node.next = next;
        self.top = Some(Box::new(node));
    }

    pub fn pop(&mut self) {
        if self.is_empty() {
            return;
        }
        self._pop();
        self._pop();
    }

    fn _pop(&mut self) {
        let node = self.top.take();
        match node {
            Some(mut x) => self.top = x.next.take(),
            None => {}
        };
    }

    pub fn top(&self) -> i32 {
        match &self.top {
            Some(x) => x.val,
            None => unreachable!(),
        }
    }

    pub fn get_min(&self) -> i32 {
        match &self.top {
            None => unreachable!(),
            Some(cur) => match &cur.next {
                None => unreachable!(),
                Some(min) => min.val,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut stack = MinStack::new();
        stack.push(-2);
        stack.push(0);
        stack.push(-1);
        assert_eq!(stack.get_min(), -2);
        assert_eq!(stack.top(), -1);
        stack.pop();
        assert_eq!(stack.get_min(), -2);
    }
}
