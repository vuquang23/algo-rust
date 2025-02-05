// https://leetcode.com/problems/reorder-list/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut current = head.as_ref();
        let mut length = 0;

        while let Some(node) = current {
            current = node.next.as_ref();
            length += 1;
        }

        let mut middle = head.as_mut();
        let mut count = 1;

        while count < (length + 1) / 2 {
            if let Some(node) = middle {
                middle = node.next.as_mut();
                count += 1;
            } else {
                break;
            }
        }

        let second_half = match middle {
            None => None,
            Some(node) => {
                let mut previous = None;
                let mut current = node.next.take();

                while let Some(mut inner_node) = current {
                    current = inner_node.next.take();
                    inner_node.next = previous.take();
                    previous = Some(inner_node);
                }

                previous
            }
        };

        let mut first_half = head.take();
        let mut second_half = second_half;

        *head = Solution::merge_two_lists(first_half, second_half);
    }

    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1.as_mut(), l2.as_mut()) {
            (None, None) => None,
            (Some(_), None) => l1,
            (None, Some(_)) => l2,
            (Some(node1), Some(_)) => {
                node1.next = Self::merge_two_lists(l2, node1.next.take());
                l1
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
