use std::collections::HashMap;

pub fn char_frequencies(corpus: &str) -> HashMap<char, f64> {
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

pub fn score_text(text: &str, freqs: &HashMap<char, f64>) -> f64 {
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
    fn char_frequencies_with_empty_corpus() {
        let corpus = "";
        let freqs = char_frequencies(&corpus);
    }


    #[test]
    fn char_freqencies() {
        let corpus = fs::read_to_string(CORPUS_FILE_PATH).expect("Corpus not found");
        let freqs = char_frequencies(&corpus);
        let mut vec = freqs.iter().collect::<Vec<_>>();
        vec.sort_by(|a, b| (a.1).partial_cmp(b.1).unwrap());
        println!("{:#?}", vec);
    }
}
