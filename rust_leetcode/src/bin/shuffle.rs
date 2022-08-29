struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        // let size = n as usize;
        // let mut ans = Vec::with_capacity(nums.len());

        // for i in 0..size {
        //     ans.push(nums[i]);
        //     ans.push(nums[i + n as usize]);
        // }
        // ans
        nums.iter()
            .zip(nums.iter().skip(n as usize))
            .flat_map(|(&x, &y)| vec![x, y])
            .collect::<Vec<i32>>()
    }
}

fn main() {
    println!("{:?}", Solution::shuffle(vec![2, 5, 1, 3, 4, 7], 3));
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test() {
        assert_eq!(
            Solution::shuffle(vec![2, 5, 1, 3, 4, 7], 3),
            vec![2, 3, 5, 4, 1, 7]
        );
        assert_eq!(
            Solution::shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4),
            vec![1, 4, 2, 3, 3, 2, 4, 1]
        );
        assert_eq!(Solution::shuffle(vec![1, 1, 2, 2], 2), vec![1, 2, 1, 2]);
    }
}
