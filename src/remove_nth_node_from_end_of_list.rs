#![allow(dead_code)]

use crate::linked_list::ListNode;

// iter twice
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut len = 0;
    let mut p = dummy.as_ref();
    while p.unwrap().next.is_some() {
        len += 1;
        p = p.unwrap().next.as_ref();
    }

    let idx = len - n;
    let mut p = dummy.as_mut();
    for _ in 0..idx {
        p = p.unwrap().next.as_mut();
    }
    let next = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
    p.as_mut().unwrap().next = next;
    dummy.unwrap().next
}

// ref other people's method
pub fn remove_nth_from_end2(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    fn rebuild(head: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, i32) {
        if let Some(mut node) = head {
            let (tail, m) = rebuild(node.next, n);
            if m == n {
                (tail, m + 1)
            } else {
                node.next = tail;
                (Some(node), m + 1)
            }
        } else {
            (None, 1)
        }
    }
    rebuild(head, n).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let l1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));

        assert_eq!(
            remove_nth_from_end(l1, 2),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            }))
        );
    }

    #[test]
    fn test2() {
        let l1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));

        assert_eq!(
            remove_nth_from_end2(l1, 2),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            }))
        );
    }
}
