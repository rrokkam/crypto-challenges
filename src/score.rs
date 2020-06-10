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
        let mut counts = HashMap::new();
        for character in corpus.chars() {
            *counts.entry(character).or_insert(0) += 1;
        }
        counts
    }

    pub fn score(&self, text: &str) -> f64 {
        match text.chars().count() {
            0 => 0.0,
            len => self.total_count_in(text) as f64 / len as f64,
        }
    }

    fn total_count_in(&self, text: &str) -> usize {
        text.chars().fold(0, |acc, c| acc + self.count_of(c))
    }

    fn count_of(&self, c: char) -> usize {
        *self.counts.get(&c).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_corpus() {
        let scorer = Scorer::new("");
        assert_eq!(scorer.score("🦀 is a crab emoji"), 0.0);
    }

    #[test]
    fn empty_text() {
        let scorer = Scorer::new("doing cryptopals in rust");
        assert_eq!(scorer.score(""), 0.0);
    }

    #[test]
    fn one_char_not_in_corpus() {
        let scorer = Scorer::new("doing cryptopals in rust");
        assert_eq!(scorer.score("z"), 0.0);
    }

    #[test]
    fn one_char_in_corpus() {
        let scorer = Scorer::new("doing cryptopals in rust");
        assert_eq!(scorer.score(" "), 3.0);
        assert_eq!(scorer.score("a"), 1.0);
    }

    #[test]
    fn multiple_chars_in_corpus() {
        let scorer = Scorer::new("doing cryptopals in rust");
        assert_eq!(scorer.score("a z "), (1.0 + 3.0 + 0.0 + 3.0) / 4.0);
    }
}
