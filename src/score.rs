use std::collections::HashMap;

pub struct Scorer {
    counts: HashMap<char, usize>,
}

impl Scorer {
    pub fn new(corpus: &str) -> Self {
        let counts = Self::build_counts(corpus);
        Scorer { counts }
    }

    fn build_counts(corpus: &str) -> HashMap<char, usize> {
        corpus.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        })
    }

    pub fn score(&self, text: &str) -> f64 {
        if text == "" {
            return 0.0;
        }

        text.chars().map(|c| self.count(c)).sum::<usize>() as f64 / text.chars().count() as f64
    }

    fn count(&self, c: char) -> usize {
        *self.counts.get(&c).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let counts = Scorer::build_counts("");
        let scorer = Scorer { counts };

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
