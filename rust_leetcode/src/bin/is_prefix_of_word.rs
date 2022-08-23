struct Solution;
impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let mut ans = i32::MAX;
        let mut count = 0;

        for s in sentence.split_ascii_whitespace() {
            count += 1;
            if s.starts_with(&search_word) {
                ans = ans.min(count);
            }
        }
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}

fn main() {
    let sentence = "i am tired".to_string();
    let search_word = "you".to_string();
    println!("{}", Solution::is_prefix_of_word(sentence, search_word));
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test() {
        assert_eq!(
            Solution::is_prefix_of_word("i love eating burger".to_string(), "burg".to_string()),
            4
        );
        assert_eq!(
            Solution::is_prefix_of_word(
                "this problem is an easy problem".to_string(),
                "pro".to_string()
            ),
            2
        );
        assert_eq!(
            Solution::is_prefix_of_word("i am tired".to_string(), "you".to_string()),
            -1
        );
    }
}
