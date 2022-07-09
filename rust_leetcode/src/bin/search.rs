struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = (left + right) / 2;

            if nums[mid] < target {
                left = mid + 1;
            } else if nums[mid] > target {
                right = mid;
            } else {
                return mid as i32;
            }
        }
        -1
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use crate::Solution;

        //示例1
        let v1 = vec![-1, 0, 3, 5, 9, 12];
        assert_eq!(4, Solution::search(v1, 9));

        //示例2
        let v1 = vec![-1, 0, 3, 5, 9, 12];
        assert_eq!(-1, Solution::search(v1, 2));
    }
}
