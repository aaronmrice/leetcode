struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(left: Vec<i32>, right: Vec<i32>) -> f64 {
        assert!(left.len() + right.len() > 0);
        assert!(left.len() <= 1000);
        assert!(right.len() <= 1000);
        assert!(left.len() + right.len() <= 2_000);

        let must_average_middle_values: bool = (left.len() + right.len()) % 2 == 0;
        let mut left_idx = left.len() / 2;
        let mut right_idx = right.len() / 2;

        if must_average_middle_values {
            // exactly two median values
            if left.len() == 0 {
                (right[right_idx] + right[right_idx - 1]) as f64 / 2.0
            } else if right.len() == 0 {
                (left[left_idx] + left[left_idx - 1]) as f64 / 2.0
            } else if right[right_idx] <= left[left_idx]
                && (right_idx + 1 < right.len() && left[left_idx] <= right[right_idx + 1])
            {
                (left[left_idx] + right[right_idx]) as f64 / 2.0
            } else if left[left_idx] <= right[right_idx]
                && (left_idx + 1 < left.len() && right[right_idx] <= left[left_idx + 1])
            {
                (left[left_idx] + right[right_idx]) as f64 / 2.0
            } else {
                unimplemented!();
            }
        } else {
            // exactly one median value
            if left.len() == 0 {
                right[right_idx] as f64
            } else if right.len() == 0 {
                left[left_idx] as f64
            } else {
                loop {
                    if right[right_idx] <= left[left_idx]
                        && (right_idx + 1 < right.len() && left[left_idx] <= right[right_idx + 1])
                    {
                        return left[left_idx] as f64;
                    } else if left[left_idx] <= right[right_idx]
                        && (left_idx + 1 < left.len() && right[right_idx] <= left[left_idx + 1])
                    {
                        return right[right_idx] as f64;
                    } else {
                        // not at median, move indices
                        unimplemented!()
                        // if right[right_idx] <= left[left_idx] {}
                    }
                }
            }
        }
    }
}

fn main() {
    dbg!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]));
    dbg!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3]));
    dbg!(Solution::find_median_sorted_arrays(
        vec![0, 1, 2],
        vec![0, 1, 2]
    ));
    dbg!(Solution::find_median_sorted_arrays(
        vec![1, 2, 3],
        vec![1, 2, 3]
    ));
    dbg!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]));
    dbg!(Solution::find_median_sorted_arrays(
        vec![1, 2, 3,],
        vec![4, 5]
    ));
    dbg!(Solution::find_median_sorted_arrays(
        vec![1, 2, 3, 4, 5],
        vec![6, 7]
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn test_1() {
        assert_approx_eq!(
            1.0,
            dbg!(Solution::find_median_sorted_arrays(vec![1], vec![]))
        );
        assert_approx_eq!(
            1.0,
            dbg!(Solution::find_median_sorted_arrays(vec![], vec![1]))
        );
    }
    #[test]
    fn test_1_1() {
        assert_approx_eq!(
            1.0,
            dbg!(Solution::find_median_sorted_arrays(vec![1], vec![1]))
        );
        assert_approx_eq!(
            1.0,
            dbg!(Solution::find_median_sorted_arrays(vec![1, 1], vec![]))
        );
        assert_approx_eq!(
            1.0,
            dbg!(Solution::find_median_sorted_arrays(vec![], vec![1, 1]))
        );
    }
    #[test]
    fn test_1_3_2() {
        assert_approx_eq!(
            2.0,
            dbg!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]))
        );
    }
    #[test]
    fn test_1_2_3() {
        assert_approx_eq!(
            2.0,
            dbg!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3]))
        );
    }
    #[test]
    fn test_0_1_2_0_1_2() {
        assert_eq!(
            1.0,
            dbg!(Solution::find_median_sorted_arrays(
                vec![0, 1, 2],
                vec![0, 1, 2]
            ))
        );
    }
    #[test]
    fn test_1_2_3_1_2_3() {
        assert_eq!(
            2.0,
            dbg!(Solution::find_median_sorted_arrays(
                vec![1, 2, 3],
                vec![1, 2, 3]
            ))
        );
    }
    #[test]
    fn test_1_2_3_4() {
        assert_eq!(
            2.5,
            dbg!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]))
        );
    }
    #[test]
    fn test_1_2_3_4_5() {
        assert_eq!(
            3.0,
            dbg!(Solution::find_median_sorted_arrays(
                vec![1, 2, 3,],
                vec![4, 5]
            ))
        );
    }
    #[test]
    fn test_1_2_3_4_5_6_7() {
        assert_eq!(
            4.0,
            dbg!(Solution::find_median_sorted_arrays(
                vec![1, 2, 3, 4, 5],
                vec![6, 7]
            ))
        );
    }
}
