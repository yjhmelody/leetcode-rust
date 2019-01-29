#![allow(dead_code)]

pub fn int_to_roman(num: i32) -> String {
    let mut res = String::from("");
    let mut num = num;
    while num >= 1000 {
        res.push('M');
        num -= 1000;
    }
    if num >= 900 {
        res.push_str("CM");
        num -= 900;
    }
    if num >= 500 {
        res.push('D');
        num -= 500;
    }
    if num >= 400 {
        res.push_str("CD");
        num -= 400;
    }
    while num >= 100 {
        res.push('C');
        num -= 100;
    }
    if num >= 90 {
        res.push_str("XC");
        num -= 90;
    }
    if num >= 50 {
        res.push('L');
        num -= 50;
    }
    if num >= 40 {
        res.push_str("XL");
        num -= 40;
    }
    while num >= 10 {
        res.push('X');
        num -= 10;
    }
    if num >= 9 {
        res.push_str("IX");
        num -= 90;
    }
    if num >= 5 {
        res.push('V');
        num -= 5;
    }
    if num >= 4 {
        res.push_str("IV");
        num -= 4;
    }
    while num >= 1 {
        res.push('I');
        num -= 1;
    }

    res
}


pub fn int_to_roman2(num: i32) -> String {
    let romans = vec!["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
    let integers = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let mut res = String::from("");
    let mut num = num;
    for (i, &val) in integers.iter().enumerate() {
        while num >= val {
            num -= val;
            res.push_str(romans[i])
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(int_to_roman(1994), String::from("MCMXCIV"));
    }

    #[test]
    fn test2() {
        assert_eq!(int_to_roman2(1994), String::from("MCMXCIV"));
    }
}