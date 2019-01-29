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

    println!("{:?}", colors);
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
}