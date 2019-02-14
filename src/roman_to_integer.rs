#![allow(dead_code)]

// 16ms
pub fn roman_to_int(s: String) -> i32 {
    use std::collections::HashMap;
    let mut map: HashMap<char, i32> = HashMap::new();
    map.insert('I', 1);
    map.insert('V', 5);
    map.insert('X', 10);
    map.insert('L', 50);
    map.insert('C', 100);
    map.insert('D', 500);
    map.insert('M', 1000);

    let mut sum = 0;
    let mut i = 0;
    loop {
        match s.chars().nth(i) {
            Some('I') => {
                match s.chars().nth(i + 1) {
                    Some('V') => {
                        sum += 4;
                        i += 1
                    }
                    Some('X') => {
                        sum += 9;
                        i += 1
                    }
                    _ => sum += 1,
                };
            }
            Some('X') => {
                match s.chars().nth(i + 1) {
                    Some('L') => {
                        sum += 40;
                        i += 1
                    }
                    Some('C') => {
                        sum += 90;
                        i += 1
                    }
                    _ => sum += 10,
                };
            }
            Some('C') => {
                match s.chars().nth(i + 1) {
                    Some('D') => {
                        sum += 400;
                        i += 1
                    }
                    Some('M') => {
                        sum += 900;
                        i += 1
                    }
                    _ => sum += 100,
                };
            }
            Some(c) => {
                sum += *map.get(&c).unwrap();
            }

            None => {
                break;
            }
        }
        i += 1;
    }

    sum
}

// 20ms
pub fn roman_to_int2(s: String) -> i32 {
    use std::collections::HashMap;
    let mut map: HashMap<u8, i32> = HashMap::new();
    map.insert(b'I', 1);
    map.insert(b'V', 5);
    map.insert(b'X', 10);
    map.insert(b'L', 50);
    map.insert(b'C', 100);
    map.insert(b'D', 500);
    map.insert(b'M', 1000);

    let mut sum = 0;
    let s = s.into_bytes();
    for i in 0..s.len() - 1 {
        let left = map.get(&s[i]).unwrap();
        let right = map.get(&s[i + 1]).unwrap();
        if left < right {
            sum -= left;
        } else {
            sum += left;
        }
    }
    sum += map.get(&s[s.len() - 1]).unwrap();

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(roman_to_int(String::from("LVIII")), 58);
        assert_eq!(roman_to_int(String::from("MCMXCIV")), 1994);
    }

    #[test]
    fn test2() {
        assert_eq!(roman_to_int2(String::from("LVIII")), 58);
        assert_eq!(roman_to_int2(String::from("MCMXCIV")), 1994);
    }
}
