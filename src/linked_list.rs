#![allow(dead_code)]

#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

//#[macro_export]
//macro_rules! linkedlist {
//    () => { None };
//    ($($e: expr), *) => {
//        {
//            let mut head = Box::new($crate::ListNode::new(0));
//            let mut ref_head = &mut head;
//
//            $(
//               ref_head.next = Some(Box::new($crate::ListNode::new($e)));
//               ref_head = ref_head.next.as_mut().unwrap();
//            )*
//
//            // avoid reassign
//            let _ = ref_head;
//            head.next
//        }
//    };
//}

//#[cfg(test)]
//mod tests {
//    #[test]
//    fn test1() {
//        let mut list = linkedlist![1, 2, 3];
//    }
//}
