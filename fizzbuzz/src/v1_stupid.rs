/// Simplest implementation of FizzBuzz
fn fizzbuzz(n: usize) -> Vec<String> {
    let mut res = vec![];

    for i in 1..=n {
        match (i % 3, i % 5) {
            (0, 0) => {
                res.push("FizzBuzz".into());
            }
            (_, 0) => {
                res.push("Buzz".into());
            }
            (0, _) => {
                res.push("Fizz".into());
            }
            _ => {
                res.push(format!("{}", i));
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizzbuzz_test1() {
        assert_eq!(
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ],
            fizzbuzz(15)
        );
    }
}
