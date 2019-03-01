#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::binary_tree::TreeNode;

pub fn find_duplicate_subtrees(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    let mut res = vec![];
    match root {
        None => res,
        root => {
            post_order(root, &mut HashMap::new(), &mut res);
            res
        }
    }
}

// post order with serialization
fn post_order(
    root: Option<Rc<RefCell<TreeNode>>>,
    mut map: &mut HashMap<String, i32>,
    mut res: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
) -> String {
    match root {
        None => "#".to_string(),
        Some(node) => {
            let val = node.borrow().val;
            let serial = format!("{}", val)
                + &post_order(node.borrow().left.clone(), &mut map, &mut res)
                + " "
                + &post_order(node.borrow().right.clone(), &mut map, &mut res);

            match map.get_mut(&serial) {
                Some(v) => {
                    *v += 1;
                    if *v == 2 {
                        res.push(Some(node))
                    }
                }

                None => {
                    map.insert(serial.clone(), 1);
                }
            }

            serial
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
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
        })));

        let res = vec![
            Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: None,
            }))),
        ];

        assert_eq!(find_duplicate_subtrees(tree), res);
    }
}
