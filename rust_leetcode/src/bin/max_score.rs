struct Solution;

impl Solution {
    pub fn max_score(s: String) -> i32 {
        //统计字符串中'1'的个数
        let ones = s.chars().filter(|&c| c == '1').count() as i32;
        let ss = s.as_bytes();
        let mut result = if ss[0] == b'0' { ones + 1 } else { ones - 1 };

        (1..s.len() - 1).fold(result, |acc, i| {
            if ss[i] == b'0' {
                result += 1;
            } else {
                result -= 1;
            }
            acc.max(result)
        })
    }
}

fn main() {
    //示例1
    let s = "011101".to_string();
    println!("{}", Solution::max_score(s));
    //示例2
    let s = "00111".to_string();
    println!("{}", Solution::max_score(s));
    //示例2
    let s = "1111".to_string();
    println!("{}", Solution::max_score(s));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        //示例1
        let s = "011101".to_string();
        assert_eq!(Solution::max_score(s), 5);
        //示例2
        let s = "00111".to_string();
        assert_eq!(Solution::max_score(s), 5);
        //示例2
        let s = "1111".to_string();
        assert_eq!(Solution::max_score(s), 3);
    }
}
