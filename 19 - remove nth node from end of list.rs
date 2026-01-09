/* 19 - Remove Nth Node From End of List

    Given the head of a linked list, remove the nth node from the end of the list and return its head.

    Example 1:
        Input: head = [1,2,3,4,5], n = 2
        Output: [1,2,3,5]

    Example 2:
        Input: head = [1], n = 1
        Output: []

    Example 3:
        Input: head = [1,2], n = 1
        Output: [1]
 */

/* My accepted solution */
//
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;
        let mut length = 0;

        let mut ptr = dummy.next.as_ref();          // ptr -> Option<&Box<ListNode>>
        while let Some(node) = ptr{
            ptr = node.next.as_ref();
            length += 1;
        }

        let mut ptr = &mut dummy;                   // ptr -> &mut Box<ListNode>
        for _ in 0..(length - n){
            ptr = ptr.next.as_mut().unwrap();
        }

        let next = ptr.next.take();                 // next -> Option<Box<ListNode>>
        if let Some(mut node) = next{
            ptr.next = node.next.take();
        }

        return dummy.next
    }
}
