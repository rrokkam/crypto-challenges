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
            len => self.total_count_of(text) as f64 / len as f64,
        }
    }

    fn total_count_of(&self, text: &str) -> usize {
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
    fn empty() {
        let scorer = Scorer::new("");

        assert_eq!(scorer.score("🦀 is a crab emoji"), 0.0);
    }

    #[test]
    fn nonempty() {
        let corpus = "doing cryptopals in rust";
        let scorer = Scorer::new(&corpus);

        assert_eq!(scorer.score(""), 0.0);
        assert_eq!(scorer.score("z"), 0.0);
        assert_eq!(scorer.score(" "), 3.0);
        assert_eq!(scorer.score(" a"), (3.0 + 1.0) / 2.0);
    }
}
