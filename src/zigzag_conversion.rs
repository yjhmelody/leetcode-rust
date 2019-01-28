#![allow(dead_code)]

pub fn convert(s: String, num_rows: i32) -> String {
    use std::iter;
    let num_rows = num_rows as usize;

    if num_rows == 1 {
        return s;
    }

    let mut mat: Vec<Vec<char>> = Vec::with_capacity(num_rows as usize);
    for _ in 0..num_rows {
        mat.push(iter::repeat('\0').take(s.len()).collect::<Vec<char>>());
    }


    let mut step = false;
    let mut j = 0;
    let mut char_i = 0;
    for i in (0..num_rows).chain((1..num_rows - 1).rev()).cycle() {
        match s.chars().nth(char_i) {
            Some(c) => { mat[i][j] = c }
            None => { break; }
        };
        if i == num_rows - 1 {
            step = true;
        }
        if i == 0 {
            step = false;
        }

        if step {
            j += 1;
        }

        char_i += 1;
    }

    let mut ret = vec![];
    for line in mat {
        for ch in line {
            if ch != '\0' {
                ret.push(ch);
            }
        }
    }

    ret.iter().collect::<String>()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(convert(String::from("LEETCODEISHIRING"), 3), String::from("LCIRETOESIIGEDHN"));
        assert_eq!(convert(String::from("LEETCODEISHIRING"), 4), String::from("LDREOEIIECIHNTSG"));
    }
}