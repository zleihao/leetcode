struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        let mut ans_vec = Vec::new();
        //对于每一个 nums[i]，我们首先查询哈希表中是否存在 target - nums[i],若存在返回其索引，
        //反之将此数 target - nums[i] 放到哈希表中
        for (i, num) in nums.into_iter().enumerate() {
            let n = target - num;
            if let Some(val) = map.get(&n) {
                ans_vec.push(*val);
                ans_vec.push(i as i32);
                return ans_vec;
            } else {
                map.insert(num, i as i32);
            }
        }
        ans_vec
    }
}

fn main() {
    //示例1
    let v1 = vec![2, 7, 11, 15];
    let v2 = Solution::two_sum(v1, 9);
    println!("{:?}", v2);

    //示例2
    let v1 = vec![3, 2, 4];
    let v2 = Solution::two_sum(v1, 6);
    println!("{:?}", v2);

    //示例3
    let v1 = vec![3, 3, 3];
    let v2 = Solution::two_sum(v1, 6);
    println!("{:?}", v2);
}
