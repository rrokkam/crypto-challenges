use std::collections::HashMap;

pub struct Scorer {
    frequencies: HashMap<char, f64>,
}

impl Scorer {
    pub fn new(corpus: &str) -> Self {
        let frequencies = Self::build_frequencies(corpus);
        Scorer { frequencies }
    }

    fn build_frequencies(corpus: &str) -> HashMap<char, f64> {
        corpus
            .chars()
            .fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            })
            .into_iter()
            .map(|(c, count)| (c, count as f64 / corpus.len() as f64))
            .collect()
    }

    pub fn score(&self, text: &str) -> f64 {
        if text == "" {
            return 0.0;
        }

        text.chars()
            .map(|c| *self.frequencies.get(&c).unwrap_or(&0.0))
            .sum::<f64>()
            / text.chars().count() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let frequencies = Scorer::build_frequencies("");
        let scorer = Scorer { frequencies };

        assert_eq!(scorer.score("🦀 is a crab emoji"), 0.0);
    }

    #[test]
    fn nonempty() {
        let corpus = "doing cryptopals in rust";
        let scorer = Scorer::new(&corpus);

        assert_eq!(scorer.score(""), 0.0);
        assert_eq!(scorer.score("z"), 0.0);
        assert_eq!(scorer.score(" "), 3.0 / corpus.len() as f64);
        assert_eq!(scorer.score(" a"), 2.0 / corpus.len() as f64);
    }
}
