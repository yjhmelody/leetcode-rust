#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::binary_tree::TreeNode;

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    preorder(root, &mut res);
    res
}

fn preorder(root: Option<Rc<RefCell<TreeNode>>>, mut res: &mut Vec<i32>) {
    match root {
        None => {}
        Some(node) => {
            res.push(node.borrow().val);
            preorder(node.borrow().left.clone(), &mut res);
            preorder(node.borrow().right.clone(), &mut res);
        }
    }
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
        let res = preorder_traversal(tree);
        assert_eq!(res, vec![1, 2, 3]);
    }
}
