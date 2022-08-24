struct Solution;

impl Solution {
    pub fn can_be_equal(mut target: Vec<i32>, mut arr: Vec<i32>) -> bool {
        target.sort();
        arr.sort();

        for (i, v) in target.iter().enumerate() {
            if *v != arr[i] {
                return false;
            }
        }
        true
    }
}
fn main() {
    println!(
        "{}",
        Solution::can_be_equal(vec![1, 2, 3, 4], vec![2, 4, 1, 3])
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test() {
        assert_eq!(
            true,
            Solution::can_be_equal(vec![1, 2, 3, 4], vec![2, 4, 1, 3])
        );
        assert_eq!(true, Solution::can_be_equal(vec![7], vec![7]));
        assert_eq!(false, Solution::can_be_equal(vec![3, 7, 9], vec![3, 7, 11]));
    }
}
