struct Solution;

/*
    输入：prices = [1,2,2], money = 3
    输出：0
    解释：分别购买价格为 1 和 2 的巧克力。你剩下 3 - 3 = 0 块钱。所以我们返回 0 。
*/

impl Solution {
    pub fn buy_choco(mut prices: Vec<i32>, money: i32) -> i32 {
        prices.sort();

        let mut ans = money - prices[0];
        ans = ans - prices[1];

        if ans < 0 {
            return money;
        }

        ans
    }
}

fn main() {
    //示例1
    let prices: Vec<i32> = vec![1, 2, 2];
    println!("{}", Solution::buy_choco(prices, 3));
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test() {
        assert_eq!(0, Solution::buy_choco(vec![1, 2, 2], 3));
        assert_eq!(3, Solution::buy_choco(vec![3, 2, 3], 3));
    }
}
