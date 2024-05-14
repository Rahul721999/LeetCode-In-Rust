fn main() {
    assert_eq!(Solution::climb_stairs(5), 8);
}

pub struct Solution;
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut a, mut b) = (1,1);

        for _i in 0..n-1{
            let c = a;
            a += b;
            b = c;
        }
        a
    }
}

#[cfg(test)]
mod test{
    use crate::Solution;
    #[test]
    fn test_with_1(){
        assert_eq!(Solution::climb_stairs(1), 1);
    }

    #[test]
    fn test_with_3(){
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}