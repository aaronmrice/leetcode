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

pub fn main() {}

struct Solution {}
impl Solution {
    pub fn is_match(input_text: String, regex_pattern: &str) -> bool {
        let regex: Regex = parse_regex(regex_pattern);
        let mut memo: Vec<Option<bool>> = Vec::with_capacity(input_text.len() * regex.len());
        let result = match_regex(&regex, &input_text, 0, 0, &mut memo);

        result
    }
}

// boolean ans;
// if (j == pattern.length()) {
//     ans = i == text.length();
// } else {
//     boolean first_match =
//     (i < text.length() &&
//     (pattern.charAt(j) == text.charAt(i) ||
//     pattern.charAt(j) == '.'));
//
//     if (j + 1 < pattern.length() && pattern.charAt(j + 1) == '*') {
//         ans = (dp(i, j + 2, text, pattern) ||
//         (first_match && dp(i + 1, j, text, pattern)));
//     } else {
//         ans = first_match && dp(i + 1, j + 1, text, pattern);
//     }
// }
// memo[i][j] = ans ? Result.TRUE : Result.FALSE;
// return ans;
fn match_regex(
    regex: &Regex,
    input: &str,
    regex_idx: usize,
    input_idx: usize,
    memo: &mut Vec<Option<bool>>,
) -> bool {
    assert!(regex_idx <= regex.len());
    assert!(input_idx <= input.len());
    let memo_cell_idx = regex_idx * input.len() + input_idx;
    assert!(memo_cell_idx < memo.len());

    println!(
        "match_regex, regex_idx={}, input_idx={},",
        regex_idx, input_idx
    );

    {
        let memo_cell: Option<bool> = memo[memo_cell_idx];
        if memo_cell.is_some() {
            return memo_cell.unwrap();
        }
    }
    let result: bool = if regex_idx == regex.len() {
        input.len() == input_idx
    } else if input_idx >= input.len() {
        false
    } else {
        // guaranteed to have data for both regex and text
        let input_char = input.chars().nth(input_idx).unwrap();
        let regex_token = regex[regex_idx];
        println!("input_char={:?}, regex_token={:?}", input_char, regex_token);

        use RegexToken::*;
        match regex_token {
            Literal(pattern) => {
                input_char == pattern
                    && match_regex(regex, input, regex_idx + 1, input_idx + 1, memo)
            }
            Wildcard => true,
            Star(pattern) => {
                // if we can match current regex token as zero width, then yes it matches
                match_regex(regex, input, regex_idx + 1, input_idx, memo)
                    || (
                        // if the pattern doesn't match this char in the text, we can't match
                        input_char == pattern
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
                        ||
                        // we can either match more input with the same token
                        match_regex(regex,input,regex_idx, input_idx + 1, memo)
                        ||
                        // we can match the last char for the token, advance to next regex token and next char of the text
                        match_regex(regex,input,regex_idx + 1, input_idx + 1, memo)
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
        assert_eq!(true, Solution::is_match("".into(), "a*".into()));

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
}
