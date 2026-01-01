/* 21 - Merge Two Sorted Lists

    You are given the heads of two sorted linked lists list1 and list2.
    Merge the two lists into one sorted list.
    The list should be made by splicing together the nodes of the first two lists.
    Return the head of the merged linked list.

    Example 1:
        Input: list1 = [1,2,4], list2 = [1,3,4]
        Output: [1,1,2,3,4,4]

    Example 2:
        Input: list1 = [], list2 = []
        Output: []

    Example 3:
        Input: list1 = [], list2 = [0]
        Output: [0]
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
//   #[inline] // usually used for small, frequently called functions to improve performance
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = list1;                           // l1 -> Option<Box<ListNode>>
        let mut l2 = list2;                           // l2 -> Option<Box<ListNode>>
        let mut new_list = Box::new(ListNode::new(0));
        let mut output = &mut new_list;

        if l1.is_none() {
            return l2;
        }
        if l2.is_none() {
            return l1;
        }
        while l1.is_some() && l2.is_some(){
            let take_l1 = {
                let v1 = l1.as_ref().unwrap().val;
                let v2 = l2.as_ref().unwrap().val;
                v1 <= v2
            };
            if take_l1 {
                let mut node = l1.take().unwrap();   // node -> Box<ListNode>
                l1 = node.next.take();               // node.next -> Option<Box<ListNode>>
                output.next = Some(node);
            } else {
                let mut node = l2.take().unwrap();
                l2 = node.next.take();
                output.next = Some(node);
            }
            output = output.next.as_mut().unwrap();  // output -> &mut Box<ListNode>
        }
        output.next = if l1.is_some() { l1 } else { l2 };
        new_list.next
    }
}
