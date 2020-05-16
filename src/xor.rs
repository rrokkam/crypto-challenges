use std::collections::HashMap;

fn fixed_xor(first: Vec<u8>, second: Vec<u8>) -> Vec<u8> {
    first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| a ^ b)
        .collect()
}

fn char_frequencies(corpus: String) -> HashMap<char, f64> {
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

fn score(text: String, freqs: HashMap<char, f64>) -> f64 {
    let mut score = 0.0;
    for c in text.chars() {
        score += freqs.get(&c).unwrap_or(&0.0);
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn test_fixed_xor() {
        let first = hex!("1c0111001f010100061a024b53535009181c").to_vec();
        let second = hex!("686974207468652062756c6c277320657965").to_vec();
        let xored = fixed_xor(first, second);
        assert_eq!(xored, hex!("746865206b696420646f6e277420706c6179").to_vec());
    }
}
