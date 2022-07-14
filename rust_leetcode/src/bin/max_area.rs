struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_ans = std::i32::MIN;

        if height.is_empty() {
            max_ans = 0;
            return max_ans;
        }

        let (mut left, mut right) = (0, height.len() - 1);

        while left < right {
            //指针之间的距离
            let width = right - left;

            //最大盛水量 = 两个指针指向的数字中较小值 * 指针之间的距离
            let temp = height[left].min(height[right]) * width as i32;
            max_ans = max_ans.max(temp);

            //左边値小，移动左指针
            if height[left] < height[right] {
                left += 1;
            } else {
                //右边値小，移动右指针
                right -= 1;
            }
        }

        max_ans
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        //示例1
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(Solution::max_area(height), 49);
        //示例2
        let height = vec![1, 1];
        assert_eq!(Solution::max_area(height), 1);
    }
}
