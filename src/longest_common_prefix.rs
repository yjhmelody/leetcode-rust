#![allow(dead_code)]

// todo: there are some other solutions
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut common = String::new();
    if strs.len() == 0 {
        return common;
    } else if strs.len() == 1 {
        return strs[0].clone();
    }

    let mut k: usize = 0;
    loop {
        let cur_ch: char;
        match strs[0].chars().nth(k) {
            None => {
                return common;
            }
            Some(ch) => {
                cur_ch = ch;
            }
        };

        for i in 1..strs.len() {
            match strs[i].chars().nth(k) {
                Some(ch) if cur_ch == ch => {}
                _ => return common,
            }
        }

        common.push(cur_ch);
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let strs = vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight"),
        ];

        assert_eq!(longest_common_prefix(strs), String::from("fl"));
        assert_eq!(longest_common_prefix(vec![]), String::from(""));
        assert_eq!(longest_common_prefix(vec![String::from("flower")]), String::from("flower"));
    }
}
