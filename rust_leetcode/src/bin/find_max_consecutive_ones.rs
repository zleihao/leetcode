struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut ans = 0;

        for i in 1..=nums.len() {
            if nums[i - 1] == 1 {
                count += 1;
                ans = ans.max(count);
            } else {
                count = 0;
            }
        }
        ans
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn find_max_consecutive_ones() {
        //示例1
        assert_eq!(
            3,
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1])
        );
        //示例2
        assert_eq!(
            2,
            Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1])
        );
    }
}
