pub fn main() {}
struct Solution {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum RegexToken {
    Literal(char),
    Wildcard,
    Star(char),
    WildcardStar,
}
type Regex = Vec<RegexToken>;

fn parse_regex(pattern: &str) -> Regex {
    let mut ret: Vec<RegexToken> = vec![];
    let chars: Vec<char> = pattern.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let current: char = chars[i];
        let current_is_wildcard: bool = current == '.';
        let is_star: bool = i < chars.len() - 1 && chars[i + 1] == '*';
        ret.push(match (current_is_wildcard, is_star) {
            (true, true) => RegexToken::WildcardStar,
            (true, false) => RegexToken::Wildcard,
            (false, true) => RegexToken::Star(current),
            (false, false) => RegexToken::Literal(current),
        });
        if is_star {
            i += 2;
        } else {
            i += 1;
        }
    }

    ret
}

impl Solution {
    pub fn is_match(input_text: String, regex_pattern: String) -> bool {
        let regex: Regex = parse_regex(regex_pattern.as_str());
        let chars: Vec<_> = input_text.chars().collect();
        let mut memo: Vec<Option<bool>> = vec![None; (input_text.len() + 1) * (regex.len() + 1)];
        let result = match_regex(&regex, &chars[..], 0, 0, &mut memo);

        result
    }
}

fn match_regex(
    regex: &Regex,
    input: &[char],
    regex_idx: usize,
    input_idx: usize,
    memo: &mut Vec<Option<bool>>,
) -> bool {
    assert!(regex_idx <= regex.len() + 1);
    assert!(input_idx <= input.len() + 1);
    let memo_cell_idx = regex_idx * input.len() + input_idx;
    assert!(memo_cell_idx < memo.len());

    dbg!(regex_idx, input_idx, regex.len(), input.len());

    {
        let memo_cell: Option<bool> = memo[memo_cell_idx];
        if memo_cell.is_some() {
            return memo_cell.unwrap();
        }
    }
    let result: bool = if regex_idx == regex.len() {
        // if we're one-past-the-end of regex tokens, we better be one-past-the-end on the input text
        input.len() == input_idx
    } else {
        let regex_token = regex[regex_idx];

        use RegexToken::*;
        match regex_token {
            Literal(pattern) => {
                input.len() > input_idx
                    && input[input_idx] == pattern
                    && match_regex(regex, input, regex_idx + 1, input_idx + 1, memo)
            }
            Wildcard => {
                input.len() > input_idx
                    && match_regex(regex, input, regex_idx + 1, input_idx + 1, memo)
            }
            Star(pattern) => {
                // if we can match current regex token as zero width, then yes it matches
                match_regex(regex, input, regex_idx + 1, input_idx, memo)
                    || (
                        // if the pattern doesn't match this char in the text, we can't match
                        input.len() > input_idx
                            && input[input_idx] == pattern
                            && (
                                // we can either match more input with the same token
                                match_regex(regex,input,regex_idx, input_idx + 1, memo)
                    ||
                        // or we can match the last char for the token, advance to next regex token and next char of the text
                        match_regex(regex,input,regex_idx + 1, input_idx + 1, memo)
                            )
                    )
            }
            WildcardStar => {
                // we can match current regex token as zero width
                match_regex(regex, input, regex_idx + 1, input_idx, memo)
                    || (input.len() > input_idx
                        && (
                            // we can either match more input with the same token
                            match_regex(regex,input,regex_idx, input_idx + 1, memo)
                        ||
                        // we can match the last char for the token, advance to next regex token and next char of the text
                        match_regex(regex,input,regex_idx + 1, input_idx + 1, memo)
                        ))
            }
        }
    };

    // write the result to memory
    memo[memo_cell_idx] = Some(result);

    // return the result up the stack
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_aa_a_() {
        assert_eq!(true, Solution::is_match("a".into(), "a".into()));

        assert_eq!(false, Solution::is_match("aa".into(), "a".into()));
        assert_eq!(false, Solution::is_match("b".into(), "a".into()));
    }

    #[test]
    fn test_aa_astar() {
        assert_eq!(true, Solution::is_match("a".into(), "a*".into()));
        assert_eq!(true, Solution::is_match("aa".into(), "a*".into()));
        assert_eq!(true, Solution::is_match("aaaaaaaaaa".into(), "a*".into()));
        assert!(Solution::is_match("".into(), "a*".into()));

        assert_eq!(false, Solution::is_match("aaaaaab".into(), "a*".into()));
        assert_eq!(false, Solution::is_match("b".into(), "a*".into()));
    }

    #[test]
    fn test_ab_dotstarc() {
        assert_eq!(false, Solution::is_match("ab".into(), ".*c".into()));
    }
    #[test]
    fn test_ab_dotstar() {
        assert_eq!(true, Solution::is_match("ab".into(), ".*".into()));
        assert_eq!(true, Solution::is_match("aaaaaabb".into(), ".*".into()));
        assert_eq!(
            true,
            Solution::is_match("abbbbbbbbbbbbbbb".into(), ".*".into())
        );
    }

    #[test]
    fn test_aabb_cstarastarbstar() {
        assert_eq!(true, Solution::is_match("aabb".into(), "c*a*b*".into()));
    }

    #[test]
    fn leetcode_testcase_25_a_abstara() {
        assert!(!Solution::is_match("a".into(), "ab*a".into()))
    }

    #[test]
    fn leetcode_testcase_28_a_dotstardotdotastar() {
        assert!(!Solution::is_match("a".into(), ".*..a*".into()));
    }
}
