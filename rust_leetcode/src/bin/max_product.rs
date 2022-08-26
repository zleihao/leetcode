struct Solution;
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);
        let mut ans = 0;

        while left != right {
            let tmp = (nums[left] - 1) * (nums[right] - 1);
            if tmp > ans {
                ans = tmp;
            }
            if nums[left] > nums[right] {
                right -= 1;
            } else {
                left += 1;
            }
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::max_product(vec![3, 4, 5, 2]));
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test() {
        assert_eq!(Solution::max_product(vec![3, 4, 5, 2]), 12);
        assert_eq!(Solution::max_product(vec![1, 5, 4, 5]), 16);
        assert_eq!(Solution::max_product(vec![3, 7]), 12);
    }
}
