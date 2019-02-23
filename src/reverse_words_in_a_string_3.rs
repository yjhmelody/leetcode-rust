#![allow(dead_code)]

pub fn reverse_words(s: String) -> String {
    let words: Vec<&str> = s.split_whitespace().collect();
    let mut res = String::new();
    for word in words {
        for c in word.chars().rev() {
            res.push(c);
        }
        res.push(' ');
    }

    res.pop();
    res
}

// in-place
pub fn reverse_words2(s: String) -> String {
    let mut words: Vec<Vec<u8>> = s.split_whitespace().map(|s| s.into()).collect();
    for word in &mut words {
        let mut i: i32 = 0;
        let mut j: i32 = word.len() as i32 - 1;
        while i <= j {
            word.swap(i as usize, j as usize);
            i += 1;
            j -= 1;
        }
    }
    words
        .into_iter()
        .map(|s| unsafe { String::from_utf8_unchecked(s) })
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            reverse_words("Let's take LeetCode contest".to_string()),
            "s'teL ekat edoCteeL tsetnoc".to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            reverse_words2("Let's take LeetCode contest".to_string()),
            "s'teL ekat edoCteeL tsetnoc".to_string()
        );

        assert_eq!(
            reverse_words2("I love u".to_string()),
            "I evol u".to_string()
        );
    }
}
