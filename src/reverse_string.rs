#![allow(dead_code)]

// The problem has a bug in Rust
pub fn reverse_string(s: &mut Vec<char>) {
    if s.len() < 2 {
        return;
    }
    let mut i = 0;
    let mut j = s.len() - 1;
    while i < j {
        s.swap(i, j);
        i += 1;
        j -= 1;
    }
}

pub fn reverse_string2(s: &mut Vec<char>) {
    s.reverse();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        reverse_string(&mut s);
        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
    }

    #[test]
    fn test2() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        reverse_string2(&mut s);
        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
    }
}
