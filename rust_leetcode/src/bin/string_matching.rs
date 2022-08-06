struct Solution;

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        use std::collections::HashSet;
        let mut ans = HashSet::new();
        for (i, w) in words.iter().enumerate() {
            for j in words.iter().skip(i + 1) {
                if w.contains(j) {
                    ans.insert(j.clone());
                } else if j.contains(w) {
                    ans.insert(w.clone());
                }
            }
        }
        ans.into_iter().collect::<Vec<String>>()
    }
}

fn main() {
    //示例1
    let words = vec![
        "mass".to_string(),
        "as".to_string(),
        "hero".to_string(),
        "superhero".to_string(),
    ];
    println!("{:?}", Solution::string_matching(words)); //["as","hero"]

    //示例2
    let words = vec!["leetcode".to_string(), "et".to_string(), "code".to_string()];
    println!("{:?}", Solution::string_matching(words)); //["et","code"]

    //示例3
    let words = vec!["blue".to_string(), "green".to_string(), "bu".to_string()];
    println!("{:?}", Solution::string_matching(words)); //[]
}
