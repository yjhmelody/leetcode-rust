#![allow(dead_code)]

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    if n == 0 {
        return;
    }
    if m == 0 {
        for (i, num) in nums2.iter().enumerate() {
            nums1[i] = *num;
        }
        return;
    }

    let m = m as usize;
    let n = n as usize;
    // move n position distances
    for i in (0..m).rev() {
        nums1[i + n] = nums1[i];
    }
    let mut count = 0;
    let mut i = n;
    let mut j = 0;

    while i != m + n && j != n {
        if nums1[i] < nums2[j] {
            nums1[count] = nums1[i];
            i += 1;
        } else {
            nums1[count] = nums2[j];
            j += 1;
        }
        count += 1;
    }

    while i != m + n {
        nums1[count] = nums1[i];
        i += 1;
        count += 1;
    }

    while j != n {
        nums1[count] = nums2[j];
        j += 1;
        count += 1;
    }
}

// insert larger number to tail
pub fn merge2(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    if n == 0 {
        return;
    }
    if m == 0 {
        for (i, num) in nums2.iter().enumerate() {
            nums1[i] = *num;
        }
        return;
    }
    let mut k = m + n - 1;
    let mut i = m - 1;
    let mut j = n - 1;
    // guard
    while k >= 0 {
        if (j < 0 && i >= 0) || (i >= 0 && nums1[i as usize] > nums2[j as usize]) {
            nums1[k as usize] = nums1[i as usize];
            i -= 1;
            k -= 1;
        } else if (i < 0 && j >= 0) || (j >= 0 && nums1[i as usize] <= nums2[j as usize]) {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
            k -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        let mut nums3 = vec![1, 2, 2, 3, 5, 6];
        merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, nums3);

        let mut nums1 = vec![1, 2, 4, 5, 6, 0];
        let mut nums2 = vec![3];
        let mut nums3 = vec![1, 2, 3, 4, 5, 6];
        merge(&mut nums1, 5, &mut nums2, 1);
        assert_eq!(nums1, nums3);
    }

    #[test]
    fn test2() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        let mut nums3 = vec![1, 2, 2, 3, 5, 6];
        merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, nums3);

        let mut nums1 = vec![1, 2, 4, 5, 6, 0];
        let mut nums2 = vec![3];
        let mut nums3 = vec![1, 2, 3, 4, 5, 6];
        merge2(&mut nums1, 5, &mut nums2, 1);
        assert_eq!(nums1, nums3);
    }
}
