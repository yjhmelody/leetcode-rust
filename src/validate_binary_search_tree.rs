#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::binary_tree::TreeNode;

// failed: need to consider the minimum boundary
pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    use std::i32;
    let mut min_val = i32::min_value();
    _is_valid_bst(root, &mut min_val)
}

fn _get_left_val(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut cur = root;
    let mut prev = cur.clone();
    while cur.is_some() {
        prev = cur.clone();
        cur = cur.unwrap().borrow().left.clone();
    }

    prev.unwrap().borrow().val
}

fn _is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>, mut val: &mut i32) -> bool {
    match root {
        None => true,
        Some(node) => {
            if _is_valid_bst(node.borrow().left.clone(), &mut val) && *val < node.borrow().val {
                *val = node.borrow().val;
                _is_valid_bst(node.borrow().right.clone(), &mut val)
            } else {
                false
            }
        }
    }
}

pub fn is_valid_bst2(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let vals = inorder_traversal(root);
    if vals.len() == 0 {
        return true;
    }
    for i in 0..vals.len() - 1 {
        if vals[i] >= vals[i + 1] {
            return false;
        }
    }
    true
}

fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    inorder(root, &mut res);
    res
}

fn inorder(root: Option<Rc<RefCell<TreeNode>>>, mut res: &mut Vec<i32>) {
    match root {
        None => {}
        Some(node) => {
            inorder(node.borrow().left.clone(), &mut res);
            res.push(node.borrow().val);
            inorder(node.borrow().right.clone(), &mut res);
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
        assert_eq!(is_valid_bst(tree), false);

        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));
        assert_eq!(is_valid_bst(tree), true);
    }

    #[test]
    fn test2() {
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
        assert_eq!(is_valid_bst2(tree), false);

        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));
        assert_eq!(is_valid_bst2(tree), true);

        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        assert_eq!(is_valid_bst2(tree), true);
        assert_eq!(is_valid_bst2(None), true);
    }
}
