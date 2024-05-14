fn main() {
    // test
    assert_eq!(Solution::four_sum(vec![0], 0), Vec::<Vec<i32>>::new());

    println!("run -> Cargo test");
}

struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let len = nums.len();

        if len < 4 {
            return vec![];
        }
        use std::collections::HashSet;
        let mut res: HashSet<(i32, i32, i32, i32)> = HashSet::new();
        nums.sort();
        for i in 0..len - 3 {
            // skip for duplicate of i
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            for j in i + 1..len - 2 {
                // skip for duplicate of j
                if j < i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }

                let (mut k, mut l) = (j + 1, len - 1);

                while k < l {
                    //NOTE:- .saturation_add() is necessory to controll integer overflow /
                    match nums[i]
                        .saturating_add(nums[j])
                        .saturating_add(nums[k])
                        .saturating_add(nums[l])
                    {
                        s if s > target => l -= 1,
                        s if s < target => k += 1,
                        _ => {
                            res.insert((nums[i], nums[j], nums[k], nums[l]));

                            // skip for duplicate k
                            while k < l && nums[k] == nums[k + 1] {
                                k += 1;
                            }
                            // skip for duplicate l
                            while k < l && nums[l] == nums[k - 1] {
                                l -= 1;
                            }

                            k += 1;
                            l -= 1;
                        }
                    }
                }
            }
        }
        res.iter().map(|&(a, b, c, d)| vec![a, b, c, d]).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let input = vec![1, 0, -1, 0, -2, 2];
        let target = 0;
        let expected = vec![vec![-2, 0, 0, 2], vec![-1, 0, 0, 1], vec![-2, -1, 1, 2]];
        // test 1
        let res = Solution::four_sum(input, target);
        assert!(res.iter().all(|quad| expected.contains(quad)));
    }
    #[test]
    fn test2() {
        let input = vec![-3, -2, -1, 0, 0, 1, 2, 3];
        let target = 0;
        let expected = vec![
            vec![-3, -2, 2, 3],
            vec![-3, -1, 1, 3],
            vec![-3, 0, 0, 3],
            vec![-3, 0, 1, 2],
            vec![-2, -1, 0, 3],
            vec![-2, -1, 1, 2],
            vec![-2, 0, 0, 2],
            vec![-1, 0, 0, 1],
        ];
        // test 1
        let res = Solution::four_sum(input, target);
        assert!(res.iter().all(|quad| expected.contains(quad)));
    }

    #[test]
    fn test3() {
        let input = vec![1000000000, 1000000000, 1000000000, 1000000000];
        let target = -294967296;
        let expected = Vec::new();
        // test 2
        let res = Solution::four_sum(input, target);
        assert!(res.iter().all(|quad| expected.contains(quad)));
    }
}
