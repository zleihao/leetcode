struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim()
            .to_string()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .len() as i32
    }
}

fn main() {
    let s = "Hello World".to_string();
    println!("{}", Solution::length_of_last_word(s));
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(
            Solution::length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
        assert_eq!(
            Solution::length_of_last_word("luffy is still joyboy".to_string()),
            6
        );
    }
}
