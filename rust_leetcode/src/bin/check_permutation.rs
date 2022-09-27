struct Solution;

impl Solution {
    pub fn check_permutation(s1: String, s2: String) -> bool {
        let mut t = s1.chars().collect::<Vec<char>>();
        let mut m = s2.chars().collect::<Vec<char>>();

        t.sort();
        m.sort();

        t == m
    }
}

fn main() {
    println!(
        "{}",
        Solution::check_permutation("abc".to_string(), "bca".to_string())
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test() {
        //示例1
        assert_eq!(
            Solution::check_permutation("abc".to_string(), "bca".to_string()),
            true
        );
        //示例2
        assert_eq!(
            Solution::check_permutation("abc".to_string(), "bad".to_string()),
            false
        );
    }
}
