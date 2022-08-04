struct Solution;

//先排序，遍历求出数组总和(sum)
//然后倒序求和，当大于sum/2后
//返回该元素到最后所有元素即可
impl Solution {
    pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut ans = Vec::new();
        let mut s = 0;

        nums.sort();
        let sum = nums.iter().sum::<i32>();

        for i in (0..nums.len()).rev() {
            s += nums[i];
            ans.push(nums[i]);
            if s > sum / 2 {
                break;
            }
        }

        ans
    }
}

fn main() {
    //示例1
    let nums = vec![4, 3, 10, 9, 8];
    println!("{:?}", Solution::min_subsequence(nums));

    //示例2
    let nums = vec![4, 4, 7, 6, 7];
    println!("{:?}", Solution::min_subsequence(nums));

    //示例3
    let nums = vec![6];
    println!("{:?}", Solution::min_subsequence(nums));
}
