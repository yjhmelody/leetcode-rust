#![allow(dead_code)]

use crate::linked_list::ListNode;

// add two link lists by transfer them to vector firstly and then transfer them back to lists.
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut vec1 = list_to_vec(l1);
    let mut vec2 = list_to_vec(l2);
    let mut vec3 = vec![];
    let mut carry = 0;
    let min_len = vec1.len().min(vec2.len());
    let max_len = vec1.len().max(vec2.len());
    for i in 0 .. min_len {
        let mut val = vec1[i] + vec2[i] + carry;
        if val > 9 {
            val -= 10;
            vec3.push(val);
            carry = 1;
        } else {
            vec3.push(val);
            carry = 0;
        }
    }

    let rest_vec = if max_len == vec1.len() {
            vec1
        } else {
            vec2
    };

    for i in min_len .. max_len {
        let mut val = rest_vec[i] + carry;
        if val > 9 {
            val -= 10;
            vec3.push(val);
            carry = 1;
        } else {
            vec3.push(val);
            carry = 0;
        }
    }

    if carry == 1 {
        vec3.push(1);
    }
    // The last == 0 and the before of last == 10
    let last = vec3.len() - 1;
    if vec3.len() > 1 && vec3[last] == 10 {
        vec3[last] -= 10;
        vec3.push(1);
    }
    vec_to_list(vec3)
}

fn list_to_vec(l: Option<Box<ListNode>>) -> Vec<i32> {
    let mut nums = vec![];
    let mut list = l;
    while list.is_some() {
        match list {
            Some(ref node) => nums.push(node.val),
            None => {},
        };
        list = list.unwrap().next;
    }
    nums
}

fn vec_to_list(arr: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Box::new(ListNode::new(0));

    let mut cur = &mut head;
    for num in arr.iter() {
        cur.next = Some(Box::new(ListNode::new(*num)));
        cur = cur.next.as_mut().unwrap();
    }

    head.next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: None,
                }))
            }))
        }));
        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None,
                }))
            }))
        }));
        let l3 = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: 8,
                    next: None,
                }))
            }))
        }));

        assert_eq!(add_two_numbers(l1, l2), l3);
    }
}