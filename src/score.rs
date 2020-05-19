use std::collections::HashMap;

pub fn frequencies(corpus: &str) -> HashMap<char, f64> {
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

pub fn score(text: &str, freqs: &HashMap<char, f64>) -> f64 {
    text.chars()
        .map(|c| *freqs.get(&c).unwrap_or(&0.0))
        .sum::<f64>()
        / text.chars().count() as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const CORPUS_FILE_PATH: &str = "ulysses.txt";

    #[test]
    fn frequencies_from_empty() {
        let corpus = "";
        let freqs = frequencies(&corpus);

        assert_eq!(freqs.len(), 0);
    }

    #[test]
    fn frequencies_from_small_corpus() {
        let corpus = "🦀doing cryptopals in rust🦀";
        let freqs = frequencies(&corpus);

        for (_, v) in freqs.iter() {
            assert!(v.is_normal());
            assert!(v.is_sign_positive())
        }

        assert_eq!(freqs.len(), 16);
        assert_eq!(*freqs.get(&' ').unwrap(), 3.0 / corpus.len() as f64);
        assert_eq!(*freqs.get(&'🦀').unwrap(), 2.0 / corpus.len() as f64);
        assert_eq!(freqs.get(&'z'), None);
    }

    #[test]
    fn freqencies_from_large_corpus() {
        let corpus = fs::read_to_string(CORPUS_FILE_PATH).expect("Corpus not found");
        let freqs = frequencies(&corpus);

        for (_, v) in freqs.iter() {
            assert!(v.is_normal());
            assert!(v.is_sign_positive())
        }

        let most_common_char = freqs
            .iter()
            .max_by(|a, b| (a.1).partial_cmp(&b.1).unwrap())
            .unwrap()
            .0;

        assert_eq!(most_common_char, &' ');
    }
}
