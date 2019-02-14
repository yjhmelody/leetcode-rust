#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::binary_tree::TreeNode;

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    let mut q = VecDeque::new();
    q.push_back((root, 0));

    while !q.is_empty() {
        let cur = q.pop_front().unwrap();
        let depth = cur.1;

        let cur = cur.0;
        match cur {
            Some(node) => {
                if depth >= res.len() {
                    res.push(vec![]);
                }
                res[depth].push(node.borrow().val);
                q.push_back((node.borrow().left.clone(), depth + 1));
                q.push_back((node.borrow().right.clone(), depth + 1));
            }
            _ => {}
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));
        let res = level_order(tree);
        assert_eq!(res, vec![vec![1], vec![2], vec![3]]);
    }
}
