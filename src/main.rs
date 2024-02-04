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

#[derive(Debug)]
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

#[derive(Clone, Debug)]
enum ParseNode {
    Literal(char),
    Wildcard(char),
    Star(char, u32),   // length
    WildcardStar(u32), // length
}

type PartialSolution = Vec<ParseNode>;
fn length(inp: &PartialSolution) -> u32 {
    todo!()
}
/// return the partial candidate at the root of the search tree.
/// for us, an empty list
fn root(inp: &str, regex: &Regex) -> PartialSolution {
    Vec::new()
}
/// return true if `partial` is a complete solution, and false otherwise.
fn accept(inp: &str, regex: &Regex, partial: &PartialSolution) -> bool {
    todo!()
}
/// return true only if the partial candidate c is not worth completing.
fn reject(inp: &str, regex: &Regex, partial: &PartialSolution) -> bool {
    todo!()
}
/// generate the first extension of candidate c.
fn first(inp: &str, regex: &Regex, partial: &PartialSolution) -> Option<PartialSolution> {
    todo!()
}
/// generate the next alternative extension of a candidate, after the extension s.
fn next(inp: &str, regex: &Regex, partial: &PartialSolution) -> Option<PartialSolution> {
    todo!()
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
    if reject(inp, regex, &current) {
        return None;
    }
    if accept(inp, regex, &current) {
        return Some(current);
    }
    let mut candidate: Option<PartialSolution> = first(inp, regex, &current);
    while let Some(c) = candidate {
        candidate = next(inp, regex, &c); // lazy fix, ought to go after return
        if let Some(x) = backtrack(inp, regex, c) {
            return Some(x);
        }
    }
    return None; // I guess?
}

struct Solution;
impl Solution {
    pub fn is_match(input: String, pattern: String) -> bool {
        let regex = parse_regex(&pattern);
        println!("regex = {:?}", &regex);
        // first node of the search graph
        let root = root(&input, &regex);
        println!("root = {:?}", &root);
        let result = backtrack(&input, &regex, root);
        println!("result = {:?}", &result);
        return result.is_some();
    }
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
