use crate::score;
use std::collections::HashMap;

fn fixed_xor(first: &[u8], second: &[u8]) -> Vec<u8> {
    first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| a ^ b)
        .collect()
}

fn single_byte_xor(buffer: &[u8], c: u8) -> Vec<u8> {
    buffer.iter().map(|b| b ^ c).collect()
}

/// Will return None if none of the xors yielded a valid UTF8 encoding
fn decrypt_single_byte_xor(ciphertext: &[u8], freqs: &HashMap<char, f64>) -> Option<(f64, String)> {
    (u8::MIN..=u8::MAX)
        .map(|c| String::from_utf8(single_byte_xor(ciphertext, c)))
        .filter_map(Result::ok)
        .map(|text| (score::score(&text, freqs), text))
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
}

fn find_single_byte_xor(ciphertexts: Vec<&[u8]>, freqs: &HashMap<char, f64>) -> String {
    ciphertexts
        .iter()
        .map(|&ciphertext| decrypt_single_byte_xor(ciphertext, freqs))
        .filter_map(|x| x)
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        .unwrap() // assert that at least one decoding was valid. TODO: remove this and return Option<String> instead.
        .1
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;
    use hex_literal;
    use std::fs;

    const CORPUS_FILE_PATH: &str = "ulysses.txt";

    #[test]
    fn test_fixed_xor() {
        let first = hex_literal::hex!("1c0111001f010100061a024b53535009181c");
        let second = hex_literal::hex!("686974207468652062756c6c277320657965");
        let xored = fixed_xor(&first, &second);

        assert_eq!(
            xored,
            hex_literal::hex!("746865206b696420646f6e277420706c6179")
        );
    }

    #[test]
    fn test_decrypt_single_byte_xor() {
        let ciphertext = hex_literal::hex!(
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
        );
        let corpus = fs::read_to_string(CORPUS_FILE_PATH).unwrap();
        let freqs = score::frequencies(&corpus);

        let (_, plaintext_guess) = decrypt_single_byte_xor(&ciphertext, &freqs).unwrap();
        assert_eq!(plaintext_guess, "Cooking MC's like a pound of bacon");
    }

    #[test]
    #[ignore]
    fn test_find_single_byte_xor() {
        let ciphertexts = fs::read("single_byte_xored.txt").unwrap();

        let texts: Vec<Vec<u8>> = ciphertexts
            .split(|&c| c == b'\n')
            .map(|ciphertext| hex::decode(ciphertext).unwrap())
            .collect();
        let texts = texts.iter().map(|item| item.as_slice()).collect();

        let corpus = fs::read_to_string(CORPUS_FILE_PATH).unwrap();
        let freqs = score::frequencies(&corpus);

        let plaintext_guess = find_single_byte_xor(texts, &freqs);
        assert_eq!(plaintext_guess, "Now that the party is jumping");
    }
}
