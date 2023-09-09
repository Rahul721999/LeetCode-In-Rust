fn main() {
    let head = Some(Box::new(ListNode{val : 1, next:Some(Box::new(ListNode{val : 2, next:Some(Box::new(ListNode{val : 3, next:None}))}))}));
    let k = 5;
    let res = split_list_to_parts(head, k);
    let actual_res = vec![Some(Box::new(ListNode{val : 1, next : None})),Some(Box::new(ListNode{val : 2, next : None})),Some(Box::new(ListNode{val : 3, next : None})), None, None];
    assert_eq!(res,actual_res);
}

 pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        
    // first get the size of the linked list
    let mut cursor = &head;
    let mut list_len = 0;
    while let Some(node) = cursor{
        list_len += 1;
        cursor = &node.next;
    };
    let parts = list_len / k ;

    // create the vector to store the res
    let mut res = Vec::<Option<Box<ListNode>>>::with_capacity(k as usize);
    let mut cursor = head;
    
    for i in 0..k{
        res.push(cursor.take());
        let mut node = &mut res[i as usize];
        for _j in 0..parts + if list_len%k > i as i32 {1} else {0}{
            if let Some(n) = node{
                node = &mut n.next;
            }
        }
        cursor = node.take();
    }
    res
}

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