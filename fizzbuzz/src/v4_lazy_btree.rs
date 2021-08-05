use std::collections::BTreeMap;
use std::collections::btree_map::Entry::*;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Variants {
    Number(usize),
    Fizz,
    Buzz,
    FizzBuzz,
}

trait FizzBuzzable {
    fn fuzz(self) -> Variants;
}

impl FizzBuzzable for usize {
    fn fuzz(self) -> Variants {
        match (self % 3, self % 5) {
            (0, 0) => Variants::FizzBuzz,
            (_, 0) => Variants::Buzz,
            (0, _) => Variants::Fizz,
            _ => Variants::Number(self),
        }
    }
}

struct LazyFizzBuzz {
    inner: BTreeMap<usize, Variants>,
}

impl LazyFizzBuzz {
    fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    fn get(&mut self, i: usize) -> Variants {
        let entry = self.inner.entry(i);
        match entry {
            Vacant(v) => {
                // The .fuzz() is considered heavy in this context
                let val = i.fuzz();
                v.insert(val.clone());

                val
            },
            Occupied(o) => {
                o.get().clone()
            }
        }
    }

    fn iter(&mut self) -> LazyFizzBuzzIter {
        LazyFizzBuzzIter {
            curr: 0,
            upper: None,
            inner: self
        }
    }

    fn iter_cached(&self) -> std::collections::btree_map::Values<'_, usize, Variants> {
        self.inner.values()
    }
}

struct LazyFizzBuzzIter<'a> {
    curr: usize,
    upper: Option<usize>,
    inner: &'a mut LazyFizzBuzz,
}

impl LazyFizzBuzzIter<'_> {
    fn upto(self, size: usize) -> Self {
        Self {
            upper: Some(size),
            .. self
        }
    }
}

impl<'a> Iterator for LazyFizzBuzzIter<'a> {
    type Item = Variants;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(upper) = self.upper {
            if self.curr < upper {
                self.curr += 1;
                Some(self.inner.get(self.curr))
            } else {
                None
            }
        } else {
            self.curr += 1;
            Some(self.inner.get(self.curr))
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
    fn lazy_btree_get1() {
        assert_eq!(
                Variants::Number(1), LazyFizzBuzz::new().get(1)
        );
    }

    #[test]
    fn lazy_btree_get3() {
        assert_eq!(
            Variants::Fizz, LazyFizzBuzz::new().get(3)
        );
    }

    #[test]
    fn lazy_btree_get5() {
        assert_eq!(
            Variants::Buzz, LazyFizzBuzz::new().get(5)
        );
    }

    #[test]
    fn lazy_btree_get15() {
        assert_eq!(
            Variants::FizzBuzz, LazyFizzBuzz::new().get(15)
        );
    }

    #[test]
    fn lazy_btree_iter() {
        let mut lazybtree = LazyFizzBuzz::new();

        assert_eq!(
            vec![
                Variants::Number(1),
                Variants::Number(2),
                Variants::Fizz,
                Variants::Number(4),
                Variants::Buzz,
                Variants::Fizz,
                Variants::Number(7),
                Variants::Number(8),
                Variants::Fizz,
                Variants::Buzz,
                Variants::Number(11),
                Variants::Fizz,
                Variants::Number(13),
                Variants::Number(14),
                Variants::FizzBuzz
            ],
            lazybtree.iter().upto(15).collect::<Vec<_>>()
        );
    }

    #[test]
    fn lazy_btree_iter_cached1() {
        let mut lazybtree = LazyFizzBuzz::new();
        for i in 1..=15 {
            lazybtree.get(i);
        }

        assert_eq!(
            vec![
                &Variants::Number(1),
                &Variants::Number(2),
                &Variants::Fizz,
                &Variants::Number(4),
                &Variants::Buzz,
                &Variants::Fizz,
                &Variants::Number(7),
                &Variants::Number(8),
                &Variants::Fizz,
                &Variants::Buzz,
                &Variants::Number(11),
                &Variants::Fizz,
                &Variants::Number(13),
                &Variants::Number(14),
                &Variants::FizzBuzz
            ],
            lazybtree.iter_cached().collect::<Vec<_>>()
        );
    }

    fn lazy_btree_iter_cached2() {
        let mut lazybtree = LazyFizzBuzz::new();
        lazybtree.get(1);
        lazybtree.get(3);
        lazybtree.get(5);
        lazybtree.get(15);

        assert_eq!(
            vec![
                &Variants::Number(1),
                &Variants::Fizz,
                &Variants::Buzz,
                &Variants::FizzBuzz
            ],
            lazybtree.iter_cached().collect::<Vec<_>>()
        );
    }
}
