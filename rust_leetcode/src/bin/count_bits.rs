// 题目地址: https://leetcode.cn/problems/counting-bits/
struct Solution;

impl Solution {
    // 338. 比特位计数
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut v = Vec::with_capacity(n as usize + 1);
        for _ in 0..=n {
            v.push(1);
        }

        for i in 0..=n {
            let mut count = 0;
            let mut num = i;
            //首先从低位开始与1相与，然后将num右移一位，直到 num == 0
            while num != 0 {
                if num & 0x01 == 1 {
                    count += 1;
                }
                num >>= 1;
            }
            v[i as usize] = count;
        }
        v
    }
}

fn main() {
    //示例1
    println!("{:?}", Solution::count_bits(2));
    //示例2
    println!("{:?}", Solution::count_bits(5));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn count_bit() {
        assert_eq!(Solution::count_bits(2), [0, 1, 1]);
        assert_eq!(Solution::count_bits(5), [0, 1, 1, 2, 1, 2]);
    }
}
