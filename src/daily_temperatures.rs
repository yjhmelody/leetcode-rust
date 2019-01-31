#![allow(dead_code)]

pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<usize> = vec![];
    let mut res = vec![];
    res.resize(t.len(), 0);

    let mut count = 0;
    for i in 0..t.len() {
        while !stack.is_empty() && t[stack[stack.len() - 1]] < t[i] {
            let cur = stack.pop().unwrap();
            let days = count - cur;
            res[cur] = days as i32;
        }
        stack.push(count);
        count += 1;
    }
    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let t = vec![73, 74, 75, 71, 69, 72, 76, 73];
        assert_eq!(daily_temperatures(t), vec![1, 1, 4, 2, 1, 1, 0, 0]);
    }
}