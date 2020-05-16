use std::collections::HashMap;

fn fixed_xor(first: Vec<u8>, second: Vec<u8>) -> Vec<u8> {
    first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| a ^ b)
        .collect()
}

fn char_frequencies(corpus: &str) -> HashMap<char, f64> {
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

fn score(text: &str, freqs: &HashMap<char, f64>) -> f64 {
    text.chars()
        .map(|c| freqs.get(&c).unwrap_or(&0.0))
        .sum::<f64>()
        / text.chars().count() as f64
}

fn single_byte_xor(buffer: Vec<u8>, c: u8) -> Vec<u8> {
    buffer.iter().map(|b| b ^ c).collect()
}

fn decrypt_single_byte_xor(ciphertext: Vec<u8>, corpus: &str) -> String {
    let freqs = char_frequencies(corpus);

    let mut max = 0.0;
    let mut best = String::new();
    for c in u8::MIN..=u8::MAX {
        let xored = single_byte_xor(ciphertext.clone(), c);
        if let Ok(utf8) = String::from_utf8(xored) {
            let score = score(&utf8, &freqs);
            if score > max {
                max = score;
                best = utf8;
            }
        }
    }

    best
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
