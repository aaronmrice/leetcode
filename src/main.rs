/*
In order to apply backtracking to a specific class of problems,
one must provide the data P for the particular instance of the problem that is to be solved,
and six procedural parameters, root, reject, accept, first, next, and output.
These procedures should take the instance data P as a parameter and should do the following:

root(P): return the partial candidate at the root of the search tree.
reject(P,c): return true only if the partial candidate c is not worth completing.
accept(P,c): return true if c is a solution of P, and false otherwise.
first(P,c): generate the first extension of candidate c.
next(P,s): generate the next alternative extension of a candidate, after the extension s.
output(P,c): use the solution c of P, as appropriate to the application.

The backtracking algorithm reduces the problem to the call backtrack(root(P)),
where backtrack is the following recursive procedure:
procedure backtrack(P, c) is
    if reject(P, c) then return
    if accept(P, c) then output(P, c)
    s ← first(P, c)
    while s ≠ NULL do
        backtrack(P, s)
        s ← next(P, s)
*/

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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ParseNode {
    Literal(char),
    Wildcard,
    Star(char, u32),   // length
    WildcardStar(u32), // length
}

type PartialSolution = Vec<ParseNode>;

fn length(inp: &PartialSolution) -> u32 {
    use ParseNode::*;
    inp.iter()
        .map(|x| match x {
            Literal(_) => 1,
            Wildcard => 1,
            Star(_, len) => *len,
            WildcardStar(len) => *len,
        })
        .sum()
}
/// return the partial candidate at the root of the search tree.
/// for us, an empty list, having matched nothing
fn root() -> PartialSolution {
    Vec::new()
}

/// returns true if the partial solution matches the input
fn partial_solution_matches(inp: &str, partial: &PartialSolution) -> bool {
    let inp_len: usize = inp.chars().count();
    let mut len_currently_matched: usize = 0;
    for node in partial {
        match node {
            ParseNode::Wildcard => {
                if len_currently_matched + 1 > inp_len {
                    return false;
                }
                len_currently_matched += 1;
            }
            ParseNode::Literal(char) => {
                if len_currently_matched >= inp_len {
                    return false;
                }
                if inp.chars().nth(len_currently_matched).unwrap() != *char {
                    return false;
                }
                len_currently_matched += 1;
            }
            ParseNode::Star(char, len) => {
                for i in len_currently_matched..len_currently_matched + *len as usize {
                    if let Some(inp_char) = inp.chars().nth(i) {
                        if inp_char != *char {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                len_currently_matched += *len as usize;
            }
            ParseNode::WildcardStar(len) => {
                if inp_len < len_currently_matched + *len as usize {
                    return false;
                }
                len_currently_matched += *len as usize;
            }
        }
    }
    true
}

/// return true if `partial` is a complete solution, and false otherwise.
fn accept(inp: &str, regex: &Regex, partial: &PartialSolution) -> bool {
    regex.len() == partial.len()
        && length(partial) == inp.len() as u32
        && partial_solution_matches(inp, partial)
}
/// return true only if the partial candidate c is not worth completing.
fn reject(inp: &str, partial: &PartialSolution) -> bool {
    !partial_solution_matches(inp, partial)
}

/// generate the first extension of candidate c.
fn first(inp: &str, regex: &Regex, partial: &PartialSolution) -> Option<PartialSolution> {
    let len_currently_matched = length(partial);
    let next_regex_token = regex.get(partial.len());
    if let Some(token) = next_regex_token {
        use RegexToken::*;
        let parse_node = match *token {
            Literal(char) => ParseNode::Literal(char),
            Wildcard => ParseNode::Wildcard,
            Star(char) => ParseNode::Star(char, inp.len() as u32 - len_currently_matched),
            WildcardStar => ParseNode::WildcardStar(inp.len() as u32 - len_currently_matched),
        };

        let mut ret: Vec<ParseNode> = partial.clone();
        ret.push(parse_node);

        Some(ret)
    } else {
        None
    }
}
/// generate the next alternative extension of a candidate, after the extension s.
/// at the same node level, which for us, I think, means just decrementing the star matching length
fn next(inp: &str, regex: &Regex, mut candidate: PartialSolution) -> Option<PartialSolution> {
    let next: Option<ParseNode> = match candidate.last().unwrap() {
        ParseNode::Literal(x) => None,
        ParseNode::Wildcard => None,
        ParseNode::Star(x, len) => {
            if *len == 0 {
                None
            } else {
                Some(ParseNode::Star(*x, *len - 1))
            }
        }
        ParseNode::WildcardStar(len) => {
            if *len == 0 {
                None
            } else {
                Some(ParseNode::WildcardStar(*len - 1))
            }
        }
    };
    match next {
        None => None,
        Some(parse_node) => {
            let final_idx = candidate.len() - 1;
            candidate[final_idx] = parse_node;
            Some(candidate)
        }
    }
}

/* procedure backtrack(P, c) is
    if reject(P, c) then return
    if accept(P, c) then output(P, c)
    s ← first(P, c)
    while s ≠ NULL do
        backtrack(P, s)
        s ← next(P, s)
*/
// this function `backtrack` is given a node in the search graph
// it will check if the node is final (reject, accept) and return if so
// then it will iterate over the subnodes and call `backtrack` on each
// success is bubbled up the call stack
// failure is None
fn backtrack(inp: &str, regex: &Regex, current: PartialSolution) -> Option<PartialSolution> {
    println!(
        "backtrack inp={} regex={:?} currrent={:?}",
        inp, regex, &current
    );
    if dbg!(reject(inp, &current)) {
        return None;
    }
    if dbg!(accept(inp, &regex, &current)) {
        return Some(current);
    }
    let mut candidate: Option<PartialSolution> = first(inp, regex, &current);
    while let Some(c) = candidate {
        dbg!(&c);
        if let Some(x) = backtrack(inp, regex, c.clone()) {
            return Some(x);
        }
        candidate = dbg!(next(inp, regex, c.clone()));
    }
    return None; // I guess?
}

struct Solution;
impl Solution {
    pub fn is_match(input: String, pattern: String) -> bool {
        let regex = parse_regex(&pattern);
        let root = root();
        let result = backtrack(&input, &regex, root.clone());
        return result.is_some();
    }
}

fn main() {
    assert_eq!(false, Solution::is_match("aa".into(), "a".into()));
    assert_eq!(true, Solution::is_match("aa".into(), "a*".into()));
    assert_eq!(true, Solution::is_match("ab".into(), ".*".into()));
    assert_eq!(
        true,
        Solution::is_match("abbababababagf".into(), ".*".into())
    );
    assert_eq!(true, Solution::is_match("aaabb".into(), "c*a*b*".into()));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_aa_a_() {
        assert_eq!(true, Solution::is_match("a".into(), "a".into()));

        // assert_eq!(false, Solution::is_match("aa".into(), "a".into()));
        // assert_eq!(false, Solution::is_match("b".into(), "a".into()));
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

    #[test]
    fn test_first() {
        let inp = "abccbcabcbcbaaa";
        let regex = parse_regex("a*.*a*.*a*.*");
        let expected = Some(vec![ParseNode::Star('a', inp.len() as u32)]);
        assert_eq!(expected, first(inp, &regex, &vec![]));
    }
}
