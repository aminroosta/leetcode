// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }

// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>)
        -> Option<Box<ListNode>> {
            let mut l1 = &l1;
            let mut l2 = &l2;

            let mut head : Option<Box<ListNode>> = None;
            let mut last = &mut head;
            let mut extra = 0;
            loop {
                match (l1, l2) {
                    (Some(a), Some(b)) => {
                        extra += a.val + b.val;
                        l1 = &a.next;
                        l2 = &b.next;
                    },
                    (Some(a), None) => {
                        extra += a.val;
                        l1 = &a.next;
                    },
                    (None, Some(b)) => {
                        extra += b.val;
                        l2 = &b.next;
                    },
                    (None, None) => {
                        if(extra == 0) {
                            break;
                        }
                    }
                };
                if let Some(ref mut node) = last {
                    node.next = Some(Box::new(ListNode::new(extra%10)));
                    last = &mut node.next;
                } else {
                    *last = Some(Box::new(ListNode::new(extra%10)));
                }
                extra /= 10;
            };
            return head;
    }
} 
