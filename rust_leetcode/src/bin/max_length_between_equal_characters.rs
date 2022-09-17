struct Solution;

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut ans = -1;
        let mut map = [-1; 26];

        for (i, c) in s.bytes().enumerate() {
            if map[(c - b'a') as usize] < 0 {
                map[(c - b'a') as usize] = i as i32;
            } else {
                ans = ans.max(i as i32 - map[(c - b'a') as usize] - 1);
            }
        }

        ans
    }
}

fn main() {
    let s = "scayofdzca".to_string();

    let ans = Solution::max_length_between_equal_characters(s);

    println!("{ans}");
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test() {
        assert_eq!(
            0,
            Solution::max_length_between_equal_characters("aa".to_string())
        );
        assert_eq!(
            2,
            Solution::max_length_between_equal_characters("abca".to_string())
        );
        assert_eq!(
            -1,
            Solution::max_length_between_equal_characters("cbzxy".to_string())
        );
        assert_eq!(
            4,
            Solution::max_length_between_equal_characters("cabbac".to_string())
        );
    }
}
