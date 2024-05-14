fn main() {
  assert_eq!(Solution::merge_two_lists(helper(vec![1,2,4]), helper(vec![1,3,4])), helper(vec![1,1,2,3,4,4]));
}


// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

// helper fn to create LinkedList
fn helper(v : Vec<i32>) ->  Option<Box<ListNode>>{
  // create a head first
  let mut head = Box::new(ListNode::new(0));
  // apoint head as current node
  let mut curr_node = &mut head;

  for i in v{
      curr_node.next = Some(Box::new(ListNode::new(i)));
      curr_node = curr_node.next.as_mut().unwrap();
  }
  head.next

}

struct Solution;
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      match (list1, list2){
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => {
          Some(n)
        },
        (Some(mut n1), Some(mut n2)) => {
          if n1.val > n2.val{
            n2.next = Solution::merge_two_lists(Some(n1), n2.next);
            return Some(n2)
          }else{
            n1.next = Solution::merge_two_lists(n1.next, Some(n2));
            return Some(n1)
          }
        }
      }
    }
}