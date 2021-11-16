#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum FizzBuzzItem {
    Number(usize),
    Fizz,
    Buzz,
    FizzBuzz,
}

pub(crate) trait FizzBuzzable {
    fn fuzz(self) -> FizzBuzzItem;
}

impl FizzBuzzable for usize {
    fn fuzz(self) -> FizzBuzzItem {
        match (self % 3, self % 5) {
            (0, 0) => FizzBuzzItem::FizzBuzz,
            (_, 0) => FizzBuzzItem::Buzz,
            (0, _) => FizzBuzzItem::Fizz,
            _ => FizzBuzzItem::Number(self),
        }
    }
}

struct FizzBuzz {
    curr: usize,
    upper: Option<usize>,
}

impl FizzBuzz {
    fn upto(self, size: usize) -> Self {
        Self {
            upper: Some(size),
            .. self
        }
    }

    fn start(self, lower: usize) -> Self {
        Self {
            curr: lower - 1,
            .. self
        }
    }
}

impl Default for FizzBuzz {
    fn default() -> FizzBuzz {
        FizzBuzz {
            curr: 0,
            upper: None,
        }
    }
}

impl Iterator for FizzBuzz {
    type Item = FizzBuzzItem;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(upper) = self.upper {
            if self.curr < upper {
                self.curr += 1;
                Some(self.curr.fuzz())
            } else {
                None
            }
        } else {
            self.curr += 1;
            Some(self.curr.fuzz())
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.upper {
            Some(upper) => {
                (upper, self.upper)
            },
            None => {
                (0, None)
            },

        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizzbuzz_iter_test1() {
        assert_eq!(
            vec![
                FizzBuzzItem::Number(1),
                FizzBuzzItem::Number(2),
                FizzBuzzItem::Fizz,
                FizzBuzzItem::Number(4),
                FizzBuzzItem::Buzz,
                FizzBuzzItem::Fizz,
                FizzBuzzItem::Number(7),
                FizzBuzzItem::Number(8),
                FizzBuzzItem::Fizz,
                FizzBuzzItem::Buzz,
                FizzBuzzItem::Number(11),
                FizzBuzzItem::Fizz,
                FizzBuzzItem::Number(13),
                FizzBuzzItem::Number(14),
                FizzBuzzItem::FizzBuzz
            ],
            FizzBuzz::default().upto(15).collect::<Vec<_>>()
        );
    }

    #[test]
    fn fizzbuzz_iter_test2() {
        assert_eq!(
            vec![
                FizzBuzzItem::Buzz,
                FizzBuzzItem::Fizz,
                FizzBuzzItem::Number(7),
                FizzBuzzItem::Number(8),
                FizzBuzzItem::Fizz,
                FizzBuzzItem::Buzz,
                FizzBuzzItem::Number(11),
                FizzBuzzItem::Fizz,
                FizzBuzzItem::Number(13),
                FizzBuzzItem::Number(14),
                FizzBuzzItem::FizzBuzz
            ],
            FizzBuzz::default().start(5).upto(15).collect::<Vec<_>>()
        );
    }

    #[test]
    fn fizzbuzz_size_hint1() {
        assert_eq!(FizzBuzz::default().upto(15).size_hint(), (15, Some(15)));
    }

    #[test]
    fn fizzbuzz_size_hint2() {
        assert_eq!(FizzBuzz::default().size_hint(), (0, None));
    }
}
