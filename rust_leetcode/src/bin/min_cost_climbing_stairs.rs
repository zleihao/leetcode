struct Solution;
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        //定义一个数组opt，来记录爬楼梯所支付额，数组cost的最后一个值为楼梯的最后一个台阶
        let mut opt = Vec::with_capacity(cost.len() + 1);

        for _ in 0..cost.len() + 1 {
            opt.push(0);
        }
        //可以从第1个台阶或者第二的台阶开始爬。所以记录当前使用台阶数为0
        opt[0] = 0;
        opt[1] = 0;
        //此时开始正式往上爬
        for i in 2..cost.len() + 1 {
            //爬一个台阶支持额
            let a = opt[i - 1] + cost[i - 1];
            //爬两个台阶支付额
            let b = opt[i - 2] + cost[i - 2];
            //比较爬一个台阶支付的少还是两个台阶少，记录到opt数组中
            opt[i] = a.min(b);
        }
        //cost.len表示此时已经爬到楼梯顶部，并返回总支付额
        opt[cost.len()]
    }
}

fn main() {
    println!("{}", Solution::min_cost_climbing_stairs(vec![10, 15, 20]));
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }
}
