struct Solution;
impl Solution {
    //方法1
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        (0..start_time.len())
            .filter(|&i| start_time[i] <= query_time && end_time[i] >= query_time)
            .count() as i32
    }
    //方法2
    // pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
    //     start_time
    //         .iter()
    //         .zip(end_time.iter())
    //         .into_iter()
    //         .fold(0, |mut acc, v| {
    //             if *v.1 >= query_time && *v.0 <= query_time {
    //                 acc += 1;
    //             }
    //             acc
    //         })
    // }
}

fn main() {
    //示例1
    let startTime = vec![1, 2, 3];
    let endTime = vec![3, 2, 7];
    let queryTime = 4;
    println!("{}", Solution::busy_student(startTime, endTime, queryTime));
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test() {
        assert_eq!(Solution::busy_student(vec![1, 2, 3], vec![3, 2, 7], 4), 1);
        assert_eq!(Solution::busy_student(vec![4], vec![4], 4), 1);
        assert_eq!(Solution::busy_student(vec![4], vec![4], 5), 0);
        assert_eq!(
            Solution::busy_student(vec![1, 1, 1, 1], vec![1, 3, 2, 4], 7),
            0
        );
        assert_eq!(
            Solution::busy_student(
                vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
                vec![10, 10, 10, 10, 10, 10, 10, 10, 10],
                5
            ),
            5
        );
    }
}
