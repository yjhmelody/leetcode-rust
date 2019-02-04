#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::binary_tree::TreeNode;

pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    postorder(root, &mut res);
    res
}

fn postorder(root: Option<Rc<RefCell<TreeNode>>>, mut res: &mut Vec<i32>) {
    match root {
        None => {}
        Some(node) => {
            postorder(node.borrow().left.clone(), &mut res);
            postorder(node.borrow().right.clone(), &mut res);
            res.push(node.borrow().val);
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
        let res = postorder_traversal(tree);
        assert_eq!(res, vec![3, 2, 1]);
    }
}
