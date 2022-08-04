struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::with_capacity(s.len());

        for c in s.chars() {
            match stack.last() {
                Some(&v) => {
                    if v == '(' && c == ')' {
                        stack.pop();
                    } else if v == '{' && c == '}' {
                        stack.pop();
                    } else if v == '[' && c == ']' {
                        stack.pop();
                    } else {
                        stack.push(c);
                    }
                }
                None => stack.push(c),
            }
        }
        if stack.is_empty() {
            true
        } else {
            false
        }
    }
}

fn main() {
    //示例 1：
    let s = "()";
    assert_eq!(true, Solution::is_valid(s.to_string()));

    // 示例 2：
    let s = "()[]{}";
    assert_eq!(true, Solution::is_valid(s.to_string()));

    // 示例 3：
    let s = "(]";
    assert_eq!(false, Solution::is_valid(s.to_string()));

    // 示例 4：
    let s = "([)]";
    assert_eq!(false, Solution::is_valid(s.to_string()));

    // 示例 5：
    let s = "{[]}";
    assert_eq!(true, Solution::is_valid(s.to_string()));
}
