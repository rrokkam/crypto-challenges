use std::collections::HashMap;

pub struct Scorer {
    frequencies: HashMap<char, f64>,
}

impl Scorer {
    pub fn new(corpus: &str) -> Self {
        let frequencies = corpus
            .chars()
            .fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            })
            .into_iter()
            .map(|(c, count)| (c, count as f64 / corpus.len() as f64))
            .collect();

        Scorer { frequencies }
    }

    pub fn score(&self, text: &str) -> f64 {
        let text_len = text.chars().count();
        if text_len == 0 {
            return 0.0;
        }

        text.chars()
            .map(|c| *self.frequencies.get(&c).unwrap_or(&0.0))
            .sum::<f64>()
            / text_len as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_corpus() {
        let corpus = "doing cryptopals in rust";
        let freqs = Scorer::new(&corpus);

        assert_eq!(freqs.score(" "), 3.0 / corpus.len() as f64);
        assert_eq!(freqs.score("z"), 0.0);
    }

    #[test]
    fn empty_text() {
        let freqs = Scorer::new("doing cryptopals in rust");
        assert_eq!(freqs.score(""), 0.0);
    }

    #[test]
    fn empty_corpus() {
        let freqs = Scorer::new("");
        let text = "🦀 is a crab emoji";

        assert_eq!(freqs.score(text), 0.0);
    }
}
