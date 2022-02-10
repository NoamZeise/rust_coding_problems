
fn palindrome(string: &str) -> bool {
    let mut start = 0;
    let mut end = string.len() - 1;
    while start < end {
        if string.as_bytes()[start] != string.as_bytes()[end] {
            return false;
        }
        start += 1;
        end -= 1;
    }
    true
}

pub fn palindromes_from_string(string: &str) -> Vec<String> {
    let mut palindromes: Vec<String> = vec![];

    let mut start = 0;
    while start < string.len() {
        let mut end = string.len();
        while start != end {
            if palindrome(&string[start..end]) {
                palindromes.push(string[start..end].to_string());
                start = end;
            } else {
                end -= 1
            }
        }
    }
    palindromes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palindromes_test_simple() {
        let test_str = "racecarannakayak".to_string();
        assert_eq!(palindromes_from_string(&test_str), ["racecar", "anna", "kayak"]);
    }

    #[test]
    fn palindromes_test_none() {
        let test_str = "abc".to_string();
        assert_eq!(palindromes_from_string(&test_str), ["a", "b", "c"]);
    }

    #[test]
    fn palindromes_test_harder() {
        let test_str = "racecarracecaryoopooygoog".to_string();
        assert_eq!(palindromes_from_string(&test_str), ["racecarracecar", "yoopooy", "goog"]);
    }
}
