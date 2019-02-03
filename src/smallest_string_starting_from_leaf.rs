#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::binary_tree::TreeNode;

// todo

//pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//    let s = preorder_traversal(root);
//    let min_val = s.iter().min_by(|&a, &b| (*a).0.cmp(&(*b).0));
//
//    (*min_val.unwrap()).1.borrow().val
//}
//
//pub fn preorder_traversal(
//    root: Option<Rc<RefCell<TreeNode>>>,
//) -> Vec<(i32, Rc<RefCell<TreeNode>>)> {
//    let mut res: Vec<(i32, Rc<RefCell<TreeNode>>)> = vec![];
//    preorder(root, &mut res);
//    res
//}
//
//fn preorder(root: Option<Rc<RefCell<TreeNode>>>, mut res: &mut Vec<(i32, Rc<RefCell<TreeNode>>)>) {
//    match root {
//        None => {}
//        Some(node) => {
//            preorder(node.borrow().left.clone(), &mut res);
//            preorder(node.borrow().right.clone(), &mut res);
//            match (node.borrow().left, node.borrow().right) {
//                (None, None) => {
//                    let val = (node.borrow().val, node.clone());
//                    res.push(val);
//                }
//
//                _ => {}
//            }
//        }
//    }
//}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test1() {
//        let tree = Some(Rc::new(RefCell::new(TreeNode {
//            val: 1,
//            left: None,
//            right: Some(Rc::new(RefCell::new(TreeNode {
//                val: 2,
//                left: Some(Rc::new(RefCell::new(TreeNode {
//                    val: 3,
//                    left: None,
//                    right: None,
//                }))),
//                right: None,
//            }))),
//        })));
//        assert_eq!(smallest_from_leaf(tree), 1);
//    }
//}
