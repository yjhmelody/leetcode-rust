#![allow(dead_code)]

// todo
// let nums1[0..i-1]'s length + nums2[0..j-1]'s length == the rest's length
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let m = nums1.len();
    let n = nums2.len();
    let nums3;
    let nums4;
    if m > n {
        nums3 = nums2;
        nums4 = nums1;
    } else {
        nums3 = nums1;
        nums4 = nums2;
    }
    let mut i_min = 0;
    let mut i_max = m;
    let half = (m + n + 1) / 2;
    while i_min <= i_max {
        let i = (i_min + i_max) / 2;
        let j = half - i;
        if i < i_max && nums4[j - 1] > nums3[i] {
            i_min = i + 1;
        } else if i > i_min && nums3[i - 1] > nums4[j] {
            i_max = i - 1;
        } else {
            let max_left;
            if i == 0 {
                max_left = nums4[j - 1];
            } else if j == 0 {
                max_left = nums3[i - 1];
            } else {
                max_left = nums3[i - 1].max(nums4[j - 1]);
            }
            if (m + n) % 2 == 1 {
                return max_left as f64;
            }

            let min_right;
            if i == m {
                min_right = nums4[j];
            } else if j == n {
                min_right = nums3[i];
            } else {
                min_right = nums3[i].max(nums4[j]);
            }
            return (max_left + min_right) as f64 / 2.0;
        }
    }
    0f64
}


#[cfg(test)]
mod tests {

    #[test]
    fn test1() {
//        assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0f64);
//        assert_eq!(find_median_sorted_arrays(vec![1,2], vec![3,4]), 2.5f64);
//        assert_eq!(find_median_sorted_arrays(vec![], vec![1]), 1f64);
//        assert_eq!(find_median_sorted_arrays(vec![1], vec![1]), 1f64);
    }
}