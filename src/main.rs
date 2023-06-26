struct Solution;
impl Solution {
    pub fn is_match(input: String, pattern: String) -> bool {
        dbg!(&pattern);
        let pattern = parse_pattern(&pattern[..]);
        dbg!(&pattern);

        dbg!(string_matches_regex(&input, &pattern))
    }
}

#[derive(Debug)]
enum RegexpToken {
    Literal(char),
    Wildcard,
    Star(char),
    WildcardStar,
}

type Regexp = Vec<RegexpToken>;

fn parse_pattern(pattern: &str) -> Regexp {
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
The backtracking algorithm reduces the problem to the call backtrack(root(P)), where backtrack is the following recursive procedure:

procedure backtrack(P, c) is
    if reject(P, c) then return
    if accept(P, c) then output(P, c)
    s ← first(P, c)
    while s ≠ NULL do
        backtrack(P, s)
        s ← next(P, s)
*/

enum PartialRegexpToken {
    Literal(char),
    Wildcard(char),
    Star(char, u32),
    WildcardStar(u32),
}
type RegexpPartialSolution = Vec<PartialRegexpToken>;
/// return the partial candidate at the root of the search tree.
fn root(inp: &str, regexp: &Regexp) -> PartialSolution {
    Vec::new()
}
/// return true only if the partial candidate c is not worth completing.
fn accept(inp: &str, regexp: &Regexp, partial: &PartialSolution) -> bool {
    for 
}
/// return true only if the partial candidate c is not worth completing.
fn reject(inp: &str, regexp: &Regexp, partial: &PartialSolution) -> bool {}
/// generate the first extension of candidate c.
fn first(inp: &str, regexp: &Regexp, partial: &PartialSolution) -> &PartialSolution {}
/// generate the next alternative extension of a candidate, after the extension s.
fn next(inp: &str, regexp: &Regexp, partial: &PartialSolution) -> &PartialSolution {}

fn string_matches_regex(string: &str, regexp: &[RegexpToken]) -> bool {
    let stack: Vec<PartialSolution> = Vec::new();
}

fn backtrack(problem: P, candidate: C) {}

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
