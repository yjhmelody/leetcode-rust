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

    // ineffective
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


pub fn convert2(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }
    let num_rows = num_rows as usize;
    let mut rows: Vec<String> = Vec::with_capacity(usize::min(num_rows, s.len()));
    for _ in 0..num_rows {
        rows.push(String::from(""));
    }
    let mut cur_row = 0;
    let mut down = false;
    for c in s.chars() {
        rows[cur_row] += &c.to_string();
        if cur_row == 0 || cur_row == num_rows - 1 {
            down = !down;
        }
        if down {
            cur_row += 1;
        } else {
            cur_row -= 1;
        }
    }

    rows.iter().fold(String::from(""), |acc, s| {
        acc + s
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(convert(String::from("LEETCODEISHIRING"), 3), String::from("LCIRETOESIIGEDHN"));
        assert_eq!(convert(String::from("LEETCODEISHIRING"), 4), String::from("LDREOEIIECIHNTSG"));
    }

    #[test]
    fn test2() {
        assert_eq!(convert2(String::from("LEETCODEISHIRING"), 3), String::from("LCIRETOESIIGEDHN"));
        assert_eq!(convert2(String::from("LEETCODEISHIRING"), 4), String::from("LDREOEIIECIHNTSG"));
    }
}