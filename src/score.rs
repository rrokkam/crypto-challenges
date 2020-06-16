use ordered_float::NotNan;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct Score(NotNan<f64>);

impl Score {
    fn new(score: f64) -> Self {
        Score(NotNan::new(score).unwrap())
    }
}

pub struct Scorer {
    counts: HashMap<u8, usize>,
}

impl Scorer {
    pub fn new(corpus: &[u8]) -> Self {
        let counts = Self::build_counts(corpus);
        Scorer { counts }
    }

    fn build_counts(corpus: &[u8]) -> HashMap<u8, usize> {
        let mut counts = HashMap::new();
        for byte in corpus {
            *counts.entry(*byte).or_insert(0) += 1;
        }
        counts
    }

    pub fn score(&self, text: &[u8]) -> Score {
        match text.len() {
            0 => Score::new(0.0),
            len => Score::new(self.total_count_in(text) as f64 / len as f64),
        }
    }

    fn total_count_in(&self, text: &[u8]) -> usize {
        text.iter().fold(0, |acc, &c| acc + self.count_of(c))
    }

    fn count_of(&self, c: u8) -> usize {
        *self.counts.get(&c).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_corpus() {
        let scorer = Scorer::new(b"");
        assert_eq!(scorer.score(b"A crab emoji"), Score::new(0.0));
    }

    #[test]
    fn empty_text() {
        let scorer = Scorer::new(b"doing cryptopals in rust");
        assert_eq!(scorer.score(b""), Score::new(0.0));
    }

    #[test]
    fn one_char_not_in_corpus() {
        let scorer = Scorer::new(b"doing cryptopals in rust");
        assert_eq!(scorer.score(b"z"), Score::new(0.0));
    }

    #[test]
    fn one_char_in_corpus() {
        let scorer = Scorer::new(b"doing cryptopals in rust");
        assert_eq!(scorer.score(b" "), Score::new(3.0));
        assert_eq!(scorer.score(b"a"), Score::new(1.0));
    }

    #[test]
    fn multiple_chars_in_corpus() {
        let scorer = Scorer::new(b"doing cryptopals in rust");
        assert_eq!(
            scorer.score(b"a z "),
            Score::new((1.0 + 3.0 + 0.0 + 3.0) / 4.0)
        );
    }
}
