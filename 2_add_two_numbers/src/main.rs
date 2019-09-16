// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

struct Solution {}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    pub fn equals(&mut self, ln: &Option<Box<ListNode>>) -> bool {
        if ln.is_none() {
            return false;
        }
        let mut p = self;
        let mut q = ln.as_ref().unwrap();
        if p.val != q.val {
            return false;
        }
        while p.next != None {
            if let Some(ref mut _p) = p.next {
                p = _p
            }
            q = q.next.as_ref().unwrap();
            if p.val != q.val {
                return false;
            }
        }
        return true;
    }
}

impl ListNode {
    pub fn push_back(&mut self, val: i32) {
        let mut p = self;
        while p.next != None {
            if let Some(ref mut _p) = p.next {
                p = _p
            } else {
                break
            }
        }
        p.next = Some(Box::new(ListNode::new(val)))
    }
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = Box::new(ListNode::new(0));
        let mut next = &mut result;
        let mut next1 = l1;
        let mut next2 = l2;
        let mut carry = 0;
        let mut first = true;

        while next1 != None || next2 != None {
            let mut sum = carry;
            if next1 != None {
                let l = next1.unwrap();
                sum += l.val;
                next1 = l.next;
            }
            if next2 != None {
                let l = next2.unwrap();
                sum += l.val;
                next2 = l.next;
            }

            if first {
                first = false;
                if sum > 9 {
                    next.val = sum - 10;
                    carry = 1
                } else {
                    next.val = sum;
                    carry = 0
                }
            } else {
                if sum > 9 {
                    next.next = Some(Box::new(ListNode::new(sum - 10)));
                    carry = 1;
                } else {
                    next.next = Some(Box::new(ListNode::new(sum)));
                    carry = 0;
                }
                next = next.next.as_mut().unwrap();
            }
        }
        if carry != 0 {
            next.next = Some(Box::new(ListNode::new(carry)));
        }
        Some(result)
    }
}

fn main() {
    // 2 -> 4 -> 3
    let mut n1 = Box::new(ListNode::new(2));
    n1.push_back(4);
    n1.push_back(3);

    // 5 -> 6 -> 4
    let mut n2 = Box::new(ListNode::new(5));
    n2.push_back(6);
    n2.push_back(4);

    let mut expect_sum = Box::new(ListNode::new(7));
    expect_sum.push_back(0);
    expect_sum.push_back(8);

    let sum = Solution::add_two_numbers(Some(n1), Some(n2));
    if expect_sum.equals(&sum) {
        println!("PASS");
    } else {
        println!("{:?}", sum);
    }
}
