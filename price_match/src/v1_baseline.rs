use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Outcome {
    Exact,
    MoreThanOthers,
    WrongResults,
}

fn validate(all_data: HashMap<String, usize>, predicted: String, baseline: usize) -> Outcome {
    let record = all_data.get(&predicted);

    // exact match
    if let Some(amount) = record {
        if *amount == baseline {
            return Outcome::Exact
        }
    }

    // no match, choose highest
    let max_amount = all_data.iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(_k, v)| v)
        .expect("not found");

    if let Some(amount) = record {
        if *amount == *max_amount {
            return Outcome::MoreThanOthers
        }
    }

    Outcome::WrongResults
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_exact() {
        let all_data: HashMap<String, usize> = vec![
            ("amazon".into(), 200),
            ("google".into(), 100)
        ].into_iter().collect() ;
        let predicted = "amazon";
        let baseline = 200;

        assert_eq!(
            validate(all_data, predicted.into(), baseline),
            Outcome::Exact
        );
    }

    #[test]
    fn test_exact_two_same() {
        let all_data: HashMap<String, usize> = vec![
            ("amazon".into(), 200),
            ("google".into(), 200)
        ].into_iter().collect() ;
        let predicted = "amazon";
        let baseline = 200;

        assert_eq!(
            validate(all_data, predicted.into(), baseline),
            Outcome::Exact
        );
    }

    #[test]
    fn test_more_than_others() {
        let all_data: HashMap<String, usize> = vec![
            ("amazon".into(), 200),
            ("google".into(), 100)
        ].into_iter().collect() ;
        let predicted = "amazon";
        let baseline = 300;

        assert_eq!(
            validate(all_data, predicted.into(), baseline),
            Outcome::MoreThanOthers
        );
    }

    #[test]
    fn test_wrong1() {
        let all_data: HashMap<String, usize> = vec![
            ("amazon".into(), 200),
            ("google".into(), 100)
        ].into_iter().collect() ;
        let predicted = "google";
        let baseline = 200;

        assert_eq!(
            validate(all_data, predicted.into(), baseline),
            Outcome::WrongResults
        );
    }

    #[test]
    fn test_wrong2() {
        let all_data: HashMap<String, usize> = vec![
            ("amazon".into(), 200), ("google".into(), 100)
        ].into_iter().collect() ;
        let predicted = "netflix";
        let baseline = 200;

        assert_eq!(
            validate(all_data, predicted.into(), baseline),
            Outcome::WrongResults
        );
    }

    #[test]
    fn test_wrong3() {
        let all_data: HashMap<String, usize> = vec![
            ("amazon".into(), 200),
            ("google".into(), 100)
        ].into_iter().collect() ;
        let predicted = "google";
        let baseline = 300;

        assert_eq!(
            validate(all_data, predicted.into(), baseline),
            Outcome::WrongResults
        );
    }
}