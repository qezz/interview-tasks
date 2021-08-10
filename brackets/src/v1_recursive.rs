use std::collections::HashMap;

struct Config<'a, 'b> {
    openings: &'a [&'a str],
    closings: &'b [&'b str],
    matches: HashMap<&'b str, &'a str>
}

fn reduce_internal<'a>(config: &Config, head: &'a str, s: &'a str) -> Option<&'a str> {
    if s.is_empty() {
        return None;
    }

    let (first, _tail) = s.split_at(1);

    if config.closings.contains(&first) {
        if let Some(x) = config.matches.get(first) {
            if x == &head {
                return Some(_tail);
            }
        }
    }

    if config.openings.contains(&first) {
        let res = reduce_internal(config, first, _tail);
        if let Some(t) = res {
            return reduce_internal(config, head, t);
        }
    }

    None
}

fn reduce<'a>(config: &Config, s: &'a str) -> Option<&'a str> {
    if s.is_empty() {
        return Some("");
    }

    let (head, tail) = s.split_at(1);
    let res = reduce_internal(config, head, tail);

    if let Some(tail) = res {
        return reduce(config, tail)
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const OPENINGS: [&str; 3] = ["(", "[", "{"];
    const CLOSINGS: [&str; 3] = [")", "]", "}"];

    fn config<'a, 'b>(openings: &'a [&str], closings: &'b [&str]) -> Config<'a, 'b> {
        let mut hm = HashMap::new();
        for i in 0..openings.len() {
            hm.insert(closings[i], openings[i]);
        }

        Config {
            openings,
            closings,
            matches: hm,
        }
    }

    #[test]
    fn usage_example() {
        let input = "()";
        let config = config(&OPENINGS, &CLOSINGS);
        assert_eq!(Some(""), reduce(&config, input));
    }

    macro_rules! reduce_test {
        ($test_name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $test_name() {
                let config = config(&OPENINGS, &CLOSINGS);
                let result = match reduce(&config, $input) {
                    Some(tail) if tail.is_empty() => {
                        true
                    },
                    _ => {
                        false
                    }
                };
                assert_eq!($expected, result);
            }
        };
    }

    reduce_test!(sample1, "()", true);
    reduce_test!(sample2, "()()", true);
    reduce_test!(sample3, "(())", true);
    reduce_test!(sample4, "()[]{}", true);
    reduce_test!(sample5, "([{}])", true);
    reduce_test!(sample6, "())", false);
    reduce_test!(sample7, "(){", false);
    reduce_test!(sample8, "{[}]", false);
    reduce_test!(sample9, "[[[[[[]]]", false);
    reduce_test!(sample10, "[)]", false);
    reduce_test!(sample11, "[(]", false);
    reduce_test!(unknown_char, "w", false);

    #[test]
    fn rosetta_code() {
        let test_cases = vec![
            ("", Some("")),
            ("[]", Some("")),
            ("[]", Some("")),
            ("[][]", Some("")),
            ("[[][]]", Some("")),
            ("][][", None),
            ("[]][[]", None),
        ];
        let config = config(&OPENINGS, &CLOSINGS);

        for test_case in test_cases {
            assert_eq!(test_case.1, reduce(&config, test_case.0));
        }
    }
}