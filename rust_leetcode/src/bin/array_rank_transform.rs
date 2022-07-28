struct Solution;
impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        let mut nums = arr.clone();
        let mut ans = Vec::new();
        let mut count = 0;

        nums.sort();
        let mut map = HashMap::new();

        for v in nums.iter() {
            if map.get(v).is_some() {
                continue;
            }
            count += 1;
            map.insert(v, count);
        }
        for v in arr.iter() {
            if let Some(&v) = map.get(v) {
                ans.push(v);
            }
        }
        ans
    }
}

fn main() {
    //示例1
    let arr = vec![40, 10, 20, 30];
    println!("{:?}", Solution::array_rank_transform(arr));
    //示例2
    let arr = vec![100, 100, 100];
    println!("{:?}", Solution::array_rank_transform(arr));
    //示例3
    let arr = vec![37, 12, 28, 9, 100, 56, 80, 5, 12];
    println!("{:?}", Solution::array_rank_transform(arr));
}
