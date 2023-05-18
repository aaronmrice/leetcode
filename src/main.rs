struct Solution;
impl Solution {
    pub fn is_match(input: String, pattern: String) -> bool {
        dbg!(&pattern);
        let pattern = parse_pattern(&pattern[..]);
        dbg!(pattern);
        unimplemented!();
    }
}

#[derive(Debug)]
enum RegexpToken {
    Literal(char),
    Wildcard,
    Star(char),
    WildcardStar,
}
fn parse_pattern(pattern: &str) -> Vec<RegexpToken> {
    let mut ret: Vec<RegexpToken> = vec![];
    let chars: Vec<char> = pattern.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let current: char = chars[i];
        let current_is_wildcard: bool = current == '.';
        let is_star: bool = i < chars.len() - 1 && chars[i + 1] == '*';
        ret.push(match (current_is_wildcard, is_star) {
            (true, true) => RegexpToken::WildcardStar,
            (true, false) => RegexpToken::Wildcard,
            (false, true) => RegexpToken::Star(current),
            (false, false) => RegexpToken::Literal(current),
        });
        if is_star {
            i += 2;
        } else {
            i += 1;
        }
    }

    ret
}
fn main() {
    assert_eq!(false, Solution::is_match("aa".into(), "a".into()));
    assert_eq!(false, Solution::is_match("aa".into(), "a*".into()));
    assert_eq!(false, Solution::is_match("ab".into(), ".*".into()));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_aa_a() {
        assert_eq!(false, Solution::is_match("aa".into(), "a".into()));
    }

    #[test]
    fn test_aa_astart() {
        assert_eq!(false, Solution::is_match("aa".into(), "a*".into()));
    }

    #[test]
    fn test_ab_dotstar() {
        assert_eq!(false, Solution::is_match("ab".into(), ".*".into()));
    }

    #[test]
    fn test_aabb_cstarastarbstar() {
        assert_eq!(false, Solution::is_match("aabb".into(), "c*a*b*".into()));
    }
}
