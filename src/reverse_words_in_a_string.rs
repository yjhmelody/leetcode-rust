#![allow(dead_code)]

pub fn reverse_words(s: String) -> String {
    let words: Vec<&str> = s.split(' ').collect();
    let s = words
        .into_iter()
        .rev()
        .fold(String::new(), |acc, x| match x.chars().nth(0) {
            Some(x) if x.is_whitespace() => acc,
            None => acc,
            _ => acc + " " + x,
        });
    s.trim().to_string()
}

pub fn reverse_words2(s: String) -> String {
    let s = s
        .split_whitespace()
        .into_iter()
        .rev()
        .collect::<Vec<&str>>();
    s.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            reverse_words("the sky is blue".to_string()),
            "blue is sky the".to_string()
        );

        assert_eq!(
            reverse_words("  hello world!  ".to_string()),
            "world! hello".to_string()
        );

        assert_eq!(
            reverse_words("a good   example".to_string()),
            "example good a".to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            reverse_words2("the sky is blue".to_string()),
            "blue is sky the".to_string()
        );

        assert_eq!(
            reverse_words2("  hello world!  ".to_string()),
            "world! hello".to_string()
        );

        assert_eq!(
            reverse_words2("a good   example".to_string()),
            "example good a".to_string()
        );
    }
}
