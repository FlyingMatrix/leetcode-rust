/* 92 - Reverse Linked List II

    Given the head of a singly linked list and two integers left and right where left <= right,
    reverse the nodes of the list from position left to position right, and return the reversed list.

    Example 1:
        Input: head = [1,2,3,4,5], left = 2, right = 4
        Output: [1,4,3,2,5]

    Example 2:
        Input: head = [5], left = 1, right = 1
        Output: [5]
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
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        if left == right{
            return head
        }
        if let Some(node) = head.as_ref(){
            if node.next.is_none(){
                return head
            }
        }

        let mut dummy = Box::new(ListNode::new(0));              // dummy -> Box<ListNode>
        dummy.next = head;
        let mut prev = &mut dummy;                               // prev -> &mut Box<ListNode>
        for _ in 0..left-1{
            prev = prev.next.as_mut().unwrap();
        }

        let mut curr = prev.next.take();                         // curr -> Option<Box<ListNode>>
        for _ in 0..right-left{
            let mut temp = curr.as_mut().unwrap().next.take();   // temp -> Option<Box<ListNode>>
            curr.as_mut().unwrap().next = temp.as_mut().unwrap().next.take();
            temp.as_mut().unwrap().next = prev.next.take();
            prev.next = Some(temp.unwrap());
        }

        let mut tail = prev.next.as_mut().unwrap();              // tail -> &mut Box<ListNode>
        while tail.next.is_some(){
            tail = tail.next.as_mut().unwrap();
        }
        tail.next =curr;

        return dummy.next
    }
}
