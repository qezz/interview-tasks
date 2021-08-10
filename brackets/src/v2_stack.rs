use std::collections::HashMap;

struct Validator<'a, 'b> {
    openings: &'a [char],
    closings: &'b [char],
    matches: HashMap<char, char>,
}

impl<'a, 'b> Validator<'a, 'b> {
    fn new(openings: &'a [char], closings: &'b [char]) -> Self {
        let mut hm = HashMap::new();
        for i in 0..openings.len() {
            hm.insert(closings[i], openings[i]);
        }

        Self {
            openings,
            closings,
            matches: hm,
        }
    }

    fn is_valid(&self, s: &str) -> bool {
        // s.len() / 2 is a safe bet. If sequence is valid, stack won't rise above this size
        let mut stack = Vec::with_capacity(s.len() / 2);

        for c in s.chars() {
            if self.closings.contains(&c) {
                let mut does_match = || {
                    let top = stack.pop()?;
                    let matching = self.matches.get(&c)?;
                    Some(*matching == top)
                };

                if !(does_match().unwrap_or(false)) {
                    return false;
                }
            } else if self.openings.contains(&c) {
                stack.push(c);
            } else {
                // Unknown character encountered
                return false;
            }
        }

        // Empty stack at the end is a sign that we have closed all the brackets
        stack.is_empty()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const OPENINGS: [char; 3] = ['(', '[', '{'];
    const CLOSINGS: [char; 3] = [')', ']', '}'];

    #[test]
    fn usage_example() {
        let input = "()";
        assert!(Validator::new(&OPENINGS, &CLOSINGS).is_valid(input));
    }

    macro_rules! validator_test {
        ($test_name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $test_name() {
                assert_eq!($expected, Validator::new(&OPENINGS, &CLOSINGS).is_valid($input));
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
    validator_test!(unknown_char, "w", false);

    #[test]
    fn rosetta_code() {
        let test_cases = vec![
            ("", true),
            ("[]", true),
            ("[]", true),
            ("[][]", true),
            ("[[][]]", true),
            ("][][", false),
            ("[]][[]", false),
        ];
        let validator = Validator::new(&OPENINGS, &CLOSINGS);

        for test_case in test_cases {
            assert_eq!(test_case.1, validator.is_valid(test_case.0));
        }
    }
}