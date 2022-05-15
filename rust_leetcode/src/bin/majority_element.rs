struct Solution;

impl Solution {
    //摩尔投票法
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut ans = 0;
        for val in nums.into_iter() {
            if count == 0 {
                ans = val;
            }
            if ans == val {
                count += 1;
            } else {
                count -= 1;
            }
        }
        ans
    }
}

fn main() {
    //示例1
    let v1 = vec![3, 2, 3];
    println!("{}", Solution::majority_element(v1));
    //示例1
    let v2 = vec![2, 2, 1, 1, 1, 2, 2];
    println!("{}", Solution::majority_element(v2));
}

#[cfg(test)]
mod tests {

    #[test]
    fn majority_element() {
        let v1 = vec![3, 2, 3];
        assert_eq!(3, Solution::majority_element(v1));

        let v1 = vec![2, 2, 1, 1, 1, 2, 2];
        assert_eq!(2, Solution::majority_element(v1));
    }
}
