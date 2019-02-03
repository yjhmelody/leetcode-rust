#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::binary_tree::TreeNode;

struct Solution;

// local success, remote failure
impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::iter;

        let mut left: i32 = 0;
        let mut right: i32 = 0;
        preorder(root.clone(), 0, 0, &mut left, &mut right);
        let mut res = iter::repeat(vec![])
            .take((right + 1 - left) as usize)
            .collect::<Vec<Vec<i32>>>();

        preorder2(root, &mut res, 0, 0, left.clone());

        res
    }
}

fn preorder(
    root: Option<Rc<RefCell<TreeNode>>>,
    mut x: i32,
    mut y: i32,
    left: &mut i32,
    right: &mut i32,
) {
    match root {
        None => {}
        Some(node) => {
            *left = *left.min(&mut x);
            *right = *right.max(&mut x);
            preorder(node.borrow().left.clone(), x - 1, y - 1, left, right);
            preorder(node.borrow().right.clone(), x + 1, y - 1, left, right);
        }
    }
}

fn preorder2(
    root: Option<Rc<RefCell<TreeNode>>>,
    mut res: &mut Vec<Vec<i32>>,
    x: i32,
    y: i32,
    left: i32,
) {
    match root {
        None => {}
        Some(node) => {
            res[(x - left) as usize].push(node.borrow().val);
            preorder2(node.borrow().left.clone(), &mut res, x - 1, y - 1, left);
            preorder2(node.borrow().right.clone(), &mut res, x + 1, y - 1, left);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));

        assert_eq!(
            Solution::vertical_traversal(tree),
            vec![vec![0, 2], vec![1]]
        );
    }
}
