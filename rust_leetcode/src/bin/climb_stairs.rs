struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut opt = Vec::with_capacity(n as usize + 1);
        for _ in 0..n + 1 {
            opt.push(0);
        }
        opt[0] = 1;
        opt[1] = 1;

        for i in 2..n + 1 {
            opt[i as usize] = opt[i as usize - 1] + opt[i as usize - 2];
        }

        opt[n as usize]
    }
}

fn main() {
    println!("{}", Solution::climb_stairs(2));
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}
