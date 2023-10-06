#![allow(unused)]
use std::collections::BinaryHeap;
use std::cmp::Reverse;
fn main() {
    println!("run -> 'Cargo test'");
}
struct KthLargest{
    k : i32,
    nums : BinaryHeap<Reverse<i32>> // Min_Heap
}

impl KthLargest{
    pub fn new(k : i32, nums : Vec<i32>) -> Self{
        let mut heap =  BinaryHeap::new();
        for ele in nums{
            heap.push(Reverse(ele));
        }

        while heap.len() > k as usize{
            heap.pop();
        }
        
        Self { k, nums: heap }
    } 
    pub fn add(&mut self, val: i32) -> i32{
        &self.nums.push(Reverse(val));
        while self.nums.len() > self.k as usize{
            self.nums.pop();
        }
        self.nums.peek().unwrap().0
    }
}

#[cfg(test)]
mod test{
    use crate::KthLargest;

    
    #[test]
    fn test1(){
        let mut obj = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(obj.add(3), 4);
        assert_eq!(obj.add(5), 5);
        assert_eq!(obj.add(10), 5);
        assert_eq!(obj.add(9), 8);
        assert_eq!(obj.add(4), 8);
    }
}