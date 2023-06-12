// https://leetcode.com/problems/reverse-linked-list/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

// 1 2 3 4 5
pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;

    while let Some(mut node) = head {
        head = node.next;
        node.next = prev;
        prev = Some(node);
    }

    return prev;
}
