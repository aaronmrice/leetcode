struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s: Vec<_> = s.chars().collect();
        let len = s.len();
        let mut best: String = String::new();
        // i is the index of the first char
        for i in 0..len {
            // j is the length of the substring
            for j in (1..=(len - i)).rev() {
                if j < best.len() {
                    break;
                }
                let substring: &[char] = &s[i..i + j];
                if is_palindrome(substring) {
                    use std::iter::FromIterator;
                    best = String::from_iter(substring.into_iter());
                }
            }
        }

        best
    }
}

fn is_palindrome(inp: &[char]) -> bool {
    let len = inp.len();
    if len == 0 || len == 1 {
        true
    } else if inp[0] == inp[len - 1] {
        is_palindrome(&inp[1..len - 1])
    } else {
        false
    }
}
fn main() {
    // dbg!(Solution::longest_palindrome("babad".into()));
    dbg!(Solution::longest_palindrome("cbbd".into()));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_babad() {
        let substring = Solution::longest_palindrome("babad".into());
        assert!(substring == "bab" || substring == "aba");
    }
    #[test]
    fn test_babad() {
        assert!(Solution::longest_palindrome("cbbd".into()) == "bb");
    }
}
