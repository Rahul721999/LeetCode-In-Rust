fn main() {
    let arr = vec![1,3,4,8];
    let queries = vec![vec![0,1],vec![1,2],vec![0,3],vec![3,3]];
    let res = xor_queries(arr, queries);
    assert_eq!(res, vec![2,7,14,8]);
}
pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    if arr.is_empty() || queries.is_empty(){
        return Vec::new()
    }else{
        let mut res = Vec::<i32>::with_capacity(arr.len());
        let mut iter = queries.into_iter();
        while let Some(vec) = iter.next(){
            let start = vec[0] as usize;
            let end = vec[1] as usize;
            let xor = arr[start..end+1].iter().fold(0, |acc, &x| acc^x);
            res.push(xor);
        };
        res
    }
}