#![allow(dead_code)]

// equation: V_max(i,j) = max{V_max(i', j-1), V(i, j)} which i <= i' < j
// V_max(i,j) means the max area from i to j.
// bad O(n^2)!
pub fn max_area(height: Vec<i32>) -> i32 {
    type StartAndArea = (i32, i32);
    // key: end; value: start, max_area
    let mut dp: Vec<StartAndArea> = Vec::with_capacity(height.len());
    dp.push((0, 0));
    for j in 1..height.len() {
        let mut max_area = 0;
        let mut start = 0;
        for i in 0..j {
            let area = (j - i) as i32 * i32::min(height[i], height[j]);
            if area > max_area {
                start = i;
                max_area = area;
            }
        }
        max_area = i32::max(max_area, dp[j - 1].1);
        dp.push((start as i32, max_area));
    }
    dp[dp.len() - 1].1
}

// O(n): Double pointers
pub fn max_area2(height: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let mut left = 0;
    let mut right = height.len() - 1;
    while left < right {
        let area = (right - left) as i32 * i32::min(height[left], height[right]);
        max_area = i32::max(max_area, area);
        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(max_area(vec![1, 2]), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(max_area2(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(max_area2(vec![1, 2]), 1);
    }
}
