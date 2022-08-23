struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        (0..=nums.len() as i32).sum::<i32>() - nums.iter().sum::<i32>()
    }
}

fn main() {
    let nums = vec![0, 1];

    println!("{}", Solution::missing_number(nums));
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
        assert_eq!(Solution::missing_number(vec![0, 1]), 2);
        assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
        assert_eq!(Solution::missing_number(vec![0]), 1);
    }
}
