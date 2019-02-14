#![allow(dead_code)]

// dp
pub fn longest_palindrome(s: String) -> String {
    use std::iter;
    let s = s.into_bytes();
    if s.len() <= 1 {
        return String::from_utf8(s).unwrap();
    }
    let mut dp: Vec<Vec<bool>> = Vec::with_capacity(s.len());
    for _ in 0..s.len() {
        dp.push(iter::repeat(true).take(s.len()).collect())
    }
    let mut final_i = 0;
    let mut final_j = 0;
    let mut longest = 0;
    dp[s.len() - 1][s.len() - 1] = true;
    for i in 0..s.len() - 1 {
        dp[i][i] = true;
        let j = i + 1;
        dp[i][j] = s[i] == s[j];
        if dp[i][j] && longest < j - i + 1 {
            longest = j - i + 1;
            final_i = i;
            final_j = j;
        }
    }
    for i in (0..s.len()).rev() {
        for j in i + 1..s.len() {
            dp[i][j] = dp[i + 1][j - 1] && s[i] == s[j];
            if dp[i][j] && longest <= j - i + 1 {
                longest = j - i + 1;
                final_i = i;
                final_j = j;
            }
        }
    }


    let temp = String::from_utf8(s).unwrap();
    String::from(&temp[final_i..=final_j])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            longest_palindrome(String::from("babad")),
            String::from("bab")
        );
        assert_eq!(longest_palindrome(String::from("")), String::from(""));
        assert_eq!(longest_palindrome(String::from("cbbd")), String::from("bb"));
        assert_eq!(longest_palindrome(String::from("abcda")), String::from("a"));
        assert_eq!(longest_palindrome(String::from("ccc")), String::from("ccc"));
    }
}
