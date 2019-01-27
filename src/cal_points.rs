#![allow(dead_code)]

pub fn cal_points(ops: Vec<String>) -> i32 {
    let mut nums = vec![];
    for s in ops {
        match s.as_str() {
            "+" => { nums.push(nums[nums.len() - 1] + nums[nums.len() - 2]); }
            "D" => { nums.push(nums[nums.len() - 1] * 2); }
            "C" => { nums.pop(); }
            num => { nums.push(num.parse::<i32>().unwrap()); }
        };
    }

    nums.iter().sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ops = vec![
            String::from("5"),
            String::from("-2"),
            String::from("4"),
            String::from("C"),
            String::from("D"),
            String::from("9"),
            String::from("+"),
            String::from("+"),
        ];
        assert_eq!(cal_points(ops), 27);
    }
}