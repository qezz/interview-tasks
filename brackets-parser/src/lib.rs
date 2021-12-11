use nom::character::complete::char;
use nom::combinator::fail;
use nom::IResult;
use nom::{character::complete::anychar, combinator::opt};
use std::collections::HashMap;

/// Parse open bracket
///
/// Returns a tuple (open-bracket: char, closing-bracket: char)
pub fn open<'a>(
    input: &'a str,
    mapping: &'a HashMap<char, char>,
) -> IResult<&'a str, (char, char)> {
    let (input, ch) = anychar(input)?;

    if let Some(v) = mapping.get(&ch) {
        Ok((input, (ch, *v)))
    } else {
        fail(input)
    }
}

/// Parse bracket expression
pub fn exp<'a>(input: &'a str, mapping: &'a HashMap<char, char>) -> IResult<&'a str, bool> {
    let (input, (_opening, expect_close)) = open(input, mapping)?;
    let (input, _x) = opt(|input| exp(input, mapping))(input)?;
    let (input, _x) = char(expect_close)(input)?;

    Ok((input, true))
}

#[cfg(test)]
mod tests {
    use nom::multi::many1;

    use super::*;

    const OPENINGS: [char; 3] = ['(', '[', '{'];
    const CLOSINGS: [char; 3] = [')', ']', '}'];

    fn matches() -> HashMap<char, char> {
        let mut hm = HashMap::new();
        for i in 0..OPENINGS.len() {
            hm.insert(OPENINGS[i], CLOSINGS[i]);
        }
        hm
    }

    pub fn complete_expression(input: &str) -> bool {
        let m = matches();
        let res = many1(|input| exp(input, &m))(input);
        if let Ok((tail, res)) = res {
            println!("for {}: {:?}", input, res);
            tail.is_empty() && res.iter().all(|x| *x)
        } else {
            false
        }
    }

    macro_rules! validator_test {
        ($test_name:ident, $input:expr, $expected:expr) => {
            #[test]
            #[allow(clippy::bool_assert_comparison)]
            fn $test_name() {
                assert_eq!(complete_expression($input), $expected);
            }
        };
    }

    validator_test!(sample1, "()", true);
    validator_test!(sample2, "()()", true);
    validator_test!(sample3, "(())", true);
    validator_test!(sample4, "()[]{}", true);
    validator_test!(sample5, "([{}])", true);
    validator_test!(sample6, "())", false);
    validator_test!(sample7, "(){", false);
    validator_test!(sample8, "{[}]", false);
    validator_test!(sample9, "[[[[[[]]]", false);
    validator_test!(sample10, "[)]", false);
    validator_test!(sample11, "[(]", false);
    validator_test!(sample12, "[](", false);
    validator_test!(unknown_char, "w", false);
}
