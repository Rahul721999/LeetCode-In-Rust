fn main() {
    println!("Hello, world!");
}

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

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,) 
-> Option<Box<ListNode>> 
{
  match (l1, l2){
    (None, None) => None,
    (Some(n), None) | (None, Some(n)) => Some(n),
    (Some(n1) , Some(n2)) =>{
        let sum = n1.val + n2.val;
        if sum< 10{
            Some(Box::new(ListNode { val: sum, next: add_two_numbers(n1.next, n2.next) }))
        }else{
            let carry = Some(Box::new(ListNode::new(1)));
            Some(Box::new(ListNode { val: sum - 10, next: add_two_numbers(add_two_numbers(carry, n1.next), n2.next) }))
        }
    }
  }
}



#[cfg(test)]
mod test{
    use std::vec;

    use crate::{ListNode, add_two_numbers};

    
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


    #[test]
    fn test1(){
        assert_eq!(add_two_numbers(helper(vec![2,4,3]), helper(vec![5,6,4])), helper(vec![7,0,8]))
    }



    #[test]
    fn test2(){
        assert_eq!(add_two_numbers(helper(vec![0]), helper(vec![0])), helper(vec![0]))
    }

    #[test]
    fn test3(){
        assert_eq!(add_two_numbers(helper(vec![9,9,9,9,9,9,9]), helper(vec![9,9,9,9])), 
            helper(vec![8,9,9,9,0,0,0,1]))
    }
}