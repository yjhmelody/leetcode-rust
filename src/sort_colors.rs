#![allow(dead_code)]

pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut colors = [0, 0, 0];
    for num in &mut nums.iter() {
        colors[*num as usize] += 1;
    }

    let mut base = 0;
    for (i, &count) in colors.iter().enumerate() {
        for j in 0..count {
            nums[base + j] = i as i32;
        }
        base += count;
    }
}

// three-way quick sort
pub fn sort_colors2(nums: &mut Vec<i32>) {
    let mut high = nums.len();
    let mut i = 0;
    let mut low = 0;
    while i < high {
        if nums[i] == 0 {
            nums.swap(i, low);
            i += 1;
            low += 1;
        } else if nums[i] == 2 {
            high -= 1;
            nums.swap(i, high);
        } else {
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test2() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        sort_colors2(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }
}
